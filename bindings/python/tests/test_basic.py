"""Basic tests for flash-fuzzy Python bindings."""

import pytest
from flash_fuzzy import FlashFuzzy, SearchResult


class TestFlashFuzzy:
    """Test FlashFuzzy class."""

    def test_create_instance(self):
        """Should create instance with default options."""
        ff = FlashFuzzy()
        assert ff.count == 0

    def test_create_with_options(self):
        """Should create instance with custom options."""
        ff = FlashFuzzy(threshold=0.5, max_errors=1, max_results=10)
        assert ff.count == 0

    def test_add_single_record(self):
        """Should add a single record."""
        ff = FlashFuzzy()
        added = ff.add({"id": 1, "name": "Hello World"})
        assert added == 1
        assert ff.count == 1

    def test_add_multiple_records(self):
        """Should add multiple records."""
        ff = FlashFuzzy()
        records = [
            {"id": 1, "name": "Apple"},
            {"id": 2, "name": "Banana"},
            {"id": 3, "name": "Cherry"},
        ]
        added = ff.add(records)
        assert added == 3
        assert ff.count == 3

    def test_search_exact(self):
        """Should find exact matches."""
        ff = FlashFuzzy()
        ff.add([
            {"id": 1, "name": "Wireless Headphones"},
            {"id": 2, "name": "Mechanical Keyboard"},
        ])

        results = ff.search("keyboard")
        assert len(results) > 0
        assert results[0].id == 2

    def test_search_fuzzy(self):
        """Should find fuzzy matches with typos."""
        ff = FlashFuzzy()
        ff.add([
            {"id": 1, "name": "Wireless Headphones"},
            {"id": 2, "name": "Mechanical Keyboard"},
        ])

        results = ff.search("keybord")  # typo
        assert len(results) > 0

    def test_search_case_insensitive(self):
        """Should be case insensitive."""
        ff = FlashFuzzy()
        ff.add([{"id": 1, "name": "Hello World"}])

        results = ff.search("HELLO")
        assert len(results) > 0
        assert results[0].id == 1

    def test_search_empty_query(self):
        """Should return empty for empty query."""
        ff = FlashFuzzy()
        ff.add([{"id": 1, "name": "Test"}])

        results = ff.search("")
        assert results == []

    def test_search_no_match(self):
        """Should return empty for no matches."""
        ff = FlashFuzzy()
        ff.add([{"id": 1, "name": "Hello World"}])

        results = ff.search("xyz123")
        assert results == []

    def test_remove_record(self):
        """Should remove record by ID."""
        ff = FlashFuzzy()
        ff.add([
            {"id": 1, "name": "First"},
            {"id": 2, "name": "Second"},
        ])

        assert ff.count == 2
        removed = ff.remove(1)
        assert removed == True
        assert ff.count == 1

        results = ff.search("first")
        assert len(results) == 0

    def test_remove_nonexistent(self):
        """Should return False for non-existent ID."""
        ff = FlashFuzzy()
        removed = ff.remove(999)
        assert removed == False

    def test_reset(self):
        """Should clear all records."""
        ff = FlashFuzzy()
        ff.add([{"id": 1, "name": "Test"}])
        assert ff.count == 1

        ff.reset()
        assert ff.count == 0

    def test_threshold_setting(self):
        """Should respect threshold setting."""
        ff = FlashFuzzy(threshold=0.9)
        ff.add([{"id": 1, "name": "abcdefgh"}])

        # High threshold might filter out partial matches
        ff.threshold = 0.1
        results_low = ff.search("abcd")

        ff.threshold = 0.99
        results_high = ff.search("abcd")

        assert len(results_low) >= len(results_high)

    def test_max_results(self):
        """Should respect max_results setting."""
        ff = FlashFuzzy(max_results=5)
        ff.add([{"id": i, "name": f"Product {i}"} for i in range(100)])

        results = ff.search("product")
        assert len(results) <= 5

    def test_result_properties(self):
        """Should have correct result properties."""
        ff = FlashFuzzy()
        ff.add([{"id": 42, "name": "Hello World"}])

        results = ff.search("hello")
        assert len(results) == 1

        r = results[0]
        assert r.id == 42
        assert 0.0 <= r.score <= 1.0
        assert r.start >= 0
        assert r.end > r.start


class TestValidation:
    """Test input validation."""

    def test_invalid_threshold(self):
        """Should reject invalid threshold."""
        with pytest.raises(ValueError):
            FlashFuzzy(threshold=1.5)

        with pytest.raises(ValueError):
            FlashFuzzy(threshold=-0.1)

    def test_invalid_max_errors(self):
        """Should reject invalid max_errors."""
        with pytest.raises(ValueError):
            FlashFuzzy(max_errors=5)

    def test_invalid_max_results(self):
        """Should reject invalid max_results."""
        with pytest.raises(ValueError):
            FlashFuzzy(max_results=1000)


class TestPerformance:
    """Performance benchmarks."""

    def test_large_dataset(self):
        """Should handle 10k records efficiently."""
        ff = FlashFuzzy()

        records = [{"id": i, "name": f"Product item number {i}"} for i in range(10000)]

        import time

        start = time.perf_counter()
        ff.add(records)
        index_time = time.perf_counter() - start

        assert ff.count == 10000
        print(f"\nIndexing 10k records: {index_time*1000:.2f}ms")

        start = time.perf_counter()
        results = ff.search("product 500")
        search_time = time.perf_counter() - start

        assert len(results) > 0
        print(f"Search in 10k records: {search_time*1000:.2f}ms")

        # Performance assertions
        assert index_time < 1.0  # < 1 second
        assert search_time < 0.05  # < 50ms
