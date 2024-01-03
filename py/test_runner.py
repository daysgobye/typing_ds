from app_settings import AppSettings
from file_manager import file_manager
from test_data import TestData
from tests import Tests
from window_gui import Ui_MainWindow
from keyboard_view import Keyboard_Display
from pyqtspinner import WaitingSpinner


class TestRunner:
    def __init__(self, main_window: Ui_MainWindow, app_settings: AppSettings) -> None:
        self.main_window = main_window
        self.app_settings = app_settings
        self.tests = Tests()
        self.keyboard_display = Keyboard_Display(
            self.app_settings.keyboard,
            self.app_settings.keymap,
            self.main_window,
            self.tests.keylogs.key_Press_list,
        )

        pass

    def get_lastTests(self) -> None:
        pass

    def run_tests(self) -> None:
        spinner = WaitingSpinner(
            self.main_window.tabWidget_2,
            roundness=100.0,
            fade=11.4,
            radius=22,
            lines=14,
            line_length=26,
            line_width=15,
            speed=0.98,
            color=(0, 0, 0),
        )
        spinner.start()

        avg_wpm = self.tests.get_avg_wpm()
        peek_wpm = self.tests.get_peek_wpm()
        top_words = self.tests.get_top_words()
        # top_words2 = self.tests.get_top_words_2()
        top_bigrams = self.tests.get_top_bigrams()
        top_word_pairs = self.tests.get_top_word_pairs()
        top_phrases = self.tests.get_top_phrases()
        print(dict(top_words))
        # print(dict(top_words2))

        test_data = TestData(
            avg_wpm, peek_wpm, top_words, top_bigrams, top_word_pairs, top_phrases
        )
        spinner.stop()

        return test_data
