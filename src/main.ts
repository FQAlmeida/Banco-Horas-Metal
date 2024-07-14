import "./app.pcss";
import "./styles.css";
import App from "./App.svelte";
import { setupCompleted, SetupState } from "./stores/Setup";
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

interface Payload {
  state: SetupState;
  message: string;
}
const intervalId = setInterval(async () => {
  const e = await invoke<Payload>("get_setup_state");
  const payload: Payload = e;
  setupCompleted.set(payload);
  if (payload.state === SetupState.Completed || payload.state === SetupState.Error) {
    clearInterval(intervalId);
  }
}, 100
);


export default app;
