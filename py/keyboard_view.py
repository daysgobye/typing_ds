import msgspec
from PyQt5 import QtCore, QtGui, QtWidgets


class KeyboardDataKey(msgspec.Struct):
    id: str
    w: int = 1
    h: int = 1
    x: int
    y: int


class KeyboardData(msgspec.Struct):
    home_row: list[int]
    keys: list[KeyboardDataKey]


class Keyboard_display:
    def __init__(self, keyboard: KeyboardData, render_box):
        self.keyboard = keyboard
        self.render_keyboard(render_box)
        self.groupButtons = QtWidgets.QVBoxLayout()

    def render_keyboard(self):
        for key in self.keyboard.keys:
            button = QtWidgets.QPushButton("")
            button.setText("")
            self.groupButtons.addWidget(button)
            button.setGeometry(key.x * 10, key.y * 10, key.w * 10, key.h * 10)
            button.clicked.connect(self.set_click)
        pass

    def set_click(self, key):
        print(key)


# keys="""
# keys: [
#         { id: '0', w: 1, h: 1, x: 0, y: 0 },
#         { id: '1', w: 1, h: 1, x: 1, y: 0 },
#         { id: '2', w: 1, h: 1, x: 2, y: 0 },
#         { id: '3', w: 1, h: 1, x: 3, y: 0 },
#         { id: '4', w: 1, h: 1, x: 4, y: 0 },
#         { id: '5', w: 1, h: 1, x: 6, y: 0 },
#         { id: '6', w: 1, h: 1, x: 7, y: 0 },
#         { id: '7', w: 1, h: 1, x: 8, y: 0 },
#         { id: '8', w: 1, h: 1, x: 9, y: 0 },
#         { id: '9', w: 1, h: 1, x: 10, y: 0 },

#         { id: '10', w: 1, h: 1, x: 0, y: 1  },
#         { id: '11', w: 1, h: 1, x: 1, y: 1  },
#         { id: '12', w: 1, h: 1, x: 2, y: 1  },
#         { id: '13', w: 1, h: 1, x: 3, y: 1  },
#         { id: '14', w: 1, h: 1, x: 4, y: 1  },
#         { id: '15', w: 1, h: 1, x: 6, y: 1  },
#         { id: '16', w: 1, h: 1, x: 7, y: 1  },
#         { id: '17', w: 1, h: 1, x: 8, y: 1  },
#         { id: '18', w: 1, h: 1, x: 9, y: 1  },
#         { id: '19', w: 1, h: 1, x: 10, y: 1 },

#         { id: '20', w: 1, h: 1, x: 0, y: 2 },
#         { id: '21', w: 1, h: 1, x: 1, y: 2 },
#         { id: '22', w: 1, h: 1, x: 2, y: 2 },
#         { id: '23', w: 1, h: 1, x: 3, y: 2 },
#         { id: '24', w: 1, h: 1, x: 4, y: 2 },
#         { id: '25', w: 1, h: 1, x: 6, y: 2 },
#         { id: '26', w: 1, h: 1, x: 7, y: 2 },
#         { id: '27', w: 1, h: 1, x: 8, y: 2 },
#         { id: '28', w: 1, h: 1, x: 9, y: 2 },
#         { id: '29', w: 1, h: 1, x: 10, y: 2 },

#         { id: '30', w: 1, h: 1, x: 2.25, y: 3.25 },
#         { id: '31', w: 1, h: 1, x: 3.25, y: 3.25 },
#         { id: '32', w: 1, h: 1, x: 4.25, y: 3.25 },

#         { id: '33', w: 1, h: 1, x: 5.75, y: 3.25 },
#         { id: '34', w: 1, h: 1, x: 6.75, y: 3.25 },
#         { id: '35', w: 1, h: 1, x: 7.75, y: 3.25 },

#     ]"""

poker_json = """
"home_row":[]
"keys":[
                {"x": 0, "y": 0},
                {"x": 1, "y": 0},
                {"x": 2, "y": 0},
                {"x": 3, "y": 0},
                {"x": 4, "y": 0},
                {"x": 5, "y": 0},
                {"x": 6, "y": 0},
                {"x": 7, "y": 0},
                {"x": 8, "y": 0},
                {"x": 9, "y": 0},
                {"x": 10, "y": 0},
                {"x": 11, "y": 0},
                {"x": 12, "y": 0},
                {"x": 13, "y": 0},
                {"x": 14, "y": 0},

                {"x": 0, "y": 1, "w": 1.5},
                {"x": 1.5, "y": 1},
                {"x": 2.5, "y": 1},
                {"x": 3.5, "y": 1},
                {"x": 4.5, "y": 1},
                {"x": 5.5, "y": 1},
                {"x": 6.5, "y": 1},
                {"x": 7.5, "y": 1},
                {"x": 8.5, "y": 1},
                {"x": 9.5, "y": 1},
                {"x": 10.5, "y": 1},
                {"x": 11.5, "y": 1},
                {"x": 12.5, "y": 1},
                {"x": 13.5, "y": 1, "w": 1.5},

                {"x": 0, "y": 2, "w": 1.75},
                {"x": 1.75, "y": 2},
                {"x": 2.75, "y": 2},
                {"x": 3.75, "y": 2},
                {"x": 4.75, "y": 2},
                {"x": 5.75, "y": 2},
                {"x": 6.75, "y": 2},
                {"x": 7.75, "y": 2},
                {"x": 8.75, "y": 2},
                {"x": 9.75, "y": 2},
                {"x": 10.75, "y": 2},
                {"x": 11.75, "y": 2},
                {"x": 12.75, "y": 2, "w": 2.25},

                {"x": 0, "y": 3, "w": 1.25},
                {"x": 1.25, "y": 3},
                {"x": 2.25, "y": 3},
                {"x": 3.25, "y": 3},
                {"x": 4.25, "y": 3},
                {"x": 5.25, "y": 3},
                {"x": 6.25, "y": 3},
                {"x": 7.25, "y": 3},
                {"x": 8.25, "y": 3},
                {"x": 9.25, "y": 3},
                {"x": 10.25, "y": 3},
                {"x": 11.25, "y": 3},
                {"x": 12.25, "y": 3, "w": 1.75},
                {"x": 14, "y": 3},

                {"x": 0, "y": 4, "w": 1.25},
                {"x": 1.25, "y": 4, "w": 1.25},
                {"x": 2.5, "y": 4, "w": 1.25},
                {"x": 3.75, "y": 4, "w": 2.75},
                {"x": 6.5, "y": 4, "w": 1.25},
                {"x": 7.75, "y": 4, "w": 2.25},
                {"x": 10, "y": 4, "w": 1.25},
                {"x": 11.25, "y": 4, "w": 1.25},
                {"x": 12.5, "y": 4, "w": 1.25},
                {"x": 13.75, "y": 4, "w": 1.25}
]"""
poker_keyboard = msgspec.json(poker_json, type=KeyboardData)
