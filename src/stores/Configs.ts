import { derived, get, writable } from "svelte/store";
import type { ConfigData, HourRange } from "../models/Config";
import { invoke } from "@tauri-apps/api/core";

type ConfigPayload = ConfigData & { start_time: string, end_time: string; };


const load_config_from_database = async (): Promise<ConfigData> => {
    const config: ConfigPayload = await invoke(
        "get_configuration");
    return {
        ...config,
        hour_range: {
            start: config.start_time,
            end: config.end_time,
        },
    };
};

const create_system_config = async () => {
    const { subscribe, set, update } = writable(await load_config_from_database());
    const reset = async () => {
        set(await load_config_from_database());
    };
    const update_hour_range = (new_hour_range: HourRange) => {
        update((old_config) => {
            return {
                ...old_config,
                hour_range: new_hour_range
            };
        });
    };
    const update_state = (new_state: string) => {
        update((old_config) => {
            return {
                ...old_config,
                state: new_state
            };
        });
    };

    const update_price_hour = (new_price_hour: number) => {
        update((old_config) => {
            return {
                ...old_config,
                price_hour: new_price_hour
            };
        });
    };
    subscribe((new_config) => {
        const payload = {
            id: new_config.id,
            configuration: {
                start_time: new_config.hour_range.start,
                end_time: new_config.hour_range.end,
                price_hour: new_config.price_hour,
                state: new_config.state,
            }
        };
        invoke("update_configuration", payload).catch((e) => {
            console.error(e);
        });
    });
    return {
        subscribe,
        set,
        update,
        update_hour_range,
        update_price_hour,
        update_state,
        reset,
    };
};

const system_config = await create_system_config();

export const hour_range_start = writable<string>(get(system_config).hour_range.start);

export const hour_range_end = writable<string>(get(system_config).hour_range.end);

export const hour_range = derived(
    [hour_range_start, hour_range_end],
    ([$hour_range_start, $hour_range_end]) => {
        return { start: $hour_range_start, end: $hour_range_end } as HourRange;
    });
hour_range.subscribe((new_hour_range) => {
    system_config.update_hour_range(new_hour_range);
});

export const price_hour = writable<number>(get(system_config).price_hour);
price_hour.subscribe((new_price_hour) => {
    system_config.update_price_hour(new_price_hour);
});

export const state = writable<string>(get(system_config).state);
state.subscribe((new_state) => {
    system_config.update_state(new_state);
});
