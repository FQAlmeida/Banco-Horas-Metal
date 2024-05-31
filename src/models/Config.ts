export interface HourRange {
    start: string,
    end: string;
}

export interface ConfigData {
    id: number,
    hour_range: HourRange,
    price_hour: number;
    state: string;
}