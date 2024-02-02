import { derived, get, writable } from "svelte/store";
import type { Config, HourRange } from "../models/Config";
import { invoke } from "@tauri-apps/api/core";

type ConfigDataTransfer = Omit<Config, "hour_range"> & { start_time: string, end_time: string; };


const load_config_from_database = async (): Promise<Config> => {
    const config: ConfigDataTransfer = await invoke(
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
                price_hour: new_config.price_hour
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

export const price_hour = writable<number>(24.03);
price_hour.subscribe((new_price_hour) => {
    system_config.update_price_hour(new_price_hour);
});
