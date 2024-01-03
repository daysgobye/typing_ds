import msgspec

from keyboard_view import KeyboardData


class AppSettings(msgspec.Struct):
    keyboard: str
    keymap: list[list[list[str]]]

    def change_keyboard(self, keyboard: str):
        self.keyboard = keyboard
        from file_manager import file_manager

        file_manager.save_settings(self)

    def change_keymap(self, keymap: list[list[str]]):
        self.keymap = keymap
        from file_manager import file_manager

        file_manager.save_settings(self)


default_layouts = [
    "Default",
    "30% QWERTY",
    "40% QWERTY",
    "60% QWERTY split",
    "60% QWERTY",
]


def change_default(settings: AppSettings, text: str):
    if text == default_layouts[1]:
        from default_keyboard_data import mdox_json, mdox_qwerty

        settings.change_keyboard(mdox_json)
        settings.change_keymap(mdox_qwerty)
    if text == default_layouts[4]:
        from default_keyboard_data import poker_json, poker_qwerty

        settings.change_keyboard(poker_json)
        settings.change_keymap(poker_qwerty)
