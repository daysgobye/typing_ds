from log_path import get_log_path
import msgspec
from keylogs import KeyLogManager, RawLogs, Utterance


class FileManager:
    def __init__(self) -> None:
        self.path = get_log_path()

    def get_last_tests(self):
        pass

    def save_tests(self):
        pass

    def read_keylogs(self) -> KeyLogManager:
        keylogs = ""
        try:
            with open(self.path + "/keylogger.toml", "r", encoding="utf8") as my_file:
                keylogs = my_file.read()
        except FileNotFoundError:
            print("File not found")
        parces_keylogs = msgspec.toml.decode(keylogs, type=RawLogs, strict=False)

        return KeyLogManager(parces_keylogs)


file_manager = FileManager()
