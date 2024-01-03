import collections

import msgspec


shift_list = [
    "A",
    "B",
    "C",
    "D",
    "E",
    "F",
    "G",
    "H",
    "I",
    "J",
    "K",
    "L",
    "M",
    "N",
    "O",
    "P",
    "Q",
    "R",
    "S",
    "T",
    "U",
    "V",
    "W",
    "X",
    "Y",
    "Z",
    "",
    "!",
    "@",
    "#",
    "$",
    "%",
    "^",
    "&",
    "*",
    "(",
    ">",
    "?",
    "|",
    "_",
    "<",
    "{",
    "]",
    ":",
]
unshifted_list = [
    "a",
    "b",
    "c",
    "d",
    "e",
    "f",
    "g",
    "h",
    "i",
    "j",
    "k",
    "l",
    "m",
    "n",
    "o",
    "p",
    "q",
    "r",
    "s",
    "t",
    "u",
    "v",
    "w",
    "x",
    "y",
    "z",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "0",
    ",",
    "/",
    "|",
    "_",
    ",",
    "[",
    "]",
    ";",
]
ctrl_list = [
    "CTRL(ESC)",
    "CTRL(↚)",
    "CTRL(←)",
    "CTRL(→)",
    "CTRL(↑)",
    "CTRL(↓)",
    "CTRL(a)",
    "CTRL(b)",
    "CTRL(c)",
    "CTRL(d)",
    "CTRL(e)",
    "CTRL(f)",
    "CTRL(g)",
    "CTRL(h)",
    "CTRL(i)",
    "CTRL(j)",
    "CTRL(k)",
    "CTRL(l)",
    "CTRL(m)",
    "CTRL(n)",
    "CTRL(o)",
    "CTRL(p)",
    "CTRL(q)",
    "CTRL(r)",
    "CTRL(s)",
    "CTRL(t)",
    "CTRL(u)",
    "CTRL(v)",
    "CTRL(w)",
    "CTRL(x)",
    "CTRL(y)",
    "CTRL(z)",
    "CTRL(F1)",
    "CTRL(F2)",
    "CTRL(F3)",
    "CTRL(F4)",
    "CTRL(F5)",
    "CTRL(F6)",
    "CTRL(F7)",
    "CTRL(F8)",
    "CTRL(F9)",
    "CTRL(F10)",
    "CTRL(F11)",
    "CTRL(F12)",
    "CTRL(0)",
    "CTRL(1)",
    "CTRL(2)",
    "CTRL(3)",
    "CTRL(4)",
    "CTRL(5)",
    "CTRL(6)",
    "CTRL(7)",
    "CTRL(8)",
    "CTRL(9)",
    "CTRL(.)",
    "CTRL(/)",
    "CTRL(\\)",
    "CTRL(-)",
    "CTRL(,)",
    "CTRL(TAB)",
    "CTRL([)",
    "CTRL(])",
    "CTRL(;)",
    "CTRL(')",
    "CTRL(GUI)",
    "CTRL(\n)",
    "CTRL(0)",
    "CTRL(1)",
    "CTRL(2)",
    "CTRL(3)",
    "CTRL(4)",
    "CTRL(5)",
    "CTRL(6)",
    "CTRL(7)",
    "CTRL(8)",
    "CTRL(9)",
]
unctrl_list = [
    "ESC",
    "↚",
    "←",
    "→",
    "↑",
    "↓",
    "a",
    "b",
    "c",
    "d",
    "e",
    "f",
    "g",
    "h",
    "i",
    "j",
    "k",
    "l",
    "m",
    "n",
    "o",
    "p",
    "q",
    "r",
    "s",
    "t",
    "u",
    "v",
    "w",
    "x",
    "y",
    "z",
    "F1",
    "F2",
    "F3",
    "F4",
    "F5",
    "F6",
    "F7",
    "F8",
    "F9",
    "F10",
    "F11",
    "F12",
    "0",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    ".",
    "/",
    "\\",
    "-",
    ",",
    "TAB",
    "[",
    "]",
    ";",
    "'",
    "GUI",
    "\n",
    "0",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
]


class KeyPress(msgspec.Struct):
    key_press: str
    freq: int

    def __lt__(self, other):
        return self.freq < other.freq


class KeyPressList:
    key_presses: list[KeyPress]
    min = 0
    max = 0

    def __init__(self, letters_and_timestamps: list[list[str | int]]) -> None:
        from file_manager import file_manager

        self.key_map = file_manager.read_settings().keymap

        self.raw_letters_and_timestamps = letters_and_timestamps
        self.key_presses = []
        self.build_key_press()

    def flatten(self, xss):
        return [x for xs in xss for x in xs]

    def get_flat_keymap(self):
        keymap = []
        for layer in self.key_map:
            keymap.append(self.flatten(layer))
        return keymap

    def destructure_list_of_keys(self):
        flat_keymap = self.get_flat_keymap()
        pressed_keys = list(map(lambda x: x[0], self.raw_letters_and_timestamps))
        key_presses = []
        for raw_pressed_key in pressed_keys:
            keys_pressed = []
            if raw_pressed_key in shift_list:
                keys_pressed.append("SHIFT")
                index = shift_list.index(raw_pressed_key)
                keys_pressed.append(unshifted_list[index])
            elif raw_pressed_key in ctrl_list:
                keys_pressed.append("CTRL")
                index = ctrl_list.index(raw_pressed_key)
                keys_pressed.append(unctrl_list[index])
            else:
                keys_pressed.append(raw_pressed_key)
            for key_press in keys_pressed:
                for index, layer in enumerate(flat_keymap):
                    if key_press in layer:
                        if index != 0:
                            key_presses.append(f"MO({index})")
                        key_presses.append(key_press)
                    else:
                        key_presses.append(key_press)

        return key_presses

    def build_key_press(self):
        just_letters = self.destructure_list_of_keys()
        frequency = collections.Counter(just_letters)
        key_presses = []
        min = 1000000000000
        max = 0
        for letter, freq in dict(frequency).items():
            key_presses.append(KeyPress(letter, freq))
            if freq < min:
                min = freq
            if freq > max:
                max = freq
        self.min = min
        self.max = max

        key_presses.sort()
        self.key_presses = sorted(key_presses, reverse=True)

    def get_key_value(self, key: list[str]):
        presses: list[KeyPress] = []
        for press in self.key_presses:
            if press.key_press in key:
                presses.append(press)
            if len(presses) == len(key):
                break
        if len(presses) > 0:
            press_value = 0
            strings = []
            for press in presses:
                press_value += press.freq
                strings.append(press.key_press)
            return KeyPress("|".join(strings), press_value)
        else:
            return KeyPress("|".join(key), 0)

    def get_rank(self, freq: int):
        if freq > self.max:
            return 1
        elif freq < self.min:
            return len(self.key_presses)
        for index, press in enumerate(self.key_presses):
            if press.freq == freq:
                return index
            if press.freq > freq:
                continue
            else:
                return index - 1
