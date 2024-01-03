from file_manager import file_manager
from app_settings import change_default, default_layouts
from test_data import TestData
from test_runner import TestRunner
from window_gui import Ui_MainWindow
from log_path import open_file_dialog, get_log_path


class GuiHooks:
    def __init__(self, main_window: Ui_MainWindow) -> None:
        self.main_window = main_window
        self.setup_button_clicks()
        self.setup_strings()
        self.app_settings = file_manager.read_settings()
        self.test_runner = TestRunner(main_window, self.app_settings)

        pass

    def setup_button_clicks(
        self,
    ) -> None:
        self.main_window.Log_path_file_browser.clicked.connect(open_file_dialog)
        self.main_window.run_tests.clicked.connect(self.rerun_tests)

        for layout in default_layouts:
            self.main_window.keyboard_layout_dropdown.addItem(layout)

        def on_text_change(text: str):
            change_default(self.app_settings, text)

        self.main_window.keyboard_layout_dropdown.currentTextChanged.connect(
            on_text_change
        )

    def rerun_tests(self):
        data = self.test_runner.run_tests()
        self.render_test_data(data)

    def setup_strings(self):
        self.main_window.log_path.setText(get_log_path())

    def render_test_data(self, data: TestData):
        self.main_window.avg_wpm.setText(f"{data.avg_wpm}")
        self.main_window.peek_wpm.setText(f"{data.peek_wpm}")
