from typing import List

from mlfusion_helm.charts.global_ import Global
from mlfusion_helm.utils import k8s
from pydantic import BaseModel, Field

from . import subschema


class MlfusionHelmValues(BaseModel):
    __doc__ = "@" + "generated"

    # postgresql: subschema.PostgreSQL
    generatePostgresqlPasswordSecret: bool
    imagePullSecrets: List[k8s.SecretRef]
    serviceAccount: subschema.ServiceAccount
    global_: Global = Field(..., alias="global")
