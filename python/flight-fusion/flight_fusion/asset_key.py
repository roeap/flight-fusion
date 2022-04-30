from __future__ import annotations

from typing import Mapping, NamedTuple, Optional, Sequence

try:  # noqa: C901
    from dagster import AssetKey
except ImportError:
    import json
    import re

    ASSET_KEY_REGEX = re.compile("^[a-zA-Z0-9_.-]+$")  # alphanumeric, _, -, .
    ASSET_KEY_SPLIT_REGEX = re.compile("[^a-zA-Z0-9_]")
    ASSET_KEY_STRUCTURED_DELIMITER = "."

    def parse_asset_key_string(s: str) -> list[str]:
        return list(filter(lambda x: x, re.split(ASSET_KEY_SPLIT_REGEX, s)))

    class AssetKey(NamedTuple("_AssetKey", [("path", list[str])])):
        """Object representing the structure of an asset key.  Takes in a sanitized string, list of
        strings, or tuple of strings.

        Example usage:

        .. code-block:: python

            from dagster import op

            @op
            def emit_metadata(context, df):
                yield AssetMaterialization(
                    asset_key=AssetKey('flat_asset_key'),
                    metadata={"text_metadata": "Text-based metadata for this event"},
                )

            @op
            def structured_asset_key(context, df):
                yield AssetMaterialization(
                    asset_key=AssetKey(['parent', 'child', 'grandchild']),
                    metadata={"text_metadata": "Text-based metadata for this event"},
                )

            @op
            def structured_asset_key_2(context, df):
                yield AssetMaterialization(
                    asset_key=AssetKey(('parent', 'child', 'grandchild')),
                    metadata={"text_metadata": "Text-based metadata for this event"},
                )

        Args:
            path (Sequence[str]): String, list of strings, or tuple of strings.  A list of strings
                represent the hierarchical structure of the asset_key.
        """

        def __new__(cls, path: Sequence[str]):
            if isinstance(path, str):
                path = [path]
            else:
                path = path

            return super(AssetKey, cls).__new__(cls, path=path)  # type: ignore

        def __str__(self):
            return "AssetKey({})".format(self.path)

        def __repr__(self):
            return "AssetKey({})".format(self.path)

        def __hash__(self):
            return hash(tuple(self.path))

        def __eq__(self, other):
            if not isinstance(other, AssetKey):
                return False
            return self.to_string() == other.to_string()

        def to_string(self, legacy: Optional[bool] = False) -> Optional[str]:
            if not self.path:
                return None
            if legacy:
                return ASSET_KEY_STRUCTURED_DELIMITER.join(self.path)
            return json.dumps(self.path)

        @staticmethod
        def from_db_string(asset_key_string: Optional[str]) -> Optional["AssetKey"]:
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
        def get_db_prefix(path: list[str], legacy: Optional[bool] = False):
            if legacy:
                return ASSET_KEY_STRUCTURED_DELIMITER.join(path)
            return json.dumps(path)[:-2]  # strip trailing '"]' from json string

        @staticmethod
        def from_graphql_input(asset_key: Mapping[str, list[str]]) -> Optional["AssetKey"]:
            if asset_key and asset_key.get("path"):
                return AssetKey(asset_key["path"])
            return None
