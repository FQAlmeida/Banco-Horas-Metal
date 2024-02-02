<script lang="ts">
    import { Card, P } from "flowbite-svelte";
    import type { HistoricalEntryInfo } from "../models/Entry";
    import { DateTime } from "luxon";
    export let summary: HistoricalEntryInfo;
</script>

<div class="flex flex-col items-center justify-center flex-grow">
    <Card horizontal size="xl" class="w-full">
        <div class="container auto-mx flex flex-col gap-6 mb-8">
            <div class="flex flex-col items-center justify-content">
                <P class="text-3xl font-bold">Resumo do Período</P>
                <dd class="font-light text-xl text-gray-500 dark:text-gray-400">
                    Até {summary.end.checkpoint.toLocaleString(
                        DateTime.DATE_FULL,
                    )}
                </dd>
                <dd class="font-light text-md text-gray-500 dark:text-gray-400">
                    Valor Hora: {new Intl.NumberFormat("pt-BR", {
                        style: "currency",
                        currency: "BRL",
                    }).format(summary.price_hour)}
                </dd>
            </div>
            <div
                class="flex flex-row items-center justify-center gap-8 mx-auto"
            >
                <div class="flex flex-col items-center justify-center">
                    <dt class="mb-2 text-3xl md:text-4xl font-extrabold">
                        {new Intl.NumberFormat("pt-BR", {
                            style: "currency",
                            currency: "BRL",
                        }).format(summary.info.valor_normal)}
                    </dt>
                    <dd class="font-light text-gray-500 dark:text-gray-400">
                        Valor Horas Normais
                    </dd>
                </div>
                <div class="flex flex-col items-center justify-center">
                    <dt class="mb-2 text-3xl md:text-4xl font-extrabold">
                        {new Intl.NumberFormat("pt-BR", {
                            style: "currency",
                            currency: "BRL",
                        }).format(summary.info.valor_extra)}
                    </dt>
                    <dd class="font-light text-gray-500 dark:text-gray-400">
                        Valor Horas Extra
                    </dd>
                </div>
                <div class="flex flex-col items-center justify-center">
                    <dt class="mb-2 text-3xl md:text-4xl font-extrabold">
                        {new Intl.NumberFormat("pt-BR", {
                            style: "currency",
                            currency: "BRL",
                        }).format(
                            summary.info.valor_normal +
                                summary.info.valor_extra,
                        )}
                    </dt>
                    <dd class="font-light text-gray-500 dark:text-gray-400">
                        Valor Total
                    </dd>
                </div>
            </div>
            <div
                class="flex flex-row items-center justify-center gap-8 mx-auto"
            >
                <div class="flex flex-col items-center justify-center">
                    <dt class="mb-2 text-3xl md:text-4xl font-extrabold">
                        {summary.info.normal}
                    </dt>
                    <dd class="font-light text-gray-500 dark:text-gray-400">
                        Horas Normais
                    </dd>
                </div>
                <div class="flex flex-col items-center justify-center">
                    <dt class="mb-2 text-3xl md:text-4xl font-extrabold">
                        {summary.info.extra.extra_50}
                    </dt>
                    <dd class="font-light text-gray-500 dark:text-gray-400">
                        Horas Extras 50%
                    </dd>
                </div>
                <div class="flex flex-col items-center justify-center">
                    <dt class="mb-2 text-3xl md:text-4xl font-extrabold">
                        {summary.info.extra.extra_100}
                    </dt>
                    <dd class="font-light text-gray-500 dark:text-gray-400">
                        Horas Extras 100%
                    </dd>
                </div>
            </div>
        </div>
    </Card>
</div>
