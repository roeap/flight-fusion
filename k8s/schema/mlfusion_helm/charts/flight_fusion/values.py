from typing import List

from mlfusion_helm.charts.global_ import Global
from mlfusion_helm.utils import ExtraManifestsType, k8s
from pydantic import BaseModel, Field


class FlightFusionHelmValues(BaseModel):
    __doc__ = "@" + "generated"

    imagePullSecrets: List[k8s.SecretRef]
    global_: Global = Field(..., alias="global")
    extraManifests: ExtraManifestsType
