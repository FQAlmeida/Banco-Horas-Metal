import { writable } from "svelte/store";

export enum SetupState {
    NotStarted = "NotStarted",
    Loading = "Loading",
    Error = "Error",
    Completed = "Completed"
}

export const setupCompleted = writable<{ state: SetupState, message?: string; }>(
    { state: SetupState.NotStarted });
