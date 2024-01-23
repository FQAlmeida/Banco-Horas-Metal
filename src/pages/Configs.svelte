<script lang="ts">
    import { Input, Label } from "flowbite-svelte";
    import Navbar from "../components/Navbar/Navbar.svelte";

    import { hour_range_start, price_hour } from "../stores/Configs";
    import { hour_range_end } from "../stores/Configs";
    import CurrencyInput from "@canutin/svelte-currency-input";
</script>

<Navbar />

<div class="container mx-auto px-4">
    <Label for="hour_start" class="mb-2">Hora Entrada</Label>
    <Input
        type="time"
        id="hour_start"
        placeholder="07:00"
        required
        bind:value={$hour_range_start}
    />
    <Label for="hour_end" class="mb-2">Hora Saida</Label>
    <Input
        type="time"
        id="hour_end"
        placeholder="17:00"
        required
        bind:value={$hour_range_end}
    />
    <Label for="price_hour" class="mb-2">Valor Hora</Label>
    <CurrencyInput
        name="price_hour"
        inputClasses={{
            wrapper: "form-control block",
            formatted:
                "block w-full disabled:cursor-not-allowed " +
                "disabled:opacity-50 rtl:text-right p-2.5 " +
                "focus:border-primary-500 focus:ring-primary-500 " +
                "dark:focus:border-primary-500 dark:focus:ring-primary-500 " +
                "bg-gray-50 text-gray-900 dark:bg-gray-700 dark:text-white " +
                "dark:placeholder-gray-400 border-gray-300 dark:border-gray-600 " +
                "text-sm rounded-lg",
        }}
        bind:value={$price_hour}
        locale="pt-BR"
        currency="BRL"
        isNegativeAllowed={false}
        required
    />
</div>
