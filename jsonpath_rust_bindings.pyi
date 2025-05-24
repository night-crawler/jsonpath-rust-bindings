from typing import List, Dict, Optional


class JsonPathResult:
    @property
    def data(self) -> Optional[Dict]: ...

    @property
    def path(self) -> Optional[str]: ...


class Finder:
    def __init__(
            self,
            obj: List | Dict
    ) -> None: ...

    def find(self, query: str) -> List[JsonPathResult]: ...
