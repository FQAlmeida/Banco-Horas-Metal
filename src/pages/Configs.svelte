<script lang="ts">
    import { Input, Label, P, Select } from "flowbite-svelte";

    import { hour_range_start, price_hour, state } from "../stores/Configs";
    import { hour_range_end } from "../stores/Configs";
    import CurrencyInput from "@canutin/svelte-currency-input";
    import Holidays from "date-holidays";
    const holidays = new Holidays("BR");
    const states_data = holidays.getStates("BR");
    const items = Object.keys(states_data).map((state) => {
        const region_data = holidays.getRegions("BR", state) ?? {};
        return {
            value: state,
            name: states_data[state],
            regions: Object.keys(region_data).map((region) => {
                return {
                    value: region,
                    name: region_data[region],
                };
            }),
        };
    });
</script>

<div
    class="container mx-auto px-4 flex flex-col gap-2 align-center justify-content"
>
    <P class="text-4xl font-bold mb-4 text-center">Configurações</P>
    <div class="flex flex-row flex-grow gap-4 flex-1">
        <div class="flex-grow">
            <Label for="hour_start" class="mb-2">Hora Entrada</Label>
            <Input
                type="time"
                id="hour_start"
                placeholder="07:00"
                required
                bind:value={$hour_range_start}
            />
        </div>
        <div class="flex-grow">
            <Label for="hour_end" class="mb-2">Hora Saida</Label>
            <Input
                type="time"
                id="hour_end"
                placeholder="17:00"
                required
                bind:value={$hour_range_end}
            />
        </div>
    </div>
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
    <Label for="state" class="mb-2">Estado</Label>
    <Select id="state" required {items} bind:value={$state} />
</div>
