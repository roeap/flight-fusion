from __future__ import annotations

from typing import Mapping, NamedTuple, Protocol, Sequence, Union

try:  # noqa: C901
    from dagster import AssetKey  # type: ignore
except ImportError:
    import json
    import re

    ASSET_KEY_REGEX = re.compile("^[a-zA-Z0-9_.-]+$")  # alphanumeric, _, -, .
    ASSET_KEY_SPLIT_REGEX = re.compile("[^a-zA-Z0-9_]")
    ASSET_KEY_DELIMITER = "/"
    ASSET_KEY_LEGACY_DELIMITER = "."

    def parse_asset_key_string(s: str) -> list[str]:
        return list(filter(lambda x: x, re.split(ASSET_KEY_SPLIT_REGEX, s)))

    class AssetKey(NamedTuple("_AssetKey", [("path", list[str])])):
        """Object representing the structure of an asset key.  Takes in a sanitized string, list of
        strings, or tuple of strings.

        Args:
            path (Sequence[str]): String, list of strings, or tuple of strings.  A list of strings
                represent the hierarchical structure of the asset_key.
        """

        def __new__(cls, path: Sequence[str]):
            if isinstance(path, str):
                path = [path]

            return super().__new__(cls, path=path)  # type: ignore

        def __str__(self):
            return f"AssetKey({self.path})"

        def __repr__(self):
            return f"AssetKey({self.path})"

        def __hash__(self):
            return hash(tuple(self.path))

        def __eq__(self, other):
            if not isinstance(other, AssetKey):
                return False
            return self.to_string() == other.to_string()

        def to_string(self, legacy: bool | None = False) -> str | None:
            if not self.path:
                return None
            if legacy:
                return ASSET_KEY_LEGACY_DELIMITER.join(self.path)
            return json.dumps(self.path)

        def to_user_string(self) -> str:
            """
            E.g. "first_component>second_component"
            """
            return ASSET_KEY_DELIMITER.join(self.path)

        @staticmethod
        def from_user_string(asset_key_string: str) -> AssetKey:
            return AssetKey(asset_key_string.split(ASSET_KEY_DELIMITER))

        @staticmethod
        def from_db_string(asset_key_string: str | None) -> AssetKey | None:
            if not asset_key_string:
                return None
            if asset_key_string[0] == "[":
                # is a json string
                try:
                    path = json.loads(asset_key_string)
                except json.JSONDecodeError:
                    path = parse_asset_key_string(asset_key_string)
            else:
                path = parse_asset_key_string(asset_key_string)
            return AssetKey(path)

        @staticmethod
        def get_db_prefix(path: list[str], legacy: bool | None = False):
            if legacy:
                return ASSET_KEY_LEGACY_DELIMITER.join(path)
            return json.dumps(path)[:-2]  # strip trailing '"]' from json string

        @staticmethod
        def from_graphql_input(asset_key: Mapping[str, list[str]]) -> AssetKey | None:
            if asset_key and asset_key.get("path"):
                return AssetKey(asset_key["path"])
            return None

        @staticmethod
        def from_coerceable(arg: CoercibleToAssetKey) -> AssetKey:
            if isinstance(arg, AssetKey):
                return arg
            elif isinstance(arg, str):
                return AssetKey([arg])
            elif isinstance(arg, list):
                return AssetKey(arg)
            else:
                return AssetKey(arg)

    CoercibleToAssetKey = Union[AssetKey, str, Sequence[str]]
    CoercibleToAssetKeyPrefix = Union[str, Sequence[str]]


class IAssetKey(Protocol):
    path: list[str]

    def to_string(self, legacy: bool | None = False) -> str | None:
        ...

    def to_user_string(self) -> str:
        ...
