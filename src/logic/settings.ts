import { SettingsManager, get } from 'tauri-settings';
import { Path } from 'tauri-settings/dist/types/dot-notation';

type Schema = {
    path: string
    pause: boolean;
}

const settingsManager = new SettingsManager<Schema>(
    { // defaults
        path: '/',
        pause: false
    },
    { // options
        fileName: 'typing_ds-settings'
    }
)

settingsManager.initialize().then((foo) => {
    console.log("meow", foo)

})
export const getSetting = async (key: Path<Schema>) => {
    return await get<Schema>(key).then(value => value)

}