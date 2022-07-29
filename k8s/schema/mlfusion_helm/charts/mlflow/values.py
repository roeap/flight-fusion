from typing import List

from mlfusion_helm.charts.global_ import Global
from mlfusion_helm.utils import ExtraManifestsType, k8s
from pydantic import BaseModel, Extra, Field

from . import subschema


class MlflowHelmValues(BaseModel):
    __doc__ = "@" + "generated"

    # postgresql: subschema.PostgreSQL
    replicaCount: int
    image: k8s.Image
    nameOverride: str
    service: k8s.Service
    envConfigMaps: List[k8s.ConfigMapEnvSource]
    envSecrets: List[k8s.SecretEnvSource]
    generatePostgresqlPasswordSecret: bool
    imagePullSecrets: List[k8s.SecretRef]
    serviceAccount: subschema.ServiceAccount
    nodeSelector: k8s.NodeSelector
    affinity: k8s.Affinity
    tolerations: k8s.Tolerations
    podSecurityContext: k8s.PodSecurityContext
    securityContext: k8s.SecurityContext
    resources: k8s.Resources
    readinessProbe: k8s.ReadinessProbe
    livenessProbe: k8s.LivenessProbe
    startupProbe: k8s.StartupProbe
    annotations: k8s.Annotations
    global_: Global = Field(..., alias="global")
    extraManifests: ExtraManifestsType

    class Config:
        extra = Extra.forbid
