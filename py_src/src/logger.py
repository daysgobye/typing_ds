import time


import json


class Logger:
    tmp_log = {}

    def __init__(self, time_range=10) -> None:
        self.time_range = time_range
        self.last_timestamp = int(time.time())
        self.key = f"{self.last_timestamp}"
        self.last_save = self.last_timestamp

    def save_word(self, word):
        current_time = int(time.time())
        current_key = f"{current_time}"

        if current_time - self.last_timestamp < self.time_range:
            current_key = self.key
        else:
            self.key = current_key
            self.handle_save()

        if current_key in self.tmp_log:
            self.tmp_log[current_key] += word
        else:
            self.tmp_log[current_key] = word
        self.last_timestamp = current_time

    def handle_save(self):
        current_time = int(time.time())
        if current_time - self.last_save > 10:
            print("saving file", current_time - self.last_save)
            json_object = json.dumps(self.tmp_log, indent=4)
            self.tmp_log = {}
            with open("data.json", "a") as f:
                f.write(json_object + ",\n")
            self.last_save = current_time
        else:
            print("it can wait", current_time)
