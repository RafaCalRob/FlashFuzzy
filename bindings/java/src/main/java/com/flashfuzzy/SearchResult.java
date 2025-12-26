package com.flashfuzzy;

/**
 * Represents a single search result.
 */
public class SearchResult {
    private final int id;
    private final float score;
    private final int start;
    private final int end;

    /**
     * Create a new search result.
     *
     * @param id Record ID
     * @param score Match score (0.0-1.0)
     * @param start Match start position
     * @param end Match end position
     */
    public SearchResult(int id, float score, int start, int end) {
        this.id = id;
        this.score = score;
        this.start = start;
        this.end = end;
    }

    /** Get the record ID */
    public int getId() { return id; }

    /** Get the match score (0.0-1.0) */
    public float getScore() { return score; }

    /** Get the match start position */
    public int getStart() { return start; }

    /** Get the match end position */
    public int getEnd() { return end; }

    @Override
    public String toString() {
        return String.format("SearchResult(id=%d, score=%.3f, start=%d, end=%d)",
            id, score, start, end);
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        SearchResult that = (SearchResult) obj;
        return id == that.id &&
               Float.compare(that.score, score) == 0 &&
               start == that.start &&
               end == that.end;
    }

    @Override
    public int hashCode() {
        int result = id;
        result = 31 * result + Float.floatToIntBits(score);
        result = 31 * result + start;
        result = 31 * result + end;
        return result;
    }
}
