from __future__ import annotations

from enum import Enum

from pydantic import BaseModel, Field


class ModelVersionStatus(str, Enum):
    # Request to register a new model version is pending as server performs background tasks.
    PENDING_REGISTRATION = "PENDING_REGISTRATION"
    # Request to register a new model version has failed.
    FAILED_REGISTRATION = "FAILED_REGISTRATION"
    # Model version is ready for use
    READY = "READY"


class ModelVersionTag(BaseModel):
    key: str
    value: str


class RegisteredModelTag(BaseModel):
    key: str
    value: str


class ModelVersion(BaseModel):
    # Unique name of the model
    name: str
    # Model’s version number.
    version: str
    # Timestamp recorded when this model_version was created.
    creation_timestamp: int
    # Timestamp recorded when metadata for this model_version was last updated.
    last_updated_timestamp: int
    # User that created this model_version.
    user_id: str | None
    # Current stage for this model_version.
    current_stage: str
    # Description of this model_version.
    description: str | None
    # URI indicating the location of the source model artifacts, used when creating model_version
    source: str
    # MLflow run ID used when creating model_version, if source was generated
    # by an experiment run stored in MLflow tracking server.
    run_id: str
    # Current status of model_version
    status: ModelVersionStatus
    # Details on current status, if it is pending or failed.
    status_message: str | None
    # Additional metadata key-value pairs for this model_version.
    tags: list[ModelVersionTag] = Field(default_factory=list)
    # Run Link: Direct link to the run that generated this version
    run_link: str


class RegisteredModel(BaseModel):
    # Unique name for the model.
    name: str
    # Timestamp recorded when this registered_model was created.
    creation_timestamp: int
    # Timestamp recorded when metadata for this registered_model was last updated.
    last_updated_timestamp: int
    # User that created this registered_model
    user_id: str | None
    # Description of this registered_model.
    description: str | None
    # Collection of latest model versions for each stage. Only contains models with current READY status.
    latest_versions: list[ModelVersion]
    # Tags: Additional metadata key-value pairs for this registered_model.
    tags: list[RegisteredModelTag] = Field(default_factory=list)


class ListRegisteredModelResponse(BaseModel):
    registered_models: list[RegisteredModel]
