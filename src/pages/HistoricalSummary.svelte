<script lang="ts">
    import { Spinner } from "flowbite-svelte";
    import { historical_entries_summaries } from "../stores/Entries";
    import HistoricalSummary from "../components/HistoricalSummary.svelte";

    $: entry_infos = historical_entries_summaries;
</script>

{#await $entry_infos}
    <div class="text-center">
        <Spinner />
    </div>
{:then entry_infos}
    <main class="mt-6">
        <h1 class="text-5xl font-bold mb-4 text-center mb-8">
            Resumos Históricos
        </h1>
        <div class="flex flex-col flex-grow gap-6">
            {#each entry_infos as entry_info}
                <HistoricalSummary summary={entry_info} />
            {/each}
        </div>
    </main>
{/await}
