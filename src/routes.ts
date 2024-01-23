import App from './pages/EntriesPage.svelte'
import Config from './pages/Configs.svelte'
import NotFound from './pages/NotFound.svelte'

export default {

    '/': App,
    // Exact path
    '/config': Config,

    // Using named parameters, with last being optional
    // This is optional, but if present it must be the last
    '*': NotFound,
}