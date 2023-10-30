import { createSignal } from 'solid-js'
import { Views } from './types'
import Report from './views/report'
import Settings from './views/settings'
import Layout from './components/layout'




function App() {
  const [view, SetView] = createSignal(Views.Report)
  const returnCorrectView = () => {
    switch (view()) {
      case Views.Report:
        return <Report />
      case Views.Settings:
        return <Settings />


    }
  }
  return (
    <Layout currentView={view} setView={SetView}>

      <div className="bg-red-200">thank you. thank you. </div>
      <h1 class="text-3xl font-bold underline">
        Hello world!
      </h1>
      {returnCorrectView()}
    </Layout>
  )
}

export default App
