from typing import Generic, List, TypedDict, TypeVar

from dagster import InitResourceContext, InputContext, OutputContext

CT = TypeVar("CT")
RT = TypeVar("RT")


class AreaConfig(TypedDict):
    name: str
    areas: List[str]


class TableReference(TypedDict, total=False):
    key: str
    source: AreaConfig


class TypedInitResourceContext(Generic[CT], InitResourceContext):
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


class TypedOutputContext(OutputContext, Generic[CT, RT]):
    """Auxiliary class for better type support with dagster configurations

    Args:
        OutputContext ([type]): the dagster OutputContext to be typed
        Generic ([type]): Type returned by `resource_config` and `resources` field - usually a TypedDict
    """

    @property
    def config(self) -> CT:
        ...

    @property
    def resources(self) -> RT:
        ...
