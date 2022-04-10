from dagster import Array, Enum, Field, Selector, Shape, String
from dagster_fusion._types import TableReference
from dagster_fusion.errors import MissingConfiguration
from flight_fusion.ipc.v1alpha1 import AreaSourceReference, AreaTableLocation, SaveMode

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
    is_required=True,
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


def table_reference_to_area_source(ref: TableReference) -> AreaSourceReference:
    key = ref.get("key")
    if key is not None:
        parts = key.split("/")
        areas = parts[:-1]
        name = parts[-1]

        return AreaSourceReference(location=AreaTableLocation(name=name, areas=areas))

    source = ref.get("source")
    if source is not None:
        return AreaSourceReference(
            location=AreaTableLocation(name=source["name"], areas=source["areas"])
        )

    raise MissingConfiguration("Either location 'source' or 'key' must be configured")
