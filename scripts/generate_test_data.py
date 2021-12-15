import os

import numpy as np
import pandas as pd
import pyarrow as pa
import pyarrow.parquet as pq
from pyarrow.fs import LocalFileSystem


def file_relative_path(dunderfile, relative_path):
    return os.path.join(os.path.dirname(dunderfile), relative_path)


SIZE = 100
fs = LocalFileSystem()

data = {
    "part": np.random.choice(["a", "b", "c"], SIZE),
    "value": np.random.rand(SIZE),
}

table = pa.Table.from_pandas(pd.DataFrame(data))

path = file_relative_path(__file__, "../.tmp")

pq.write_to_dataset(
    table=table,
    root_path=f"{path}/dataset",
    partition_cols=["part"],
)

fs.create_dir(f"{path}/file")
pq.write_table(table=table, where=f"{path}/file/table.parquet")
