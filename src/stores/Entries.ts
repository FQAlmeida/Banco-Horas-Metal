import { DateTime } from "luxon";
import type { Entry, EntryInfo } from "../models/Entry";
import { derived, writable } from "svelte/store";
import { calculate_entry_info } from "../lib/calc_info/calc_info";
import { hour_range, price_hour } from "./Configs";

const load_entries_from_database = async () => {
    return await new Promise<Entry[]>(resolve => {
        resolve(new Array(2)
            .fill(0)
            .map(() => {
                return {
                    started_at: DateTime.fromObject({ hour: 7 })
                        .setZone("America/Sao_Paulo")
                        .minus({
                            weeks: 2,
                            hours: Math.round(Math.random() * 4) - 1,
                        }),
                    exited_at: DateTime.fromObject({ hour: 17 })
                        .setZone("America/Sao_Paulo")
                        .minus({
                            weeks: 2,
                            hours: Math.round(Math.random() * 4) - 4,
                        }),
                };
            }));
    });
};
const create_entries_store = async () => {
    const { subscribe, set, update } = writable<Entry[]>((
        await load_entries_from_database()
    ).sort((a, b) => a.started_at.toMillis() - b.started_at.toMillis()));

    return {
        subscribe,
        set,
        update,
        add_entry: (new_entry: Entry) => {
            update((old_entries) => [
                ...old_entries,
                new_entry
            ].sort((a, b) => a.started_at.toMillis() - b.started_at.toMillis()));
        },
        remove_entry: (index: number) => {
            update((old_entries) => old_entries.filter((_, i) => i != index));
        },
        update_entry: (index: number, new_entry: Entry) => {
            update((old_entries) => old_entries.map((entry, i) => i != index ? entry : new_entry));
        },
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