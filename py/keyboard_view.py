import msgspec
from PyQt5 import QtCore, QtGui, QtWidgets
import matplotlib


from key_press import KeyPressList
from window_gui import Ui_MainWindow


class KeyboardDataKey(msgspec.Struct):
    w: float = 1
    h: float = 1
    x: float = 0
    y: float = 0


class KeyboardData(msgspec.Struct):
    home_row: list[int]
    keys: list[KeyboardDataKey]


class Keyboard_Display:
    layer = 0
    buttons = []

    def __init__(
        self,
        keyboard_json: str,
        keymap: list[list[str]],
        main_window: Ui_MainWindow,
        key_usage: KeyPressList,
    ):
        self.layer_text = main_window.layer_text
        self.main_window = main_window
        self.render_box = main_window.tab
        self.keymap = keymap
        self.keyboard = msgspec.json.decode(keyboard_json, type=KeyboardData)
        self.key_usage = key_usage
        self.render_keyboard()
        self.set_up_buttons()

    def set_up_buttons(self):
        self.main_window.layer_0.clicked.connect(lambda: self.change_layer(0))
        self.main_window.layer_1.clicked.connect(lambda: self.change_layer(1))
        self.main_window.layer_2.clicked.connect(lambda: self.change_layer(2))
        self.main_window.layer_3.clicked.connect(lambda: self.change_layer(3))
        self.main_window.layer_4.clicked.connect(lambda: self.change_layer(4))

    def render_keyboard(self):
        cmap = matplotlib.cm.get_cmap("Spectral")
        norm = matplotlib.colors.Normalize(
            vmin=self.key_usage.min, vmax=self.key_usage.max
        )

        def set_button_color(value: int, button: QtWidgets.QPushButton, border: bool):
            norm_value = norm(value)
            rgba = cmap(norm_value)
            hex = matplotlib.colors.rgb2hex(rgba)
            button.setToolTip(f"hit {value} times")
            if border:
                button.setStyleSheet(
                    f"background-color : {hex}; border:1px solid blue;"
                )
            else:
                button.setStyleSheet(f"background-color : {hex};")

        if len(self.buttons) > 0:
            for index, button in enumerate(self.buttons):
                button.setText("|".join(self.keymap[self.layer][index]))
                freq = self.key_usage.get_key_value(self.keymap[self.layer][index])
                set_button_color(freq.freq, button, index in self.keyboard.home_row)

        else:
            for index, key in enumerate(self.keyboard.keys):

                def build_click(i):
                    return lambda: self.set_click(i)

                name = f"key{index}"
                button = QtWidgets.QPushButton(self.render_box)
                button.setObjectName(name)
                button.setText("|".join(self.keymap[self.layer][index]))
                button.setGeometry(
                    int(key.x * 45), int(key.y * 45), int(key.w * 45), int(key.h * 45)
                )

                freq = self.key_usage.get_key_value(self.keymap[self.layer][index])
                set_button_color(freq.freq, button, index in self.keyboard.home_row)
                button.clicked.connect(build_click(index))

                self.buttons.append(button)

    def set_click(self, index: int):
        key_from_map = " And ".join(self.keymap[self.layer][index])
        freq = self.key_usage.get_key_value(self.keymap[self.layer][index])
        rank = self.key_usage.get_rank(freq.freq)
        distance = 0
        self.main_window.key_actions_data.setText(key_from_map)
        self.main_window.Usage_count_data.setText(f"{freq.freq}")
        self.main_window.rank_data.setText(f"{rank}")

    def change_layer(self, new_layer: int):
        self.layer_text.setText(f"{new_layer}")
        self.layer = new_layer
        self.render_keyboard()
