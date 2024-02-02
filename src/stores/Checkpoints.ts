import { DateTime } from "luxon";
import { derived, writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import type { Checkpoint } from "../models/Checkpoint";

type CheckpointDataTransfer = Omit<Checkpoint, "checkpoint" | "hour_range"> & {
    checkpoint: string;
    start_time: string,
    end_time: string;
};

const load_checkpoint_from_database = async () => {
    const checkpoints: CheckpointDataTransfer[] = await invoke(
        "get_checkpoints");
    const checkpoints_mapped = checkpoints.map(
        e => { return { checkpoint: DateTime.fromISO(e.checkpoint) }; }).sort(
            (a, b) => a.checkpoint.toMillis() - b.checkpoint.toMillis()
        ) as Checkpoint[];
    return [
        { id: -1, checkpoint: DateTime.fromMillis(0), price_hour: 0, hour_range: { start: "", end: "" } },
        ...checkpoints_mapped];
};
const create_checkpoint_store = async () => {
    const { subscribe, set, update } = writable<Checkpoint[]>((
        await load_checkpoint_from_database()
    ));

    return {
        subscribe,
        set,
        update,
        add_checkpoint: async (new_checkpoint: Omit<Checkpoint, "id">) => {
            const result: CheckpointDataTransfer = await invoke(
                "insert_checkpoint", {
                    checkpointData: {
                        ...new_checkpoint,
                        start_time: new_checkpoint.hour_range.start, 
                        end_time: new_checkpoint.hour_range.end,
                    }
            });
            const parsed_checkpoints: Checkpoint = {
                id: result.id,
                checkpoint: DateTime.fromISO(result.checkpoint),
                price_hour: result.price_hour,
                hour_range: {
                    start: result.start_time,
                    end: result.end_time
                }
            };
            update((old_checkpoints) => [
                ...old_checkpoints,
                parsed_checkpoints
            ].sort((a, b) => a.checkpoint.toMillis() - b.checkpoint.toMillis()));
        },
        remove_checkpoint: async (checkpoint: Checkpoint) => {
            await invoke("delete_checkpoint", { checkpoint: checkpoint });
            update((old_checkpoints) => old_checkpoints.filter((e) => e.checkpoint != checkpoint.checkpoint));
        },
        update_checkpoint: async (old_checkpoint: Checkpoint, new_checkpoint: Omit<Checkpoint, "id">) => {
            const result: CheckpointDataTransfer = await invoke(
                "update_checkpoint",
                { old_checkpoint: old_checkpoint, new_checkpoint: new_checkpoint });
            const parsed_checkpoint: Checkpoint = {
                id: result.id,
                price_hour: result.price_hour,
                hour_range: {
                    start: result.start_time,
                    end: result.end_time
                },
                checkpoint: DateTime.fromISO(result.checkpoint),
            };
            update((old_checkpoints) => old_checkpoints.map(
                (checkpoint) => checkpoint.checkpoint != old_checkpoint.checkpoint ?
                    checkpoint : parsed_checkpoint
            ).sort((a, b) => a.checkpoint.toMillis() - b.checkpoint.toMillis()));
        },
    };
};

export const checkpoints = await create_checkpoint_store();

export const last_checkpoint = derived(checkpoints, ($checkpoints) => {
    if ($checkpoints.length == 0) {
        return { checkpoint: DateTime.fromMillis(0) } as Checkpoint;
    }
    return $checkpoints[$checkpoints.length - 1];
});
