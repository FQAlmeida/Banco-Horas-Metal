<script lang="ts">
  import Router from "svelte-spa-router";
  import routes from "./routes";
  import Navbar from "./components/Navbar/Navbar.svelte";
</script>

<Navbar />
<body class="container auto-mx px-4">
  <Router {routes} />
</body>
