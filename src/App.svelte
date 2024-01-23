<script lang="ts">
  import Router from "svelte-spa-router";
  import routes from "./routes";
</script>

<body class="container auto-mx px-4">
  <Router {routes} />
</body>
