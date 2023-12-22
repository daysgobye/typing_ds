import { UtteranceType } from "../types";
import { Subscribable } from "./subscribable";
import { Tests } from "./tests";
import { Utterance } from "./utterance";


export class ReportManager extends Subscribable {
    private static instance: ReportManager;
    utterances!: Utterance[]
    tester!: Tests
    private constructor() {
        super();

    }
    public static getInstance(): ReportManager {
        if (!ReportManager.instance) {
            ReportManager.instance = new ReportManager();
        }
        return ReportManager.instance;
    }
    addJson(json: string) {
        //@ts-ignore
        window.key_logs = json
        try {
            const keylogs: UtteranceType[] = JSON.parse(json)
            this.utterances = keylogs.map(log => Utterance.FromUtteranceType(log)).filter(utt => utt.wordInfo.length !== 0)
            this.tester = new Tests(this.utterances)
            this.updateSubScribers()
        } catch (error) {
            console.error("error parcing json", json, error)
        }
    }
    runTests() {
        if (this.utterances && this.tester) {
            const topWords = this.tester.getTopWords()
            const topTopWordPairs = this.tester.getTopWordGrams()
            const leastUsedGrams = this.tester.leastUsedNGram()
            const averageWPM = this.tester.getAvgWPM()
            const tests = { topWords, topTopWordPairs, leastUsedGrams, averageWPM }
            console.log(tests)
            this.updateSubScribers()

        }
    }
}
