from __future__ import annotations

from abc import abstractmethod

import pyarrow as pa
import requests
from mlserver.types import InferenceResponse as RestInferenceResponse
from mlserver.types import MetadataModelResponse as RestMetadataModelResponse

from flight_fusion.ipc.inference import InferTensorContents
from flight_fusion.ipc.inference import (
    ModelInferRequestInferInputTensor as InferInputTensor,
)
from flight_fusion.ipc.inference import ModelMetadataResponse

from .inference import GrpcInferenceServiceClient


class DataTypes:
    float64: str = "FP64"
    float32: str = "FP32"
    int64: str = "INT64"
    int32: str = "INT32"
    int16: str = "INT16"
    int8: str = "INT8"


DATATYPE_MAP = {
    DataTypes.float64: pa.float64(),
    DataTypes.float32: pa.float32(),
    DataTypes.int64: pa.int64(),
    DataTypes.int32: pa.int32(),
    DataTypes.int16: pa.int16(),
    DataTypes.int8: pa.int8(),
}

FIELD_MAP = {
    pa.float64(): DataTypes.float64,
    pa.float32(): DataTypes.float32,
    pa.int64(): DataTypes.int64,
    pa.int32(): DataTypes.int32,
    pa.int16(): DataTypes.int16,
    pa.int8(): DataTypes.int8,
}


class ModelClient:
    @property
    def input_schema(self) -> pa.Schema:
        if self._input_schema is None:
            self._input_schema = pa.schema(
                pa.field(f.name, DATATYPE_MAP[f.datatype]) for f in self.get_metadata().inputs or []
            )
        return self._input_schema

    @property
    def output_schema(self) -> pa.Schema:
        if self._output_schema is None:
            self._output_schema = pa.schema(
                pa.field(f.name, DATATYPE_MAP[f.datatype]) for f in self.get_metadata().outputs or []
            )
        return self._output_schema

    @abstractmethod
    def get_metadata(self) -> ModelMetadataResponse:
        raise NotImplementedError

    @abstractmethod
    def predict(self, table: pa.Table) -> pa.Table:
        raise NotImplementedError


class GrpcModelClient(ModelClient):
    def __init__(self, client: GrpcInferenceServiceClient, name: str, version: str | None = None) -> None:
        self._client = client
        self._name = name
        self._version = version
        self._metadata = None
        self._input_schema = None
        self._output_schema = None

    def get_metadata(self) -> ModelMetadataResponse:
        if self._metadata is None:
            self._client.model_metadata(name=self._name, version=self._version or "")
            self._metadata = self._client.model_metadata(name=self._name, version=self._version or "")
        return self._metadata

    def predict(self, table: pa.Table) -> pa.Table:
        inputs = []
        for field in table.schema:
            data = table.column(field.name).to_pylist()  # type: ignore

            type_key = FIELD_MAP[field.type]
            if type_key == DataTypes.float64:
                contents = InferTensorContents(fp64_contents=data)
            elif type_key == DataTypes.float32:
                contents = InferTensorContents(fp32_contents=data)
            elif type_key == DataTypes.int64:
                contents = InferTensorContents(int64_contents=data)
            elif type_key in [DataTypes.int32, DataTypes.int16, DataTypes.int8]:
                contents = InferTensorContents(int_contents=data)
            else:
                raise NotImplementedError(f"Datatype {type_key} not yet supported")

            input_tensor = InferInputTensor(
                name=field.name,
                datatype=FIELD_MAP[field.type],
                shape=[len(data)],
                contents=contents,
            )
            inputs.append(input_tensor)

        response = self._client.model_infer(model_name=self._name, model_version=self._version or "", inputs=inputs)

        fields = []
        arrays = []
        for out in response.outputs:
            fields.append(pa.field(out.name, DATATYPE_MAP[out.datatype]))
            if out.datatype == DataTypes.float64:
                arrays.append(pa.array(out.contents.fp64_contents))
            elif out.datatype == DataTypes.float32:
                arrays.append(pa.array(out.contents.fp32_contents))
            elif out.datatype == DataTypes.int64:
                arrays.append(pa.array(out.contents.int64_contents))
            elif out.datatype in [DataTypes.int32, DataTypes.int16, DataTypes.int8]:
                arrays.append(pa.array(out.contents.int_contents))
            else:
                raise NotImplementedError(f"Datatype {out.datatype} not yet supported")

        return pa.Table.from_arrays(arrays, schema=pa.schema(fields))


class RestModelClient(ModelClient):
    def __init__(
        self,
        base_url: str,
        name: str,
        version: str | None = None,
        session: requests.Session | None = None,
    ) -> None:
        self._name = name
        self._version = version
        self._base_url = base_url
        self._session = session or requests.Session()
        self._metadata = None
        self._input_schema = None
        self._output_schema = None

    def _inference_url(self) -> str:
        if self._version:
            return f"{self._base_url}/v2/models/{self._name}/versions/{self._version}/infer"
        return f"{self._base_url}/v2/models/{self._name}/infer"

    def load(self) -> None:
        pass

    def get_metadata(self) -> RestMetadataModelResponse:
        if self._metadata is None:
            endpoint = f"{self._base_url}/v2/models/{self._name}"
            response = self._session.get(endpoint)
            self._metadata = RestMetadataModelResponse(**response.json())
        return self._metadata

    def predict(self, table: pa.Table) -> pa.Table:
        inputs = []
        for field in table.schema:
            data = table.column(field.name).to_pylist()  # type: ignore
            input_tensor = {
                "name": field.name,
                "datatype": FIELD_MAP[field.type],
                "shape": [len(data)],
                "data": data,
            }
            inputs.append(input_tensor)

        response = self._session.post(self._inference_url(), json={"inputs": inputs})
        response_data = RestInferenceResponse(**response.json())

        fields = []
        arrays = []
        for out in response_data.outputs:
            fields.append(pa.field(out.name, DATATYPE_MAP[out.datatype]))
            arrays.append(pa.array(out.data))  # type: ignore

        return pa.Table.from_arrays(arrays, schema=pa.schema(fields))
