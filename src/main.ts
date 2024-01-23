import "./app.pcss";
import "./styles.css";
import App from "./App.svelte";

const default_app_div = document.createElement("div");
const app_div = document.getElementById("app");

if (app_div == null) {
  default_app_div.id = "app";
  document.body.appendChild(default_app_div);
}

const target = app_div ?? default_app_div;

const app = new App({
  target,
});

export default app;
