from test_runner import TestRunner
from window_gui import Ui_MainWindow
from log_path import open_file_dialog, get_log_path


class GuiHooks:
    def __init__(self, main_window: Ui_MainWindow) -> None:
        self.main_window = main_window
        self.setup_button_clicks()
        self.setup_strings()
        pass

    def setup_button_clicks(
        self,
    ) -> None:
        self.main_window.Log_path_file_browser.clicked.connect(open_file_dialog)
        test_runner = TestRunner()
        self.main_window.run_tests.clicked.connect(lambda: test_runner.run_tests())

    def setup_strings(self):
        self.main_window.log_path.setText(get_log_path())
