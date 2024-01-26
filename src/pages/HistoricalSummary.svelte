<script lang="ts">
    import { type EntryInfo } from "../models/Entry";

    let entryInfos: EntryInfo[] = [
        {
            normal: 8,
            extra: { extra_50: 2, extra_100: 3 },
            valor_normal: 10,
            valor_extra: 15,
        },
        {
            normal: 7,
            extra: { extra_50: 1, extra_100: 4 },
            valor_normal: 10,
            valor_extra: 15,
        },
        {
            normal: 6,
            extra: { extra_50: 1, extra_100: 1 },
            valor_normal: 10,
            valor_extra: 15,
        },
    ];
</script>

<main>
    <h1 class="text-2xl font-bold mb-4">Historical Summary</h1>

    {#each entryInfos as entryInfo, index}
        <div class="bg-gray-100 rounded p-4 mb-4">
            <h2 class="text-xl font-bold mb-2">Entry Info {index + 1}</h2>
            <p class="mb-2">Normal Hours: {entryInfo.normal}</p>
            <p class="mb-2">
                Extra Hours: {entryInfo.extra.extra_50}
                {entryInfo.extra.extra_100}
            </p>
            <p class="mb-2">
                Normal Value: {Intl.NumberFormat("pt-BR", {
                    style: "currency",
                    currency: "BRL",
                }).format(entryInfo.valor_normal)}
            </p>
            <p class="mb-2">
                Extra Value: {Intl.NumberFormat("pt-BR", {
                    style: "currency",
                    currency: "BRL",
                }).format(entryInfo.valor_extra)}
            </p>
        </div>
    {/each}
</main>

<style>
    /* Add your custom styles here */
</style>
