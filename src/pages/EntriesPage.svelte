<script lang="ts">
    import HourTable from "../components/HourTable.svelte";
    import Navbar from "../components/Navbar/Navbar.svelte";
    import Summary from "../components/Summary.svelte";
    import { entries } from "../stores/Entries";
    import CreateEntry from "../components/CreateEntry.svelte";
</script>

<Navbar />
<Summary />
<HourTable />
<CreateEntry />
