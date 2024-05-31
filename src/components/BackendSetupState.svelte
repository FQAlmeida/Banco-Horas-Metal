<script>
    import { Alert, Button, Spinner } from "flowbite-svelte";

    import { SetupState, setupCompleted } from "../stores/Setup";
    import { InfoCircleSolid } from "flowbite-svelte-icons";
</script>

{#if $setupCompleted.state in [SetupState.Loading, SetupState.NotStarted]}
    <div class="flex justify-center items-center">
        <Spinner class="text-blue-500" size="lg" />
    </div>
{:else if $setupCompleted.state == SetupState.Error}
    <Alert dismissable>
        <div class="flex items-center gap-3">
            <InfoCircleSolid slot="icon" class="w-5 h-5" />
            <span class="text-lg font-medium">O backend teve error</span>
            <p class="mt-2 mb-4 text-sm">
                {$setupCompleted.message}
            </p>
        </div>
        <Button
            slot="close-button"
            size="xs"
            let:close
            on:click={close}
            class="ms-auto"
        >
            Fechar
        </Button>
    </Alert>
{/if}
