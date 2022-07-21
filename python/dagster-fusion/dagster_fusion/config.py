from __future__ import annotations

from dagster import Array, Enum, Field, String

from flight_fusion.ipc.v1alpha1 import SaveMode

FIELD_COLUMN_SELECTION = Field(
    Array(String),
    is_required=False,
    description="Sub-selection of columns to load from dataset",
)

FIELD_SAVE_MODE = Field(
    Enum.from_python_enum(SaveMode),
    is_required=False,
    default_value="SAVE_MODE_APPEND",
    description="Specifies behavior when saving data into a table location",
)
