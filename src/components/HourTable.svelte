<script lang="ts">
    import EntryDetail from "./EntryDetail.svelte";
    import { calculate_entry_info } from "../lib/calc_info/calc_info";
    import {
        ExclamationCircleOutline,
        TrashBinSolid,
    } from "flowbite-svelte-icons";

    import {
        Button,
        Modal,
        Table,
        TableBody,
        TableBodyCell,
        TableBodyRow,
        TableHead,
        TableHeadCell,
    } from "flowbite-svelte";
    import { hour_range, price_hour } from "../stores/Configs";
    import { entries } from "../stores/Entries";

    let openRow: number | undefined;

    const toggleRow = (i: number) => {
        openRow = openRow === i ? undefined : i;
    };

    $: modal_delete_open = false;

    let actual_entry_delete_index: number = 0;
</script>

<Table>
    <TableHead>
        <TableHeadCell>Registro</TableHeadCell>
        <TableHeadCell>Entrada</TableHeadCell>
        <TableHeadCell>Saida</TableHeadCell>
        <TableHeadCell>Ação</TableHeadCell>
    </TableHead>
    <TableBody tableBodyClass="divide-y">
        {#each $entries as entry, i}
            <TableBodyRow on:click={() => toggleRow(i)}>
                <TableBodyCell>{i + 1}</TableBodyCell>
                <TableBodyCell
                    >{entry.started_at.toFormat(
                        "dd/MM/yyyy HH:mm",
                    )}</TableBodyCell
                >
                <TableBodyCell
                    >{entry.exited_at.toFormat(
                        "dd/MM/yyyy HH:mm",
                    )}</TableBodyCell
                >
                <TableBodyCell>
                    <Button
                        on:click={() => {
                            actual_entry_delete_index = i;
                            modal_delete_open = true;
                        }}><TrashBinSolid class="mx-auto w-3 h-3" /></Button
                    ></TableBodyCell
                >
            </TableBodyRow>
            {#if openRow === i}
                <EntryDetail
                    entry_info={calculate_entry_info(
                        entry,
                        $price_hour,
                        $hour_range,
                    )}
                ></EntryDetail>
            {/if}
        {/each}
    </TableBody>
</Table>
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
            on:click={() => entries.remove_entry(actual_entry_delete_index)}
            >Confirmar</Button
        >
        <Button color="alternative">Cancelar</Button>
    </div>
</Modal>
