import "./app.pcss";
import "./styles.css";
import App from "./App.svelte";
import { listen } from "@tauri-apps/api/event";
import { setupCompleted, type SetupState } from "./stores/Setup";
import { invoke } from "@tauri-apps/api/core";

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

type Payload = {
  state: SetupState;
  message: string;
};
setInterval(async () => {
  let e = await invoke<Payload>("get_setup_state");
  const payload: Payload = e;
  setupCompleted.set(payload);
}, 100
);

export default app;
