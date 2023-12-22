import { createEffect } from "solid-js"
import { invoke } from "@tauri-apps/api/tauri";
import { ReportManager } from "../../logic/reportManager";

const reportManager = ReportManager.getInstance()

function Report() {

    const update = () => {
    }

    createEffect(() => {
        const readKeylogger = async () => {



            const contents = await invoke("read_keylogger", {
            });
            reportManager.addJson(contents as string)
            reportManager.runTests()

        }

        readKeylogger()
        reportManager.Subscribe(update)

    }, [])

    return (
        <>

            I want to show you your Report
        </>
    )
}

export default Report

