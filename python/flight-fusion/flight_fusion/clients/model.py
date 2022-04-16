from __future__ import annotations

import pyarrow as pa
import requests
from mlserver.types import InferenceResponse, MetadataModelResponse

DATATYPE_MAP = {
    "FP64": pa.float64(),
    "FP32": pa.float32(),
    "INT64": pa.int64(),
}

FIELD_MAP = {
    pa.float64(): "FP64",
    pa.float32(): "FP32",
    pa.int64(): "INT64",
}


class ModelClient:
    def __init__(self, name: str, version: str | None = None) -> None:
        self._name = name
        self._version = version
        self._base_url = "http://localhost:8080"
        self._metadata = None
        self._input_schema = None
        self._output_schema = None

    def _inference_url(self):
        if self._version:
            return f"{self._base_url}/v2/models/{self._name}/versions/{self._version}/infer"
        return f"{self._base_url}/v2/models/{self._name}/infer"

    @property
    def input_schema(self):
        if self._input_schema is None:
            self._input_schema = pa.schema(
                pa.field(f.name, DATATYPE_MAP[f.datatype]) for f in self.get_metadata().inputs or []
            )
        return self._input_schema

    @property
    def output_schema(self):
        if self._output_schema is None:
            self._output_schema = pa.schema(
                pa.field(f.name, DATATYPE_MAP[f.datatype])
                for f in self.get_metadata().outputs or []
            )
        return self._output_schema

    def get_metadata(self) -> MetadataModelResponse:
        if self._metadata is None:
            endpoint = f"{self._base_url}/v2/models/{self._name}"
            response = requests.get(endpoint)
            self._metadata = MetadataModelResponse(**response.json())
        return self._metadata

    def predict(self, table: pa.Table) -> pa.Table:
        inputs = []

        for field in table.schema:
            data = table.column(field.name).to_pylist()
            input_tensor = {
                "name": field.name,
                "datatype": FIELD_MAP[field.type],
                "shape": [len(data)],
                "data": data,
            }
            inputs.append(input_tensor)

        inference_request = {"inputs": inputs}
        response = requests.post(self._inference_url(), json=inference_request)
        response_data = InferenceResponse(**response.json())

        fields = []
        arrays = []

        for out in response_data.outputs:
            fields.append(pa.field(out.name, DATATYPE_MAP[out.datatype]))
            arrays.append(pa.array(out.data))

        return pa.Table.from_arrays(arrays, schema=pa.schema(fields))
