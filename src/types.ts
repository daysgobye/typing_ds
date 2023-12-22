export enum Views {
    Settings = "Settings",
    Report = "Report",
    Keyboard = "Keyboard"
}

export interface UtteranceType {
    words: string
    word_info: WordInfo[]
}

export interface WordInfo {
    word: string
    letters_and_timestamps: [string, number][]
}

export interface TimedWordInfoType {
    word: string
    lettersAndTimestamps: [string, Date][]
}
export type KeyCode = {
    code: string
    also: string[]
}
export type KeyboardLayoutKey = {
    id: string
    x: number
    y: number
    w: number
    h: number
    key: KeyCode
}
export type Keyboard = {
    keys: KeyboardLayoutKey[]
    homeRow: string[]
}