from typing import Tuple

import polars as pl
import pyarrow as pa
from dagster import Output, asset
from hacker_news.partitions import hourly_partitions

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


@asset(
    namespace=["demo", "hacker"],
    io_manager_key="fusion_io_manager",
    required_resource_keys={"hn_client"},
    partitions_def=hourly_partitions,
)
def items(context, id_range_for_time: Tuple[int, int]):
    """Items from the Hacker News API: each is a story or a comment on a story."""
    start_id, end_id = id_range_for_time

    context.log.info(f"Downloading range {start_id} up to {end_id}: {end_id - start_id} items.")

    rows = []
    for item_id in range(start_id, end_id):
        rows.append(context.resources.hn_client.fetch_item_by_id(item_id))
        if len(rows) % 100 == 0:
            context.log.info(f"Downloaded {len(rows)} items!")

    non_none_rows = [row for row in rows if row is not None]
    table = pa.Table.from_pylist(non_none_rows, HN_ACTION_SCHEMA)
    # TODO rename id to user_id

    return Output(
        table,
        metadata={"Non-empty items": len(non_none_rows), "Empty items": rows.count(None)},
    )


@asset(
    namespace=["demo", "hacker"],
    io_manager_key="fusion_io_manager",
    partitions_def=hourly_partitions,
)
def comments(items: pl.DataFrame) -> pa.Table:
    return items.filter(pl.col("type") == "comment").to_arrow()


@asset(
    namespace=["demo", "hacker"],
    io_manager_key="fusion_io_manager",
    partitions_def=hourly_partitions,
)
def stories(items: pl.DataFrame) -> pa.Table:
    return items.filter(pl.col("type") == "story").to_arrow()
