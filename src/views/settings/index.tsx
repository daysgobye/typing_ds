import { createEffect, createSignal } from "solid-js"
import { getSetting } from "../../logic/settings";


function Settings() {
    const [path, setPath] = createSignal(""),
        getAndSetPath = async () => {
            const path = await getSetting("path")
            if (typeof path === "string") {
                setPath(path)
            }

        }
    createEffect(() => {
        getAndSetPath()
    });

    return (
        <>
            path:{path()}
        </>
    )
}

export default Settings
