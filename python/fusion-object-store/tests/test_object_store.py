from __future__ import annotations

from pathlib import Path

import pytest

from object_store import ObjectStore
from object_store import Path as ObjectStorePath


@pytest.fixture
def object_store(datadir: Path) -> tuple[ObjectStore, Path]:
    return ObjectStore(str(datadir)), datadir


def test_put_get_delete_list(object_store: tuple[ObjectStore, Path]):
    store, _ = object_store

    files = store.list()
    assert len(files) == 0

    location = ObjectStorePath("test_dir/test_file.json")
    store.put("test_dir/test_file.json", b"arbitrary data")

    files = store.list()
    assert len(files) == 1
    assert files[0].location == location

    files = store.list(ObjectStorePath("/"))
    assert len(files) == 1
    assert files[0].location == location

    result = store.list_with_delimiter()
    assert len(result.objects) == 0
    assert len(result.common_prefixes) == 1
    assert result.common_prefixes[0] == ObjectStorePath("test_dir")

    result = store.list_with_delimiter(ObjectStorePath("/"))
    assert len(result.objects) == 0
    assert len(result.common_prefixes) == 1
    assert result.common_prefixes[0] == ObjectStorePath("test_dir")

    files = store.list(ObjectStorePath("test_dir"))
    assert len(files) == 1
    assert files[0].location == location

    files = store.list(ObjectStorePath("something"))
    assert len(files) == 0

    data = store.get(location)
    assert data == b"arbitrary data"

    head = store.head(location)
    assert head.location == location
