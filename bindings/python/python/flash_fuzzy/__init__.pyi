"""Type stubs for flash_fuzzy."""

from typing import List, Dict, Any, Union, overload

class SearchResult:
    """A single search result."""

    @property
    def id(self) -> int:
        """Record ID."""
        ...

    @property
    def score(self) -> float:
        """Match score (0.0-1.0)."""
        ...

    @property
    def start(self) -> int:
        """Match start position."""
        ...

    @property
    def end(self) -> int:
        """Match end position."""
        ...

class FlashFuzzy:
    """High-performance fuzzy search engine."""

    def __init__(
        self,
        threshold: float = 0.25,
        max_errors: int = 2,
        max_results: int = 50,
    ) -> None:
        """
        Create a new FlashFuzzy instance.

        Args:
            threshold: Minimum score (0.0-1.0) for results. Default: 0.25
            max_errors: Maximum edit distance (0-3). Default: 2
            max_results: Maximum results to return. Default: 50
        """
        ...

    @overload
    def add(self, records: Dict[str, Any]) -> int: ...
    @overload
    def add(self, records: List[Dict[str, Any]]) -> int: ...

    def add(self, records: Union[Dict[str, Any], List[Dict[str, Any]]]) -> int:
        """
        Add a single record or list of records.

        Args:
            records: A dict or list of dicts with 'id' and text fields

        Returns:
            Number of records added
        """
        ...

    def search(self, query: str) -> List[SearchResult]:
        """
        Search for matching records.

        Args:
            query: The search query string

        Returns:
            List of SearchResult objects sorted by score (descending)
        """
        ...

    def remove(self, id: int) -> bool:
        """
        Remove a record by ID.

        Args:
            id: The record ID to remove

        Returns:
            True if found and removed, False otherwise
        """
        ...

    def reset(self) -> None:
        """Clear all records."""
        ...

    @property
    def count(self) -> int:
        """Number of records in the index."""
        ...

    @property
    def threshold(self) -> float:
        """Minimum score threshold."""
        ...

    @threshold.setter
    def threshold(self, value: float) -> None: ...

    @property
    def max_errors(self) -> int:
        """Maximum edit distance."""
        ...

    @max_errors.setter
    def max_errors(self, value: int) -> None: ...

    @property
    def max_results(self) -> int:
        """Maximum results to return."""
        ...

    @max_results.setter
    def max_results(self, value: int) -> None: ...

__all__ = ["FlashFuzzy", "SearchResult"]
