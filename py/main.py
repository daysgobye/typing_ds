from gui_hooks import GuiHooks
from log_path import get_log_path
from test_runner import TestRunner
import typing_ds
from window_gui import Ui_MainWindow
import sys
from PyQt5 import QtWidgets


def start_logger():
    path = get_log_path()
    typing_ds.main_loop(path)


def test():
    testRunner = TestRunner()
    testRunner.run_tests()


if __name__ == "__main__":
    # test()
    start_logger()
    app = QtWidgets.QApplication(sys.argv)
    MainWindow = QtWidgets.QMainWindow()
    ui = Ui_MainWindow()
    ui.setupUi(MainWindow)
    gui_hooks = GuiHooks(ui)
    MainWindow.show()
    exit_code = app.exec_()
    sys.exit(exit_code)
