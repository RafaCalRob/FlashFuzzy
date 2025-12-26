# Flash-Fuzzy Java

High-performance fuzzy search engine for Java/Kotlin, powered by Rust via JNI.

## Installation

### Maven

```xml
<dependency>
    <groupId>com.flashfuzzy</groupId>
    <artifactId>flash-fuzzy</artifactId>
    <version>0.1.0</version>
</dependency>
```

### Gradle

```groovy
implementation 'com.flashfuzzy:flash-fuzzy:0.1.0'
```

## Quick Start

### Java

```java
import com.flashfuzzy.FlashFuzzy;
import com.flashfuzzy.SearchResult;

// Create instance
FlashFuzzy ff = new FlashFuzzy();

// Add records
ff.add(1, "Wireless Headphones");
ff.add(2, "Mechanical Keyboard");
ff.add(3, "USB-C Cable");

// Search with typos
SearchResult[] results = ff.search("keybord"); // Note the typo
for (SearchResult r : results) {
    System.out.printf("ID: %d, Score: %.2f%n", r.getId(), r.getScore());
}

// Clean up
ff.close();
```

### Kotlin

```kotlin
import com.flashfuzzy.FlashFuzzy

FlashFuzzy().use { ff ->
    ff.add(1, "Wireless Headphones")
    ff.add(2, "Mechanical Keyboard")

    val results = ff.search("keybord")
    results.forEach { println("ID: ${it.id}, Score: ${it.score}") }
}
```

## API

### FlashFuzzy

```java
// Constructors
FlashFuzzy()                                    // Default options
FlashFuzzy(float threshold, int maxErrors, int maxResults)

// Methods
boolean add(int id, String text)                // Add a record
int addAll(Map<Integer, String> records)        // Add multiple records
SearchResult[] search(String query)             // Search for matches
boolean remove(int id)                          // Remove by ID
void reset()                                    // Clear all records
int getCount()                                  // Get record count

// Configuration
void setThreshold(float threshold)              // 0.0-1.0
void setMaxErrors(int maxErrors)                // 0-3
void setMaxResults(int maxResults)              // 1-100
```

### SearchResult

```java
int getId()         // Record ID
float getScore()    // Match score (0.0-1.0)
int getStart()      // Match start position
int getEnd()        // Match end position
```

## Building

```bash
# Build the native library
cd bindings/java
cargo build --release

# Build Java
mvn package

# Run tests (requires native library in java.library.path)
mvn test -Djava.library.path=target/release
```

## Performance

- Sub-millisecond search in 100k+ records
- Bloom filter pre-filtering for O(1) rejection
- JNI overhead minimized with batch operations

## License

MIT
