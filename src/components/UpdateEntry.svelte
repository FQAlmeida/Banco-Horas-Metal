<script lang="ts">
    import { Button, Input, Label, Modal, P } from "flowbite-svelte";
    import { DateTime } from "luxon";
    import { hour_range, price_hour } from "../stores/Configs";
    import { ExclamationCircleOutline } from "flowbite-svelte-icons";
    import { calculate_entry_info } from "../lib/calc_info/calc_info";
    import { entries } from "../stores/Entries";
    export let entry_index: number;
    export let modal_update_open = false;
    const [hour_start, minute_start] = $hour_range.start
        .split(":")
        .map((aux) => {
            return parseInt(aux);
        });
    const [hour_end, minute_end] = $hour_range.end.split(":").map((aux) => {
        return parseInt(aux);
    });
    let datetime_start_str: string = (
        $entries.at(entry_index)?.started_at ||
        DateTime.now().set({ hour: hour_start, minute: minute_start })
    ).toFormat("yyyy-MM-dd'T'HH:mm");
    let datetime_end_str: string = (
        $entries.at(entry_index)?.exited_at ||
        DateTime.now().set({ hour: hour_end, minute: minute_end })
    ).toFormat("yyyy-MM-dd'T'HH:mm");

    $: datetime_start = DateTime.fromFormat(
        datetime_start_str,
        "yyyy-MM-dd'T'HH:mm",
    );
    $: datetime_end = DateTime.fromFormat(
        datetime_end_str,
        "yyyy-MM-dd'T'HH:mm",
    );

    $: is_valid = datetime_start < datetime_end;

    let modal_open = false;

    $: temp_entry = {
        started_at: datetime_start,
        exited_at: datetime_end,
    };
    $: temp_entry_info = calculate_entry_info(
        temp_entry,
        $price_hour,
        $hour_range,
    );
</script>

<Modal bind:open={modal_update_open} size="sm" autoclose outsideclose>
    <P class="h3">Atualizando registro {entry_index}</P>
    <Label for="datetime-start" class="mb-2">Data Entrada</Label>
    <Input
        type="datetime-local"
        id="datetime-start"
        required
        bind:value={datetime_start_str}
    />
    <Label for="datetime-end" class="mb-2">Data Saida</Label>
    <Input
        type="datetime-local"
        id="datetime-end"
        required
        bind:value={datetime_end_str}
    />
    <Button
        class="mt-4"
        on:click={() => {
            modal_update_open = false;
            modal_open = true;
        }}
        disabled={!is_valid}
    >
        Atualizar
    </Button>
</Modal>
<Modal bind:open={modal_open} size="md" autoclose outsideclose>
    <div class="text-center">
        <ExclamationCircleOutline
            class="mx-auto mb-4 text-gray-400 w-9 h-9 dark:text-gray-200"
        />
        <h3 class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">
            Tem certeza sobre essas informações?
        </h3>
        <div class="columns-2 mx-auto my-2">
            <p>Quantidade Horas Normais: {temp_entry_info.normal}</p>
            <p>
                Quantidade Horas Extras (50%): {temp_entry_info.extra.extra_50}
            </p>
            <p>
                Quantidade Horas Extras (100%): {temp_entry_info.extra
                    .extra_100}
            </p>
            <p>
                Valor Horas Normal: R${temp_entry_info.valor_normal.toFixed(2)}
            </p>
            <p>
                Valor Horas Extras: R${temp_entry_info.valor_extra.toFixed(2)}
            </p>
            <p>
                Valor Total: R${(
                    temp_entry_info.valor_normal + temp_entry_info.valor_extra
                ).toFixed(2)}
            </p>
        </div>
        <Button
            color="red"
            class="me-2 mt-2"
            on:click={async () => {
                console.log(entry_index, temp_entry);
                return await entries.update_entry(entry_index, temp_entry);
            }}
        >
            Confirmar
        </Button>
        <Button color="alternative">Cancelar</Button>
    </div>
</Modal>
