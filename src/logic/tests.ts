import { Utterance } from "./utterance";
import { Methodius } from "./methodius"

// install these
// https://www.npmjs.com/package/english-bigrams
// https://www.npmjs.com/package/methodius

export class Tests {
    utterances: Utterance[]
    fullText: string
    methodius: any
    constructor(utterances: Utterance[]) {
        this.utterances = utterances
        this.fullText = this.getFullText()
        this.methodius = new Methodius(this.fullText);

    }


    getWPMForUtterance(utt: Utterance) {
        const length = utt.timeLength ? utt.timeLength : 1 / 1000,
            partOfMin = (length / 60) * 100,
            wordCount = utt.words.length / 5,
            wpm = wordCount / partOfMin
        return wpm

    }
    average = (array: number[]) => array.reduce((a, b) => a + b) / array.length;

    getAvgWPM() {
        const wpms = this.utterances.map(this.getWPMForUtterance)
        return this.average(wpms)
    }
    getTopWords(letterCount: number = 4) {
        const topWords: Map<string, number> = this.methodius.getTopWords(100)
        const filteredWords = Array.from(topWords.keys()).filter(word => word.length > letterCount)
        return filteredWords
    }
    getTopWordGrams() {
        const filterGrams = (grams: string[]) => grams.every((word, i) => {
            const length = word.length > 1,
                match = i == 0 ? true : word !== grams[0]
            return length && match
        })
        const pairs = Methodius.getTopGrams(Methodius.getFrequencyMap(Methodius.getWordNGrams(this.fullText).filter(filterGrams).map((grams: string[]) => grams.join(" "))))
        const triplets = Methodius.getTopGrams(Methodius.getFrequencyMap(Methodius.getWordNGrams(this.fullText, 3).filter(filterGrams).map((grams: string[]) => grams.join(" "))))
        const quads = Methodius.getTopGrams(Methodius.getFrequencyMap(Methodius.getWordNGrams(this.fullText, 4).filter(filterGrams).map((grams: string[]) => grams.join(" "))))
        return { pairs: Array.from(pairs.keys()), triplets: Array.from(triplets.keys()), quads: Array.from(quads.keys()) }

    }
    getFullText() {
        return this.utterances.map(utt => utt.words).join(" ")
    }
    leastUsedNGram() {
        const bigrams = Methodius.getBottomGrams(Methodius.getFrequencyMap(Methodius.getNGrams(this.fullText))),
            trigrams = Methodius.getBottomGrams(Methodius.getFrequencyMap(Methodius.getNGrams(this.fullText, 3)))
        return {
            bi: Array.from(bigrams.keys()),
            tri: Array.from(trigrams.keys())

        }
    }

}