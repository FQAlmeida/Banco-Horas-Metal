import { DateTime } from "luxon";
import type { Entry, EntryInfo } from "../models/Entry";
import { derived, get, writable } from "svelte/store";
import { calculate_entry_info } from "../lib/calc_info/calc_info";
import { hour_range, price_hour } from "./Configs";
import { invoke } from "@tauri-apps/api/core";
import { last_checkpoint } from "./Checkpoints";

type EntryDataTransfer = Omit<Entry, "started_at" | "exited_at"> & { started_at: string, exited_at: string; };

const load_entries_from_database = async () => {
    const entries: EntryDataTransfer[] = await invoke(
        "get_entries", { enteredAt: get(last_checkpoint).checkpoint });
    return entries.sort((a, b) => a.register - b.register).map((entry) => ({
        register: entry.register,
        started_at: DateTime.fromISO(entry.started_at),
        exited_at: DateTime.fromISO(entry.exited_at)
    })) as Entry[];
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
            ].sort((a, b) => a.register - b.register));
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
            ).sort((a, b) => a.register - b.register));
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