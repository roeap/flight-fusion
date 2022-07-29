from mlfusion_helm.utils import k8s
from pydantic import BaseModel


class ServiceAccount(BaseModel):
    create: bool
    name: str
    annotations: k8s.Annotations
