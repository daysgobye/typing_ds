import { Match, Switch, createEffect, createSignal } from 'solid-js'
import { Views } from './types'
import Report from './views/report'
import Settings from './views/settings'
import Layout from './components/layout'
import { invoke } from "@tauri-apps/api/tauri";
import KeyboardView from './views/keyboardView'

// is this working? I think it is maybe cats and dogs will eats fish ppppmy li


function App() {
  const [view, SetView] = createSignal(Views.Report)
  createEffect(() => {
    invoke("main_loop", {
      savePath: "C:\\Users\\Owner\\Desktop"
    });
  }, [])
  return (
    <Layout currentView={view} setView={SetView}>
      <Switch fallback={<div>Not Found</div>}>
        <Match when={view() === Views.Report}>
          <Report />
        </Match>
        <Match when={view() === Views.Settings}>
          <Settings />
        </Match>
        <Match when={view() === Views.Keyboard}>
          <KeyboardView />
        </Match>
      </Switch>
    </Layout>
  )
}

export default App


