import nltk
from file_manager import file_manager
import msgspec


class Tests:
    def __init__(self) -> None:
        self.keylogs = file_manager.read_keylogs()
        pass

    def get_avg_wpm(self):
        pass

    def get_peek_wpm(self):
        pass

    def get_top_words(self):
        tokens = nltk.word_tokenize(self.keylogs.get_big_text())
        print(self.keylogs.get_big_text())
        print(tokens)
        text = nltk.Text(tokens)
        print(text)
        tokens_l = [w.lower() for w in tokens]
        freq = nltk.FreqDist(tokens_l)
        print(freq.most_common(10))

    def get_top_bigrams(self):
        pass

    def get_top_word_pairs(self):
        pass

    def get_top_phrases(self):
        pass


class TestData(msgspec.Struct):
    avg_wpm: list[int]
    peek_wpm: list[int]
    top_words: list[str]
    top_bigrams: list[str]
    top_word_pairs: list[str]
    top_phrases: list[str]
