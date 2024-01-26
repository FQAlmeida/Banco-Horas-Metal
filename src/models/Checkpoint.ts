import type { DateTime } from "luxon";

export interface Checkpoint {
    id: number;
    checkpoint: DateTime;
}
