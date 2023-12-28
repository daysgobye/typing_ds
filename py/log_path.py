from PyQt5 import QtWidgets


def set_log_path(string):
    pass


def get_log_path() -> str:
    path = ""
    try:
        with open("path.txt") as my_file:
            path = my_file.read()
    except FileNotFoundError:
        import install_setup

        install_setup.setup()
        with open("path.txt") as my_file:
            path = my_file.read()
    return path


def open_file_dialog() -> None:
    folderpath = QtWidgets.QFileDialog.getExistingDirectory(caption="Select Folder")
    with open("path.txt", "w+") as my_file:
        my_file.write(folderpath)
