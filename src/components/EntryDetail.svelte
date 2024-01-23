<script lang="ts">
    import { TableBodyCell, TableBodyRow } from "flowbite-svelte";
    import { slide } from "svelte/transition";
    import type { EntryInfo } from "../models/Entry";

    export let entry_info: EntryInfo;
</script>

<TableBodyRow>
    <TableBodyCell colspan="4" class="p-0">
        <div
            class="px-2 py-3 columns-2"
            transition:slide={{ duration: 150, axis: "y" }}
        >
            <p>Quantidade Horas Normais: {entry_info.normal}</p>
            <p>Quantidade Horas Extras (50%): {entry_info.extra.extra_50}</p>
            <p>Quantidade Horas Extras (100%): {entry_info.extra.extra_100}</p>
            <p>Valor Horas Normal: R${entry_info.valor_normal.toFixed(2)}</p>
            <p>Valor Horas Extras: R${entry_info.valor_extra.toFixed(2)}</p>
            <p>
                Valor Total: R${(entry_info.valor_normal +
                    entry_info.valor_extra).toFixed(2)}
            </p>
        </div>
    </TableBodyCell>
</TableBodyRow>
