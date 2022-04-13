import pyarrow as pa
from dagster import asset

from .id_range_for_time import id_range_for_time

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
    name="items",
    namespace=["demo", "hacker"],
    required_resource_keys={"hn_client", "partition_bounds"},
)
def download_items(context) -> pa.Table:
    """
    Downloads all of the items for the id range passed in as input and creates a DataFrame with
    all the entries.
    """
    # The lower (inclusive) and upper (exclusive) ids that bound the range for the partition
    (start_id, end_id), metadata_entries = id_range_for_time(
        context.resources.partition_bounds["start"],
        context.resources.partition_bounds["end"],
        context.resources.hn_client,
    )

    context.log.info(f"Downloading range {start_id} up to {end_id}: {end_id - start_id} items.")

    rows = []
    for item_id in range(start_id, end_id):
        rows.append(context.resources.hn_client.fetch_item_by_id(item_id))
        if len(rows) % 100 == 0:
            context.log.info(f"Downloaded {len(rows)} items!")

    non_none_rows = [row for row in rows if row is not None]
    table = pa.Table.from_pylist(non_none_rows, HN_ACTION_SCHEMA)

    return table
