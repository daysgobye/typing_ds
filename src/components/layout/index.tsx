import { For } from "solid-js/web";
import { Views } from "../../types";


type Props = {

    children: any,
    currentView: () => Views,
    setView: (view: Views) => void

};

export default function Layout({ currentView, setView, children }: Props) {
    const viewOptions = [{
        view: Views.Report,
        icon: (<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" /></svg>)
    }, {
        view: Views.Settings,
        icon: (<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>)
    },
    {
        view: Views.Keyboard,
        icon: (<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>)
    }
    ]

    return (
        <div class="flex flex-col w-full">
            <div class="flex w-full">
                {children}
            </div>
            <nav>
                <div class="btm-nav">
                    <For each={viewOptions}>
                        {(nav) => (
                            <button
                                onClick={() => { setView(nav.view) }}
                                class={`text-primary ${currentView() === nav.view ? "active" : ""}`}
                            >
                                {nav.icon}
                            </button>
                        )}
                    </For>


                </div>

            </nav>
        </div>

    );

}
