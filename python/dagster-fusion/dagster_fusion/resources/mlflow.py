"""
This module contains the mlflow resource provided by the MlFlow
class. This resource provides an easy way to configure mlflow for logging various
things from dagster runs.
"""
# this module is adopted from the original dagster-mlflow implementation to include some platform specific tracking logic
# https://github.com/dagster-io/dagster/tree/master/python_modules/libraries/dagster-mlflow
from __future__ import annotations

import atexit
import sys
from itertools import islice
from os import environ
from typing import Any, Iterator

import pandas as pd
from dagster import (
    Field,
    InitResourceContext,
    Noneable,
    Permissive,
    StringSource,
    resource,
)

import mlflow
from dagster_fusion.errors import MissingConfiguration
from dagster_fusion.resources.configuration import MlFusionConfiguration
from flight_fusion.asset_key import IAssetKey
from flight_fusion.tags import MlFusionTags
from mlflow.entities.model_registry import RegisteredModel
from mlflow.entities.run_status import RunStatus
from mlflow.exceptions import MlflowException

_CONFIG_SCHEMA = {
    "experiment_name": Field(StringSource, is_required=True, description="MlFlow experiment name."),
    "parent_run_id": Field(
        Noneable(str),
        default_value=None,
        is_required=False,
        description="Mlflow run ID of parent run if this is a nested run.",
    ),
    "env": Field(Permissive(), description="Environment variables for mlflow setup."),
    "env_to_tag": Field(
        Noneable(list),
        default_value=None,
        is_required=False,
        description="List of environment variables to log as tags in mlflow.",
    ),
    "extra_tags": Field(Permissive(), description="Any extra key-value tags to log to mlflow."),
}


class MlflowMeta(type):
    """Mlflow Metaclass to create methods that "inherit" all of Mlflow's
    methods. If the class has a method defined it is excluded from the
    attribute setting from mlflow.
    """

    def __new__(cls, name, bases, attrs):
        class_cls = super().__new__(cls, name, bases, attrs)
        for attr in (attr for attr in dir(mlflow) if attr not in dir(class_cls)):
            mlflow_attribute = getattr(mlflow, attr)
            if callable(mlflow_attribute):
                setattr(class_cls, attr, staticmethod(mlflow_attribute))
            else:
                setattr(class_cls, attr, mlflow_attribute)
        return class_cls


