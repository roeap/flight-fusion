import os
from typing import List, Optional

from mlflow.tracking.context.abstract_context import RunContextProvider

MLFUSION_NAMESPACE = "mlfusion."
MLFUSION_RUNTIME = "mlfusion.runtime"
MLFUSION_ENV_TAGS = [
    ("MLFUSION_PROJECT_ID", "mlfusion.project.projectId"),
]


class FusionRunContextProvider(RunContextProvider):
    """RunContextProvider provided through plugin system"""

    def __init__(self, tracking_vars: Optional[List] = None) -> None:
        super().__init__()
        self.tracking_vars = tracking_vars
        self.environ = dict(os.environ)

    def in_context(self):
        return True

    def _check_tracking_vars(self):
        available_vars = [var in self.environ for var in self.tracking_vars or []]
        if not all(available_vars):
            raise ValueError("Not all tracking_vars are defined in environment.")

    def tags(self):
        tags = {MLFUSION_RUNTIME: "true"}

        for environment_variable, tag_name in MLFUSION_ENV_TAGS:
            value = os.environ.get(environment_variable)
            if value:
                tags[tag_name] = value

        return tags
