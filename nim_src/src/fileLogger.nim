import std/times


type
  FileLogger = object
    timeRange: int
    lastTimestamp: Time
    key: string
    lastSave: Time

proc initLogger(timeRange: int = 10): FileLogger =
  result.timeRange = timeRange
  result.lastTimestamp = getTime()
  result.key = $result.lastTimestamp
  result.lastSave = result.lastTimestamp