class MlFlow(metaclass=MlflowMeta):
    """Class for setting up an mlflow resource for dagster runs.
    This takes care of all the configuration required to use mlflow tracking and the complexities of
    mlflow tracking dagster parallel runs.
    """

    def __init__(self, context: InitResourceContext, tracking_uri: str | None = None):

        # Context associated attributes
        if context.log is None:
            raise MissingConfiguration("Missing logger on context")
        self.log = context.log
        if context.dagster_run is None:
            raise MissingConfiguration("Mlfow resource requires active run")

        self.run_name = context.dagster_run.pipeline_name
        self.dagster_run_id = context.run_id

        # resource config attributes
        self.tracking_uri = tracking_uri
        if self.tracking_uri:
            mlflow.set_tracking_uri(self.tracking_uri)

        resource_config = context.resource_config
        self.parent_run_id = resource_config.get("parent_run_id")
        self.experiment_name = resource_config["experiment_name"]
        self.env_tags_to_log = resource_config.get("env_to_tag") or []
        self.extra_tags = resource_config.get("extra_tags")

        # Update env variables if any are given
        self.env_vars = resource_config.get("env", {})
        if self.env_vars:
            environ.update(self.env_vars)

        # If the experiment exists then the set won't do anything
        mlflow.set_experiment(self.experiment_name)
        self.experiment = mlflow.get_experiment_by_name(self.experiment_name)

        # Get the client object
        self.tracking_client = mlflow.tracking.MlflowClient()

        # Set up the active run and tags
        self._setup()

    def _setup(self):
        """
        Sets the active run and tags. If an Mlflow run_id exists then the
        active run is set to it. This way a single Dagster run outputs data
        to the same Mlflow run, even when multiprocess executors are used.
        """
        # Get the run id
        run_id = self._get_current_run_id()  # pylint: disable=no-member
        self._set_active_run(run_id=run_id)
        self._set_all_tags()

        # HACK needed to stop mlflow from marking run as finished when
        # a process exits in parallel runs
        atexit.unregister(mlflow.end_run)

    def _get_current_run_id(self, experiment: Any | None = None, dagster_run_id: str | None = None):
        """Gets the run id of a specific dagster run and experiment id.
        If it doesn't exist then it returns a None.

        Args:
            experiment (optional): Mlflow experiment.
            When none is passed it fetches the experiment object set in
            the constructor.  Defaults to None.
            dagster_run_id (optional): The Dagster run id.
            When none is passed it fetches the dagster_run_id object set in
            the constructor.  Defaults to None.
        Returns:
            run_id (str or None): run_id if it is found else None
        """
        experiment = experiment or self.experiment
        dagster_run_id = dagster_run_id or self.dagster_run_id
        if experiment:
            # Check if a run with this dagster run id has already been started
            # in mlflow, will get an empty dataframe if not
            current_run_df = mlflow.search_runs(
                experiment_ids=[experiment.experiment_id],
                filter_string=f"tags.{MlFusionTags.dagster.RUN_ID}='{dagster_run_id}'",
            )
            if isinstance(current_run_df, pd.DataFrame):
                if not current_run_df.empty:
                    return current_run_df.run_id.values[0]  # pylint: disable=no-member
            else:
                if not len(current_run_df) == 0:
                    current_run_df[0].info.run_id

    def _set_active_run(self, run_id=None):
        """
        This method sets the active run to be that of the specified
        run_id. If None is passed then a new run is started. The new run also
        takes care of nested runs.

        Args:
            run_id (str, optional): Mlflow run_id. Defaults to None.
        """
        nested_run = False
        if self.parent_run_id is not None:
            self._start_run(run_id=self.parent_run_id, run_name=self.run_name)
            nested_run = True
        self._start_run(run_id=run_id, run_name=self.run_name, nested=nested_run)

    def _start_run(self, **kwargs):
        """
        Catches the Mlflow exception if a run is already active.
        """

        try:
            run = mlflow.start_run(**kwargs)
            self.log.info(
                f"Starting a new mlflow run with id {run.info.run_id} " f"in experiment {self.experiment_name}"
            )
        except Exception as ex:
            run = mlflow.active_run()
            if run is None:
                raise MlflowException("Failed to get a run instance")
            if "is already active" not in str(ex):
                raise (ex)
            self.log.info(f"Run with id {run.info.run_id} is already active.")

    def _set_all_tags(self):
        """Method collects dagster_run_id plus all env variables/tags that have been
            specified by the user in the _config_schema and logs them as tags in mlflow.

        Returns:
            tags [dict]: Dictionary of all the tags
        """
        tags = {tag: environ.get(tag) for tag in self.env_tags_to_log}
        tags[MlFusionTags.dagster.RUN_ID] = self.dagster_run_id
        if self.extra_tags:
            tags.update(self.extra_tags)

        mlflow.set_tags(tags)

    def cleanup_on_error(self):
        """Method ends mlflow run with correct exit status for failed runs. Note that
        this method does not work when a job running in dagit fails, it seems
        that in this case a different process runs the job and when it fails
        the stack trace is therefore not available. For this case we can use the
        cleanup_on_failure hook defined below.
        """
        any_error = sys.exc_info()

        if any_error[1]:
            if isinstance(any_error[1], KeyboardInterrupt):
                mlflow.end_run(status=RunStatus.to_string(RunStatus.KILLED))
            else:
                mlflow.end_run(status=RunStatus.to_string(RunStatus.FAILED))

    def get_or_create_registered_model(self, asset_key: IAssetKey) -> RegisteredModel:
        try:
            model = self.tracking_client.get_registered_model(name=asset_key.to_user_string())
        except MlflowException:
            model = self.search_registered_model_by_tag(asset_key=asset_key)
        return model or self.create_registered_model_for_asset(asset_key=asset_key)

    def create_registered_model_for_asset(
        self, asset_key: IAssetKey, tags: dict[str, str] | None = None
    ) -> RegisteredModel:
        model_tags = {MlFusionTags.ASSET_KEY: asset_key.to_string(), **(tags or {})}
        return self.tracking_client.create_registered_model(name=asset_key.to_user_string(), tags=model_tags)

    def search_registered_model_by_tag(self, asset_key: IAssetKey) -> RegisteredModel | None:
        tag_value = asset_key.to_string()

        def search(page_token=None):
            models = self.tracking_client.list_registered_models(page_token=page_token)
            model = next(filter(lambda x: x.tags.get(MlFusionTags.ASSET_KEY) == tag_value, models), None)  # type: ignore
            if not model and models.token:
                return search(page_token=models.token)
            return model

        return search()

    @staticmethod
    def log_params(params: dict):
        """Overload of the mlflow.log_params. If len(params) >100 then
        params is sent to mlflow in chunks.

        Args:
            params (dict): Parameters to be logged
        """
        for param_chunk in MlFlow.chunks(params, 100):
            mlflow.log_params(param_chunk)

    @staticmethod
    def chunks(params: dict, size: int = 100):
        """Method that chunks a dictionary into batches of size.

        Args:
            params (dict): Dictionary set to be batched
            size (int, optional): Number of batches. Defaults to 100.

        Yields:
            (dict): Batch of dictionary
        """
        it = iter(params)
        for _ in range(0, len(params), size):
            yield {k: params[k] for k in islice(it, size)}


