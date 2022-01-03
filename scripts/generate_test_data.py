import os
from pathlib import Path
from subprocess import STDOUT, check_output

import numpy as np
import pandas as pd
import pyarrow as pa
import pyarrow.parquet as pq
from pyarrow.fs import LocalFileSystem


def file_relative_path(dunderfile, relative_path):
    return os.path.join(os.path.dirname(dunderfile), relative_path)


def workspace_root() -> Path:
    output = check_output(["cargo", "metadata"], stderr=STDOUT).decode()  # nosec
    key = 'workspace_root":"'
    idx = output.find(key)
    part = output[idx + len(key) :]
    idx = part.find('"')

    return Path(part[:idx])


SIZE = 100
fs = LocalFileSystem()
ws_root = workspace_root()

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

ts = pd.date_range(
    start=pd.to_datetime("2020-01-01 00:00:00"),
    end=pd.to_datetime("2020-01-02 00:00:00"),
    freq="h",
)
data1 = {
    "timestamp": ts,
    "S2": np.random.randn(len(ts)),
    "S3": np.random.randn(len(ts)),
    "S5": np.random.randn(len(ts)),
}

data2 = {
    "timestamp": ts,
    "S6": np.random.randn(len(ts)),
    "S7": np.random.randn(len(ts)),
}

path = ws_root / "test/data/"
path.mkdir(exist_ok=True, parents=True)

pd.DataFrame(data1).to_parquet(str(path / "P1.parquet"))
pd.DataFrame(data2).to_parquet(str(path / "P2.parquet"))
