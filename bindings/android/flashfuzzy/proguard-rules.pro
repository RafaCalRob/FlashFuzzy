# Flash-Fuzzy ProGuard Rules

# Keep native methods
-keepclasseswithmembernames class * {
    native <methods>;
}

# Keep FlashFuzzy and SearchResult classes
-keep class com.flashfuzzy.FlashFuzzy { *; }
-keep class com.flashfuzzy.SearchResult { *; }
-keep class com.flashfuzzy.FlashFuzzyKt { *; }
-keep class com.flashfuzzy.FuzzyResult { *; }
