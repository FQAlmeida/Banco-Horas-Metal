<script lang="ts">
  import Router from "svelte-spa-router";
  import routes from "./routes";
  import Navbar from "./components/Navbar/Navbar.svelte";
  import BackendSetupState from "./components/BackendSetupState.svelte";
</script>

<body class="bg-white dark:bg-gray-800 min-h-screen pb-8">
  <Navbar />
  <div class="container auto-mx px-4">
    <BackendSetupState />
    <Router {routes} />
  </div>
</body>
