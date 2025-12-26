package com.flashfuzzy

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

/**
 * Kotlin extension functions for FlashFuzzy
 */

/**
 * DSL builder for FlashFuzzy
 */
fun flashFuzzy(block: FlashFuzzyBuilder.() -> Unit): FlashFuzzy {
    return FlashFuzzyBuilder().apply(block).build()
}

class FlashFuzzyBuilder {
    var threshold: Float = 0.25f
    var maxErrors: Int = 2
    var maxResults: Int = 50

    fun build(): FlashFuzzy = FlashFuzzy(threshold, maxErrors, maxResults)
}

/**
 * Add records using a DSL
 */
inline fun FlashFuzzy.addRecords(block: RecordAdder.() -> Unit): Int {
    return RecordAdder(this).apply(block).count
}

class RecordAdder(private val ff: FlashFuzzy) {
    var count = 0

    fun record(id: Int, text: String) {
        if (ff.add(id, text)) count++
    }
}

/**
 * Suspend function for searching in a coroutine context
 */
suspend fun FlashFuzzy.searchAsync(query: String): Array<SearchResult> {
    return withContext(Dispatchers.Default) {
        search(query)
    }
}

/**
 * Extension to convert results to a List
 */
fun Array<SearchResult>.toList(): List<SearchResult> = this.asList()

/**
 * Data class wrapper for Kotlin
 */
data class FuzzyResult(
    val id: Int,
    val score: Float,
    val start: Int,
    val end: Int
)

/**
 * Convert SearchResult to Kotlin data class
 */
fun SearchResult.toKotlin(): FuzzyResult = FuzzyResult(id, score, start, end)

/**
 * Convert array of results to Kotlin data classes
 */
fun Array<SearchResult>.toKotlinList(): List<FuzzyResult> = map { it.toKotlin() }