@resource(config_schema=_CONFIG_SCHEMA, required_resource_keys={"mlfusion_config"})
def mlflow_tracking(context: InitResourceContext) -> Iterator[MlFlow]:
    """
    This resource initializes an MLflow run that's used for all steps within a Dagster run.

    This resource provides access to all of mlflow's methods as well as the mlflow tracking client's
    methods.

    Usage:

    1. Add the mlflow resource to any solids in which you want to invoke mlflow tracking APIs.
    2. Add the `end_mlflow_on_run_finished` hook to your pipeline to end the MLflow run
       when the Dagster run is finished.

    Examples:

        .. code-block:: python
            from dagster_fusion import end_mlflow_on_run_finished, mlflow_tracking

            @op(required_resource_keys={"mlflow"})
            def mlflow_op(context):
                mlflow.log_params(some_params)
                mlflow.tracking.MlflowClient().create_registered_model(some_model_name)

            @end_mlflow_on_run_finished
            @job(resource_defs={"mlflow": mlflow_tracking})
            def mlf_example():
                mlflow_op()

            # example using an mlflow instance with s3 storage
            mlf_example.execute_in_process(run_config={
                "resources": {
                    "mlflow": {
                        "config": {
                            "experiment_name": my_experiment,

                            # if want to run a nested run, provide parent_run_id
                            "parent_run_id": an_existing_mlflow_run_id,

                            # env variables to pass to mlflow
                            "env": {
                                "MLFLOW_S3_ENDPOINT_URL": my_s3_endpoint,
                                "AWS_ACCESS_KEY_ID": my_aws_key_id,
                                "AWS_SECRET_ACCESS_KEY": my_secret,
                            },

                            # env variables you want to log as mlflow tags
                            "env_to_tag": ["DOCKER_IMAGE_TAG"],

                            # key-value tags to add to your experiment
                            "extra_tags": {"super": "experiment"},
                        }
                    }
                }
            })
    """
    config: MlFusionConfiguration = context.resources.mlfusion_config  # type: ignore
    mlf = MlFlow(context, tracking_uri=config.mlflow_tracking_uri)
    yield mlf
    mlf.cleanup_on_error()
