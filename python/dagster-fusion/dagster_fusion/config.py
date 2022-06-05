from __future__ import annotations

from dagster import Array, AssetKey, Enum, Field, Selector, Shape, String
from flight_fusion.ipc.v1alpha1 import AreaSourceReference, SaveMode

from dagster_fusion.errors import MissingConfiguration

_KEY_SEPARATOR = "/"

FIELD_LOCATION = Field(
    Selector(
        {
            "key": Field(String, is_required=False),
            # TODO find a better name then `source`
            "source": Field(
                Shape(
                    fields={
                        "name": Field(
                            String,
                            is_required=True,
                            description="Table / location name where data is loaded from",
                        ),
                        "areas": Field(Array(String)),
                    }
                ),
                is_required=False,
            ),
        }
    ),
    is_required=False,
)

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


def area_source_to_asset_key(reference: AreaSourceReference) -> AssetKey:
    if not reference.location:
        raise MissingConfiguration("Either location 'source' or 'key' must be configured")
    return AssetKey(reference.location.areas + [reference.location.name])
