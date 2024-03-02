import { DateTime } from "luxon";
import type { Entry, EntryInfo, HistoricalEntryInfo } from "../models/Entry";
import { derived, get, writable } from "svelte/store";
import { calculate_entry_info } from "../lib/calc_info/calc_info";
import { hour_range, price_hour } from "./Configs";
import { invoke } from "@tauri-apps/api/core";
import { checkpoints, last_checkpoint } from "./Checkpoints";

type EntryDataTransfer = Omit<Entry, "started_at" | "exited_at"> & { started_at: string, exited_at: string; };

const load_entries_from_database = async () => {
    const entries: EntryDataTransfer[] = await invoke(
        "get_entries", { enteredAt: get(last_checkpoint).checkpoint });
    return entries.map((entry) => ({
        register: entry.register,
        started_at: DateTime.fromISO(entry.started_at),
        exited_at: DateTime.fromISO(entry.exited_at)
    })).sort((a, b) => a.started_at.toMillis() - b.started_at.toMillis()) as Entry[];
};
const create_entries_store = async () => {
    const { subscribe, set, update } = writable<Entry[]>((
        await load_entries_from_database()
    ));

    return {
        subscribe,
        set,
        update,
        add_entry: async (new_entry: Omit<Entry, "register">) => {
            const result: EntryDataTransfer = await invoke("insert_entry", { entry: new_entry });
            const parsed_entry: Entry = {
                register: result.register,
                started_at: DateTime.fromISO(result.started_at),
                exited_at: DateTime.fromISO(result.exited_at)
            };
            update((old_entries) => [
                ...old_entries,
                parsed_entry
            ].sort((a, b) => a.started_at.toMillis() - b.started_at.toMillis()));
        },
        remove_entry: async (index: number) => {
            await invoke("delete_entry", { id: index });
            update((old_entries) => old_entries.filter((e) => e.register != index));
        },
        update_entry: async (index: number, new_entry: Omit<Entry, "register">) => {
            const result: EntryDataTransfer = await invoke("update_entry", { id: index, entry: new_entry });
            const parsed_entry: Entry = {
                register: result.register,
                started_at: DateTime.fromISO(result.started_at),
                exited_at: DateTime.fromISO(result.exited_at)
            };
            update((old_entries) => old_entries.map(
                (entry) => entry.register != index ? entry : parsed_entry
            ).sort((a, b) => a.started_at.toMillis() - b.started_at.toMillis()));
        },
        reload: async () => {
            set(await load_entries_from_database());
        }
    };
};

export const entries = await create_entries_store();

export const summary = derived([entries, price_hour, hour_range], ([$entries, $price_hour, $hour_range]) => {
    return $entries.map((e) => calculate_entry_info(e, $price_hour, $hour_range))
        .reduce<EntryInfo>((acc, cur) => {
            return {
                normal: acc.normal + cur.normal,
                extra: {
                    extra_50: acc.extra.extra_50 + cur.extra.extra_50,
                    extra_100: acc.extra.extra_100 + cur.extra.extra_100,
                },
                valor_extra: acc.valor_extra + cur.valor_extra,
                valor_normal: acc.valor_normal + cur.valor_normal,
            };
        }, {
            normal: 0,
            extra: {
                extra_50: 0,
                extra_100: 0,
            },
            valor_extra: 0,
            valor_normal: 0,
        });
});

const load_historical_entries_from_database = async (last_checkpoint: DateTime) => {
    const historical_entries: EntryDataTransfer[] = await invoke(
        "get_historical_entries", { checkpoint: last_checkpoint });

    return historical_entries.map((entry) => ({
        register: entry.register,
        started_at: DateTime.fromISO(entry.started_at),
        exited_at: DateTime.fromISO(entry.exited_at)
    })).sort((a, b) => a.started_at.toMillis() - b.started_at.toMillis()) as Entry[];
};

const historical_entries = derived([last_checkpoint], async ([$last_checkpoint]) => {
    return await load_historical_entries_from_database($last_checkpoint.checkpoint);
});

export const historical_entries_summaries = derived(
    [historical_entries, checkpoints],
    async ([$historical_entries, $checkpoints]) => {
        const hist_entries = await $historical_entries;
        if ($checkpoints.length < 2) { return []; }

        const summaries = $checkpoints.slice(0, -1).map(
            (e, i) => {
                const next_checkpoint = $checkpoints.at(i + 1) ?? e;
                return {
                    start: e,
                    end: next_checkpoint,
                    price_hour: next_checkpoint.price_hour,
                    hour_range: next_checkpoint.hour_range,
                    infos: hist_entries
                        .filter(
                            (entry) => {
                                return entry.started_at >= e.checkpoint &&
                                    entry.started_at <= ($checkpoints.at(i + 1) ?? e).checkpoint;
                            })
                        .map((e) => calculate_entry_info(
                            e, next_checkpoint.price_hour, next_checkpoint.hour_range))
                };
            })
            .map((e) => {
                return e.infos
                    .reduce<HistoricalEntryInfo>((acc, cur) => {
                        return {
                            ...acc, info: {
                                normal: acc.info.normal + cur.normal,
                                extra: {
                                    extra_50: acc.info.extra.extra_50 + cur.extra.extra_50,
                                    extra_100: acc.info.extra.extra_100 + cur.extra.extra_100,
                                },
                                valor_extra: acc.info.valor_extra + cur.valor_extra,
                                valor_normal: acc.info.valor_normal + cur.valor_normal,
                            }
                        };
                    }, {
                        price_hour: e.price_hour, hour_range: e.hour_range,
                        start: e.start, end: e.end, info: {
                            normal: 0,
                            extra: {
                                extra_50: 0,
                                extra_100: 0,
                            },
                            valor_extra: 0,
                            valor_normal: 0,
                        }
                    });
            });
        return summaries.sort((a, b) => b.end.checkpoint.toMillis() - a.end.checkpoint.toMillis());

    });