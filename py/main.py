from gui_hooks import GuiHooks
import typing_ds
from window_gui import Ui_MainWindow
import sys
from PyQt5 import QtWidgets


def start_logger():
    typing_ds.main_loop()


if __name__ == "__main__":
    start_logger()
    app = QtWidgets.QApplication(sys.argv)
    MainWindow = QtWidgets.QMainWindow()
    ui = Ui_MainWindow()
    ui.setupUi(MainWindow)
    gui_hooks = GuiHooks(ui)
    MainWindow.show()
    sys.exit(app.exec_())
