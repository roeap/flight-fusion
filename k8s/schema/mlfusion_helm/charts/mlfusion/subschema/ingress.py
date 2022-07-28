from enum import Enum
from typing import List, Optional, Union

from mlfusion_helm.utils import k8s
from pydantic import BaseModel


class IngressPathType(str, Enum):
    EXACT = "Exact"
    PREFIX = "Prefix"
    IMPLEMENTATION_SPECIFIC = "ImplementationSpecific"


class IngressTLSConfiguration(BaseModel):
    enabled: bool
    secretName: str


# Enforce as HTTPIngressPath: see https://github.com/dagster-io/dagster/issues/3184
class IngressPath(BaseModel):
    path: str
    pathType: IngressPathType
    serviceName: str
    servicePort: Union[str, int]


class DagitIngressConfiguration(BaseModel):
    host: str
    path: str
    pathType: IngressPathType
    tls: IngressTLSConfiguration
    precedingPaths: List[IngressPath]
    succeedingPaths: List[IngressPath]


class FlowerIngressConfiguration(BaseModel):
    host: str
    path: str
    pathType: IngressPathType
    tls: IngressTLSConfiguration
    precedingPaths: List[IngressPath]
    succeedingPaths: List[IngressPath]


class Ingress(BaseModel):
    enabled: bool
    apiVersion: Optional[str]
    annotations: k8s.Annotations
    dagit: DagitIngressConfiguration
    readOnlyDagit: DagitIngressConfiguration
    flower: FlowerIngressConfiguration
