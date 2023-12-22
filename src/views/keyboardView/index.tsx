import { For, createEffect, createSignal } from "solid-js"
import { KeyboardLayoutKey } from "../../types"

import { qwertySplit36key } from "../../logic/keyboards"

const keySize = 25

function KeyboardView() {
    const [keys, setKeys] = createSignal<KeyboardLayoutKey[]>(qwertySplit36key.keys),
        [selectedKey, setSelectedKey] = createSignal<KeyboardLayoutKey | null>(null)
    createEffect(() => {
        window.addEventListener("keydown", keyDown)
        return () => window.removeEventListener("keydown", keyDown)
    }, [])

    const buildStyleObject = (layoutKey: KeyboardLayoutKey) => {
        return {
            width: `${layoutKey.w * keySize}px`,
            height: `${layoutKey.h * keySize}px`,
            left: `${layoutKey.x * keySize}px`,
            top: `${layoutKey.y * keySize}px`
        }
    }

    const keyDown = (e: any) => {

        const currentKey = selectedKey()
        const save = () => {

        }

        if (currentKey) {


        }


    }
    return (
        <div class="flex flex-col"  >

            <div class="relative h-[50vh] w-screen">
                <For each={keys()}>
                    {(item, index) => (
                        <button class={`absolute border ${selectedKey()?.id === item.id ? "border-primary" : ""}`}
                            onClick={() => setSelectedKey(item)}
                            style={buildStyleObject(item)}
                        >
                            {item.key.code}
                        </button>
                    )}
                </For>
            </div>
            <div class="flex flex-col">
                <h3>settings</h3>
                <div class="flex">

                </div>
            </div>
        </div>
    )
}

export default KeyboardView

