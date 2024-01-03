import nltk
from file_manager import file_manager
import statistics
from autocorrect import Speller
import collections


class Tests:
    def __init__(self) -> None:
        self.keylogs = file_manager.read_keylogs()

        pass

    def get_avg_wpm(self):
        wpms = []
        for utt in self.keylogs.Logs:
            wpm = utt.get_wpm()
            if wpm > 0.9:
                wpms.append(wpm)

        return statistics.mean(wpms)

    def get_peek_wpm(self):
        top_wpm = 0
        for utt in self.keylogs.Logs:
            wpm = utt.get_wpm()
            if wpm > top_wpm:
                top_wpm = wpm
        return top_wpm

    def get_top_words(self):
        spell = Speller()
        spell_checked = spell(self.keylogs.get_big_text())
        tokens = nltk.word_tokenize(spell_checked)

        text = nltk.Text(tokens)
        tokens_l = [w.lower() for w in tokens]
        freq = nltk.FreqDist(tokens_l)
        return freq

    def get_top_words_2(self):
        words = self.keylogs.get_list_of_words()
        spell_checked_words = []
        spell = Speller()
        for word in words:
            spell_checked_words.append(spell(word))
        frequency = collections.Counter(words)
        return frequency

    def get_top_bigrams(self):
        return []

    def get_top_word_pairs(self):
        return []

    def get_top_phrases(self):
        return []
