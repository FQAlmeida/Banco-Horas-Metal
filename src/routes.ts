import App from './pages/EntriesPage.svelte';
import Config from './pages/Configs.svelte';
import NotFound from './pages/NotFound.svelte';
import HistoricalSummary from './pages/HistoricalSummary.svelte';

export default {

    '/': App,
    // Exact path
    '/config': Config,
    '/summaries': HistoricalSummary,

    // Using named parameters, with last being optional
    // This is optional, but if present it must be the last
    '*': NotFound,
};