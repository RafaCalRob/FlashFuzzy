# Flash-Fuzzy Android

High-performance fuzzy search for Android with Kotlin extensions.

## Installation

### Gradle

```groovy
implementation 'com.flashfuzzy:flash-fuzzy-android:0.1.0'
```

## Quick Start

### Kotlin (Recommended)

```kotlin
import com.flashfuzzy.*

// Using DSL builder
val ff = flashFuzzy {
    threshold = 0.3f
    maxErrors = 2
    maxResults = 50
}

// Add records with DSL
ff.addRecords {
    record(1, "Wireless Headphones")
    record(2, "Mechanical Keyboard")
    record(3, "USB-C Cable")
}

// Search
val results = ff.search("keybord").toKotlinList()
results.forEach { println("${it.id}: ${it.score}") }

// Async search in coroutine
lifecycleScope.launch {
    val asyncResults = ff.searchAsync("keyboard")
    // Update UI with results
}
```

### Java

```java
import com.flashfuzzy.FlashFuzzy;
import com.flashfuzzy.SearchResult;

FlashFuzzy ff = new FlashFuzzy();
ff.add(1, "Wireless Headphones");
ff.add(2, "Mechanical Keyboard");

SearchResult[] results = ff.search("keybord");
for (SearchResult r : results) {
    Log.d("Search", "ID: " + r.getId() + ", Score: " + r.getScore());
}
```

## Building Native Libraries

The library requires native .so files for each ABI. Build with cargo-ndk:

```bash
# Install cargo-ndk
cargo install cargo-ndk

# Add Android targets
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android

# Build for all ABIs
cd bindings/java
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 -t x86 -o ../android/flashfuzzy/src/main/jniLibs build --release
```

## API

### FlashFuzzy

| Method | Description |
|--------|-------------|
| `add(id, text)` | Add a record |
| `search(query)` | Search for matches |
| `remove(id)` | Remove by ID |
| `reset()` | Clear all records |
| `getCount()` | Get record count |

### Kotlin Extensions

| Extension | Description |
|-----------|-------------|
| `flashFuzzy { }` | DSL builder |
| `addRecords { }` | DSL for adding records |
| `searchAsync(query)` | Suspend function for coroutines |
| `toKotlinList()` | Convert results to data classes |

## Performance

- Sub-millisecond search in large datasets
- Native Rust core via JNI
- Minimal memory footprint
- Thread-safe

## License

MIT
