import { TimedWordInfoType, WordInfo } from "../types";

export class TimedWordInfo implements TimedWordInfoType {
    word: string;
    lettersAndTimestamps: [string, Date][];

    constructor(wordInfo: WordInfo) {
        this.word = wordInfo.word,
            this.lettersAndTimestamps = wordInfo.letters_and_timestamps
                .map(letters => ([letters[0], new Date(letters[1] * 1000)]))


    }

    public get startTime() {
        return this.lettersAndTimestamps[0][1]
    }
    public get endTime() {
        return this.lettersAndTimestamps[this.lettersAndTimestamps.length - 1][1]
    }
    public get timeLength() {
        return this.endTime.getTime() - this.startTime.getTime();
    }
    public get length() {
        return this.word.length
    }
    public get wasExpanded() {
        return this.timeLength < 10
    }


}