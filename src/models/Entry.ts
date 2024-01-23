import type { DateTime } from "luxon";

export interface Entry {
    started_at: DateTime,
    exited_at: DateTime
}

export interface EntryInfo {
    normal: number,
    extra: { extra_50: number, extra_100: number },
    valor_normal: number,
    valor_extra: number,
}
