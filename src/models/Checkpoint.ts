import type { DateTime } from "luxon";
import type { HourRange } from "./Config";

export interface Checkpoint {
    id: number;
    checkpoint: DateTime;
    price_hour: number,
    hour_range: HourRange,
}
