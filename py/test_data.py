import msgspec


class TestData(msgspec.Struct):
    avg_wpm: int
    peek_wpm: int
    top_words: list[str]
    top_bigrams: list[str]
    top_word_pairs: list[str]
    top_phrases: list[str]


class OldTests(msgspec.Struct):
    tests: list[TestData]
