<script lang="ts">
    import { summary } from "../stores/Entries";
</script>

<div class="container auto-mx flex flex-col gap-6 mb-8">
    <div class="flex flex-row items-center justify-center gap-8 mx-auto">
        <div class="flex flex-col items-center justify-center">
            <dt class="mb-2 text-3xl md:text-4xl font-extrabold text-black dark:text-gray-400">
                {new Intl.NumberFormat("pt-BR", {
                    style: "currency",
                    currency: "BRL",
                }).format($summary.valor_normal)}
            </dt>
            <dd class="font-light text-gray-500 dark:text-gray-400">
                Valor Horas Normais
            </dd>
        </div>
        <div class="flex flex-col items-center justify-center">
            <dt class="mb-2 text-3xl md:text-4xl font-extrabold text-black dark:text-gray-400">
                {new Intl.NumberFormat("pt-BR", {
                    style: "currency",
                    currency: "BRL",
                }).format($summary.valor_extra)}
            </dt>
            <dd class="font-light text-gray-500 dark:text-gray-400">
                Valor Horas Extra
            </dd>
        </div>
        <div class="flex flex-col items-center justify-center">
            <dt class="mb-2 text-3xl md:text-4xl font-extrabold text-black dark:text-gray-400">
                {new Intl.NumberFormat("pt-BR", {
                    style: "currency",
                    currency: "BRL",
                }).format($summary.valor_normal + $summary.valor_extra)}
            </dt>
            <dd class="font-light text-gray-500 dark:text-gray-400">
                Valor Total
            </dd>
        </div>
    </div>
    <div class="flex flex-row items-center justify-center gap-8 mx-auto">
        <div class="flex flex-col items-center justify-center">
            <dt class="mb-2 text-3xl md:text-4xl font-extrabold text-black dark:text-gray-400">
                {$summary.normal}
            </dt>
            <dd class="font-light text-gray-500 dark:text-gray-400">
                Horas Normais
            </dd>
        </div>
        <div class="flex flex-col items-center justify-center">
            <dt class="mb-2 text-3xl md:text-4xl font-extrabold text-black dark:text-gray-400">
                {$summary.extra.extra_50}
            </dt>
            <dd class="font-light text-gray-500 dark:text-gray-400">
                Horas Extras 50%
            </dd>
        </div>
        <div class="flex flex-col items-center justify-center">
            <dt class="mb-2 text-3xl md:text-4xl font-extrabold text-black dark:text-gray-400">
                {$summary.extra.extra_100}
            </dt>
            <dd class="font-light text-gray-500 dark:text-gray-400">
                Horas Extras 100%
            </dd>
        </div>
    </div>
</div>
