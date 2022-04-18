from mlserver.types import MetadataModelResponse, Parameters
from mlserver_mlflow.runtime import MLflowRuntime


class MLfusionRuntime(MLflowRuntime):
    async def metadata(self) -> MetadataModelResponse:
        model_metadata = MetadataModelResponse(
            name=self.name,
            platform=self._settings.platform,
            versions=self._settings.versions,
            inputs=self._settings.inputs,
            outputs=self._settings.outputs,
        )

        if self._settings.parameters:
            # NOTE in contrast to the original implementation, we add the `extra` field
            # to the parameters where Dagster / MLFlow run IDs might be available, so that
            # clients can query / reference them.
            if self._settings.parameters.extra:
                model_metadata.parameters = Parameters(
                    content_type=self._settings.parameters.content_type,
                    **self._settings.parameters.extra,
                )
            else:
                model_metadata.parameters = Parameters(
                    content_type=self._settings.parameters.content_type
                )

        return model_metadata
