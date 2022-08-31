class Path:
    def __init__(self, raw: str | list[str]) -> None: ...
    def child(self, part: str) -> Path: ...

class ObjectStore:
    def __init__(self, root: str) -> None: ...
    def get(self, location: Path) -> bytes: ...
    def put(self, location: Path, bytes: bytes) -> None: ...
    def list(self, prefix: Path | None) -> list[ObjectMeta]: ...
    def head(self, location: Path) -> ObjectMeta: ...
    def list_with_delimiter(self, prefix: Path | None) -> ListResult: ...

class ObjectMeta:
    @property
    def size(self) -> int: ...
    @property
    def location(self) -> Path: ...
    @property
    def last_modified(self) -> int: ...

class ListResult:
    @property
    def common_prefixes(self) -> list[Path]: ...
    @property
    def objects(self) -> list[ObjectMeta]: ...
