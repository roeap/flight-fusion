from typing import Generic, TypeVar

from dagster import InitResourceContext, InputContext

CT = TypeVar("CT")
RT = TypeVar("RT")


class TypedInitResourceContext(InitResourceContext, Generic[CT]):
    """Auxiliary class for better type support with dagster configurations

    Args:
        InitResourceContext ([type]): the dagster InitResourceContext to be typed
        Generic ([type]): Type returned by `resource_config` field - usually a TypedDict
    """

    @property
    def resource_config(self) -> CT:
        ...


class TypedInputContext(InputContext, Generic[CT, RT]):
    """Auxiliary class for better type support with dagster configurations

    Args:
        InputContext ([type]): the dagster InputContext to be typed
        Generic ([type]): Type returned by `resource_config` and `resources` field - usually a TypedDict
    """

    @property
    def config(self) -> CT:
        ...

    @property
    def resources(self) -> RT:
        ...
