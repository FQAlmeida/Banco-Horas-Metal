import { derived, writable } from "svelte/store";
import type { HourRange } from "../models/Config";

export const hour_range_start = writable<string>("07:00");

export const hour_range_end = writable<string>("17:00");

export const hour_range = derived(
    [hour_range_start, hour_range_end],
    ([$hour_range_start, $hour_range_end]) => {
        return { start: $hour_range_start, end: $hour_range_end } as HourRange;
    });
export const price_hour = writable<number>(24.03);
