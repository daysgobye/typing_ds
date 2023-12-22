import { UtteranceType, WordInfo } from "../types"
import { TimedWordInfo } from "./wordInfo";

export class Utterance implements UtteranceType {
    words: string;
    dirtyWords: string;

    word_info: WordInfo[];
    wordInfo: TimedWordInfo[]
    constructor(words: string, wordInfo: WordInfo[]) {
        this.words = words
        this.dirtyWords = `${words}`
        this.word_info = wordInfo
        this.wordInfo = wordInfo.map(info => new TimedWordInfo(info)).filter(info => info.lettersAndTimestamps.length !== 0)
        this.clean()
    }
    public static FromUtteranceType(utterance: UtteranceType) {
        return new Utterance(utterance.words, utterance.word_info)
    }
    public get startTime() {
        if (this.wordInfo.length) {

            return this.wordInfo[0].startTime
        }
        else return null

    }
    public get endTime() {
        if (this.wordInfo.length) {
            return this.wordInfo[this.wordInfo.length - 1].endTime
        }
        else return null

    }
    public get timeLength() {
        if (this.wordInfo.length) {
            //@ts-ignore
            return this.endTime.getTime() - this.startTime.getTime();
        }
        else return null

    }
    public get length() {
        return this.words.split(" ").length
    }
    public get wasExpanded() {
        if (this.wordInfo.length) {
            //@ts-ignore
            return this.timeLength < 10 * this.length
        }
        else return null
    }

    clean() {
        const wordsToClean = ["KC.ESC",
            "↚",
            "←",
            "→", "↑", "↓",
            ["[KC.LSHIFT", "{}"],
            ["[KC.RSHIFT", "{}"],
            ["9KC.LSHIFT", "()"],
            ["9KC.RSHIFT", "()"],
            ["77KC.LSHIFT", "&&"],
            ["77KC.RSHIFT", "&&"],
            ["7KC.LSHIFT", "&"],
            ["7KC.RSHIFT", "&"],
            "KC.LSHIFT",
            "KC.RSHIFT",

            "vKC.LCTRL",
            "cKC.LCTRL",
            "xKC.LCTRL",
            "aKC.LCTRL",
            "zKC.LCTRL",
            "sKC.LCTRL",
            "KC.LCTRL",


            "vKC.RCTRL",
            "cKC.RCTRL",
            "xKC.RCTRL",
            "aKC.RCTRL",
            "zKC.RCTRL",
            "sKC.RCTRL",

            "KC.RCTRL",

            "KC.LALT",
            "KC.RALT",
            "KC.CAPS",
            "KC.PGUP",
            "KC.PGDW",
            "KC.INS",
            "KC.END",
            "KC.HOME",
            "KC.DEL",
            "KC.TAB",
            "KC.GUI"]
        // console.log("before", this.words)
        wordsToClean.forEach(word => {
            if (Array.isArray(word)) {
                this.words = this.words.replaceAll(word[0], word[1])

            } else {
                this.words = this.words.replaceAll(word, "")

            }
        })
        // console.log("after", this.words)

    }


}