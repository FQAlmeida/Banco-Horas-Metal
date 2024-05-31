<script>
    import {
        Navbar,
        NavBrand,
        NavHamburger,
        NavUl,
        DarkMode,
    } from "flowbite-svelte";
    import { link } from "svelte-spa-router";
</script>

<Navbar let:hidden let:toggle class="mb-8">
    <NavBrand href="/">
        <img
            src="https://flowbite-svelte.com/images/flowbite-svelte-icon-logo.svg"
            class="me-3 h-6 sm:h-9"
            alt="Flowbite Logo"
        />
        <span
            class="self-center whitespace-nowrap text-xl font-semibold dark:text-white"
            >Banco de Horas</span
        >
    </NavBrand>
    <NavHamburger on:click={toggle} />
    <div class="flex flex-row">
        <NavUl {hidden}>
            <a use:link href="/">Registros</a>
            <a use:link href="/summaries">Relatórios</a>
            <a use:link href="/config">Configuração</a>
        </NavUl>
        <DarkMode />
    </div>
</Navbar>
