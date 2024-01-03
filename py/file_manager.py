import pickle
from log_path import get_log_path
from default_keyboard_data import poker_json, poker_qwerty

import msgspec
from keylogs import KeyLogManager, RawLogs
from app_settings import AppSettings


class FileManager:
    def __init__(self) -> None:
        self.path = get_log_path()

    def read_settings(self) -> AppSettings:
        try:
            with open(self.path + "/app_setting.json", "r", encoding="utf8") as my_file:
                settings = my_file.read()
                return msgspec.json.decode(settings, type=AppSettings, strict=False)
        except FileNotFoundError:
            with open(self.path + "/app_setting.json", "wb") as my_file:
                settings = AppSettings(poker_json, poker_qwerty)
                my_file.write(msgspec.json.encode(settings))
                return settings

    def save_settings(self, settings: AppSettings):
        try:
            with open(self.path + "/app_setting.json", "wb") as my_file:
                my_file.write(msgspec.json.encode(settings))
            return True
        except FileNotFoundError:
            return False

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
