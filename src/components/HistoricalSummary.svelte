<script lang="ts">
    import { Button, Card, Modal, P } from "flowbite-svelte";
    import type { HistoricalEntryInfo } from "../models/Entry";
    import { DateTime } from "luxon";
    import {
        ExclamationCircleOutline,
        TrashBinSolid,
    } from "flowbite-svelte-icons";
    import { checkpoints } from "../stores/Checkpoints";
    export let summary: HistoricalEntryInfo;

    $: modal_delete_open = false;
    let actual_checkpoint_delete_index: number = 0;
</script>

<div class="flex flex-col items-center justify-center flex-grow">
    <Card horizontal size="xl" class="w-full">
        <div class="container auto-mx flex flex-col gap-6 mb-8">
            <div class="grid grid-cols-12">
                <div class="col-span-1" />
                <div
                    class="col-span-10 flex flex-col items-center justify-content"
                >
                    <P class="text-3xl font-bold">Resumo do Período</P>
                    <dd
                        class="font-light text-xl text-gray-500 dark:text-gray-400"
                    >
                        Até {summary.end.checkpoint.toLocaleString(
                            DateTime.DATE_FULL,
                        )}
                    </dd>
                    <dd
                        class="font-light text-md text-gray-500 dark:text-gray-400"
                    >
                        Valor Hora: {new Intl.NumberFormat("pt-BR", {
                            style: "currency",
                            currency: "BRL",
                        }).format(summary.price_hour)}
                    </dd>
                </div>
                <Button
                    class="col-span-1 my-5 mx-6"
                    on:click={() => {
                        // checkpoints.remove_checkpoint(summary.end.id);
                        actual_checkpoint_delete_index = summary.end.id;
                        modal_delete_open = true;
                    }}
                >
                    <TrashBinSolid class="mx-auto w-3 h-3" />
                </Button>
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
<Modal bind:open={modal_delete_open} size="sm" autoclose outsideclose>
    <div class="text-center">
        <ExclamationCircleOutline
            class="mx-auto mb-4 text-gray-400 w-9 h-9 dark:text-gray-200"
        />
        <h3 class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">
            Tem certeza que deseja remover?
        </h3>
        <Button
            color="red"
            class="me-2 mt-2"
            on:click={() =>
                checkpoints.remove_checkpoint(actual_checkpoint_delete_index)}
            >Confirmar</Button
        >
        <Button color="alternative">Cancelar</Button>
    </div>
</Modal>
