from typing import Tuple

import pyarrow as pa
from dagster import Out, Output, op
from pandas import DataFrame
from pyspark.sql import DataFrame as SparkDF

HN_ACTION_SCHEMA = pa.schema(
    [
        pa.field("id", pa.int64()),
        pa.field("parent", pa.float64()),
        pa.field("time", pa.int64()),
        pa.field("type", pa.string()),
        pa.field("by", pa.string()),
        pa.field("text", pa.string()),
        pa.field("kids", pa.list_(pa.int64())),
        pa.field("score", pa.float64()),
        pa.field("title", pa.string()),
        pa.field("descendants", pa.float64()),
        pa.field("url", pa.string()),
    ]
)

ACTION_FIELD_NAMES = [field.name for field in HN_ACTION_SCHEMA]


@op(
    out={
        "items": Out(
            io_manager_key="parquet_io_manager",
            metadata={"partitioned": True},
            dagster_type=DataFrame,
        )
    },
    required_resource_keys={"hn_client"},
)
def download_items(context, id_range: Tuple[int, int]) -> Output:
    """
    Downloads all of the items for the id range passed in as input and creates a DataFrame with
    all the entries.
    """
    start_id, end_id = id_range

    context.log.info(f"Downloading range {start_id} up to {end_id}: {end_id - start_id} items.")

    rows = []
    for item_id in range(start_id, end_id):
        rows.append(context.resources.hn_client.fetch_item_by_id(item_id))
        if len(rows) % 100 == 0:
            context.log.info(f"Downloaded {len(rows)} items!")

    non_none_rows = [row for row in rows if row is not None]
    result = DataFrame(non_none_rows, columns=ACTION_FIELD_NAMES).drop_duplicates(subset=["id"])
    result.rename(columns={"by": "user_id"}, inplace=True)

    return Output(
        result,
        "items",
        metadata={"Non-empty items": len(non_none_rows), "Empty items": rows.count(None)},
    )


@op(
    out=Out(
        io_manager_key="warehouse_io_manager",
        metadata={"table": "hackernews.comments", "partitioned": True},
    ),
    description="Creates a dataset of all items that are comments",
)
def build_comments(context, items: SparkDF) -> SparkDF:
    context.log.info(str(items.schema))
    return items.where(items["type"] == "comment")


@op(
    out=Out(
        io_manager_key="warehouse_io_manager",
        metadata={"table": "hackernews.stories", "partitioned": True},
    ),
    description="Creates a dataset of all items that are stories",
)
def build_stories(context, items: SparkDF) -> SparkDF:
    context.log.info(str(items.schema))
    return items.where(items["type"] == "story")
