import type { DateTime } from "luxon";
import type { Checkpoint } from "./Checkpoint";

export interface Entry {
    register: number,
    started_at: DateTime,
    exited_at: DateTime;
}

export interface EntryInfo {
    normal: number,
    extra: { extra_50: number, extra_100: number; },
    valor_normal: number,
    valor_extra: number,
}

export interface HistoricalEntryInfo {
    start: Checkpoint,
    end: Checkpoint,
    info: EntryInfo,
}
