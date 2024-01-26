<script lang="ts">
    import { Button, Input, Label, Modal, P } from "flowbite-svelte";
    import { DateTime } from "luxon";
    import { ExclamationCircleOutline } from "flowbite-svelte-icons";
    import { checkpoints } from "../stores/Checkpoints";
    import type { Checkpoint } from "../models/Checkpoint";
    let modal_datetime_open = false;
    let datetime_str: string = DateTime.now().toFormat("yyyy-MM-dd'T'HH:mm");

    $: datetime = DateTime.fromFormat(datetime_str, "yyyy-MM-dd'T'HH:mm");

    $: checkpoint = {
        checkpoint: datetime,
    } as Checkpoint;

    let modal_open = false;
</script>

<Button
    class="mt-4"
    on:click={() => {
        modal_datetime_open = false;
        modal_open = true;
    }}
>
    Finalizar Registros
</Button>

<Modal bind:open={modal_datetime_open} size="sm" autoclose outsideclose>
    <Label for="datetime-start" class="mb-2">Data Finalização</Label>
    <Input
        type="datetime-local"
        id="datetime"
        required
        bind:value={datetime_str}
    />

    <Button
        class="mt-4"
        on:click={() => {
            modal_datetime_open = false;
            modal_open = true;
        }}
    >
        Finalizar
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
        <Button
            color="red"
            class="me-2 mt-2"
            on:click={async () => {
                console.log(datetime);
                return await checkpoints.add_checkpoint(checkpoint);
            }}
        >
            Confirmar
        </Button>
        <Button color="alternative">Cancelar</Button>
    </div>
</Modal>
