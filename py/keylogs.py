from typing import List
import msgspec


from key_press import KeyPressList


class WordInfo(msgspec.Struct):
    word: str = ""
    letters_and_timestamps: list[list[str | int]] = []

    def get_start_time(self):
        return self.letters_and_timestamps[0][1]

    def get_end_time(self):
        return self.letters_and_timestamps[-1][1]

    def get_duration(self):
        return self.get_end_time() - self.get_start_time()

    def was_expanded(self):
        return self.get_duration() < 10


class Utterance(msgspec.Struct):
    words: str = ""
    word_info: list[WordInfo] = []

    def __post_init__(self):
        cleaned_word_info = filter(
            lambda x: len(x.letters_and_timestamps) != 0, self.word_info
        )
        self.word_info = list(cleaned_word_info)

    def get_start_time(self):
        return self.word_info[0].get_start_time()

    def get_end_time(self):
        return self.word_info[-1].get_end_time()

    def get_duration(self):
        return self.get_end_time() - self.get_start_time()

    def get_wpm(self):
        partOfMin = (self.get_duration() / 60) * 100
        wordCount = len(self.words) / 5
        if partOfMin < 1:
            return 0.0
        wpm = wordCount / partOfMin
        return wpm

    def was_expanded(self):
        return self.get_duration() < 10 * len(self.word_info)

    def contains_expanded(self):
        some_expanded = False
        for word in self.word_info:
            if word.was_expanded():
                some_expanded = True
        return some_expanded


class RawLogs(msgspec.Struct):
    utterance: list[Utterance] = []

    def __post_init__(self):
        cleaned_utterance = filter(lambda x: len(x.word_info) != 0, self.utterance)
        self.utterance = list(cleaned_utterance)


class KeyLogManager:
    big_text: str
    list_of_words: List[str]
    Logs: List[Utterance]
    key_Press_list: KeyPressList

    def __init__(self, key_logs: RawLogs) -> None:
        self.Logs = key_logs.utterance
        self.list_of_words = []
        self.big_text = ""
        key_Press_list = []
        for utt in self.Logs:
            for word in utt.word_info:
                for ltt in word.letters_and_timestamps:
                    key_Press_list.append(ltt)
        self.key_Press_list = KeyPressList(key_Press_list)

    def get_key_logs(self) -> List[Utterance]:
        return self.Logs

    def get_big_text(self) -> str:
        if self.big_text != "":
            return self.big_text
        self.big_text = ""
        for utterance in self.Logs:
            self.big_text += utterance.words
        return self.big_text

    def get_list_of_words(self) -> List[str]:
        if len(self.list_of_words) != 0:
            return self.list_of_words
        for utterance in self.Logs:
            for word in utterance.words:
                self.list_of_words.append(word)
        return self.list_of_words
