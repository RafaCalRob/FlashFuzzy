package com.flashfuzzy;

/**
 * Represents a single search result.
 */
public class SearchResult {
    private final int id;
    private final float score;
    private final int start;
    private final int end;

    public SearchResult(int id, float score, int start, int end) {
        this.id = id;
        this.score = score;
        this.start = start;
        this.end = end;
    }

    public int getId() { return id; }
    public float getScore() { return score; }
    public int getStart() { return start; }
    public int getEnd() { return end; }

    @Override
    public String toString() {
        return "SearchResult(id=" + id + ", score=" + score + ")";
    }
}
