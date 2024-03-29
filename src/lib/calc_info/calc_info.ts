import { DateTime, type WeekdayNumbers } from "luxon";
import type { HourRange } from "../../models/Config";
import type { Entry, EntryInfo } from "../../models/Entry";
import Holidays from "date-holidays";
import { get } from "svelte/store";
import { state } from "../../stores/Configs";

type ActiveEntry = Omit<Entry, "register">;

function get_entries_per_day(entry: ActiveEntry, hour_range: HourRange) {
    const start = entry.started_at;
    const end = entry.exited_at;
    let new_entry_early: ActiveEntry = entry;
    let info_next_days: Omit<EntryInfo, "valor_normal" | "valor_extra"> = {
        extra: { extra_100: 0, extra_50: 0 },
        normal: 0
    };
    if (start.day != end.day) {
        new_entry_early = {
            started_at: start,
            exited_at: DateTime.fromObject(
                { year: start.year, month: start.month, day: start.day, hour: 23, minute: 59, second: 59 },
                { zone: "America/Sao_Paulo" })
        };
        const new_entry_late: ActiveEntry = {
            started_at: DateTime.fromObject(
                { year: start.year, month: start.month, day: start.day, hour: 24, minute: 0, second: 0 },
                { zone: "America/Sao_Paulo" }),
            exited_at: end,
        };
        info_next_days = get_entries_per_day(new_entry_late, hour_range);
    }
    const [hour_start, , minute_start] = hour_range.start.split(":").map(parseInt);
    const [hour_end, , minute_end] = hour_range.end.split(":").map(parseInt);

    const normal_time_start = DateTime.fromObject(
        {
            year: start.year,
            month: start.month,
            day: start.day,
            hour: hour_start,
            minute: minute_start,
            second: 0
        },
        { zone: "America/Sao_Paulo" });
    const normal_time_end = DateTime.fromObject(
        {
            year: start.year,
            month: start.month,
            day: start.day,
            hour: hour_end,
            minute: minute_end,
            second: 0
        },
        { zone: "America/Sao_Paulo" });
    const diff_normal = normal_time_end.diff(normal_time_start);
    // diff_normal.as("minutes") / 60 horas normais

    const diff_extra_early = normal_time_start.diff(new_entry_early.started_at);
    // diff_extra_early.as("minutes") / 60 horas extra early

    const diff_extra_late = new_entry_early.exited_at.diff(normal_time_end);
    // diff_extra_early.as("minutes") / 60 horas extra after normal
    let normal = diff_normal.as("hours");
    let extra_early = diff_extra_early.as("hours");
    let extra_late = diff_extra_late.as("hours");
    if (extra_early < 0) {
        normal += extra_early;
        extra_early = 0;
    }
    if (extra_late < 0) {
        normal += extra_late;
        extra_late = 0;
    }
    const extra = extra_early + extra_late;

    const luxon_sunday_number: WeekdayNumbers = 7;
    const luxon_saturday_number: WeekdayNumbers = 6;
    let extra_50 = extra;
    let extra_100 = 0;
    if (normal_time_start.weekday == luxon_sunday_number || is_brazilian_holiday(normal_time_start)) {
        extra_100 = extra + normal;
        extra_50 = 0;
        normal = 0;
    } else if (normal_time_start.weekday == luxon_saturday_number) {
        extra_50 = extra + normal;
        normal = 0;
    }

    const info: Omit<EntryInfo, "valor_normal" | "valor_extra"> = {
        normal: normal + info_next_days.normal,
        extra: {
            extra_50: extra_50 + info_next_days.extra.extra_50,
            extra_100: extra_100 + info_next_days.extra.extra_100,
        }
    };
    return info;
}

export function calculate_entry_info(
    entry: ActiveEntry, price_hour: number, hour_range: HourRange): EntryInfo {
    const info = get_entries_per_day(entry, hour_range);

    const hours_normal = info.normal;
    const hours_extra = info.extra;

    const valor_normal = hours_normal * price_hour;
    const valor_extra = hours_extra.extra_50 * price_hour * 1.5 + hours_extra.extra_100 * price_hour * 2;
    return {
        normal: hours_normal,
        extra: hours_extra,
        valor_extra: valor_extra,
        valor_normal: valor_normal
    };
}

function is_brazilian_holiday(normal_time_start: DateTime): boolean {
    const hd = new Holidays("BR", get(state));
    const is_holiday = hd.isHoliday(normal_time_start.toJSDate());
    if (is_holiday !== false) {
        return is_holiday[0].type === "public";
    }
    return false;
}


