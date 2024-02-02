export interface HourRange {
    start: string,
    end: string;
}

export interface Config {
    id: number,
    hour_range: HourRange,
    price_hour: number;
}