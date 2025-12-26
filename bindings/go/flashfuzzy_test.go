package flashfuzzy

import (
	"testing"
)

func TestNew(t *testing.T) {
	ff := New(DefaultOptions())
	if ff == nil {
		t.Fatal("Expected non-nil FlashFuzzy instance")
	}
	if ff.Count() != 0 {
		t.Errorf("Expected 0 records, got %d", ff.Count())
	}
}

func TestAddAndSearch(t *testing.T) {
	ff := New(DefaultOptions())

	// Add records
	ff.Add(1, "Wireless Headphones")
	ff.Add(2, "Mechanical Keyboard")
	ff.Add(3, "USB Cable")

	if ff.Count() != 3 {
		t.Errorf("Expected 3 records, got %d", ff.Count())
	}

	// Search exact
	results := ff.Search("keyboard")
	if len(results) == 0 {
		t.Fatal("Expected at least one result")
	}
	if results[0].ID != 2 {
		t.Errorf("Expected ID 2, got %d", results[0].ID)
	}

	// Search fuzzy
	results = ff.Search("keybord") // typo
	if len(results) == 0 {
		t.Fatal("Expected fuzzy match")
	}
}

func TestCaseInsensitive(t *testing.T) {
	ff := New(DefaultOptions())
	ff.Add(1, "Hello World")

	results := ff.Search("HELLO")
	if len(results) == 0 {
		t.Fatal("Expected case-insensitive match")
	}
}

func TestRemove(t *testing.T) {
	ff := New(DefaultOptions())
	ff.Add(1, "Test Item")

	if !ff.Remove(1) {
		t.Error("Expected successful removal")
	}

	results := ff.Search("test")
	if len(results) > 0 {
		t.Error("Expected no results after removal")
	}
}

func TestReset(t *testing.T) {
	ff := New(DefaultOptions())
	ff.Add(1, "Test")
	ff.Reset()

	if ff.Count() != 0 {
		t.Errorf("Expected 0 after reset, got %d", ff.Count())
	}
}

func TestEmptyQuery(t *testing.T) {
	ff := New(DefaultOptions())
	ff.Add(1, "Test")

	results := ff.Search("")
	if results != nil {
		t.Error("Expected nil for empty query")
	}
}

func BenchmarkAdd(b *testing.B) {
	ff := New(DefaultOptions())
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		ff.Add(uint32(i), "Product item with description")
	}
}

func BenchmarkSearch(b *testing.B) {
	ff := New(DefaultOptions())

	// Add 10k records
	for i := 0; i < 10000; i++ {
		ff.Add(uint32(i), "Product item with some description text")
	}

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		ff.Search("product")
	}
}
