use chrono::{NaiveTime, Weekday, Timelike};

pub const HOUR_START: u32 = 8;
pub const HOUR_END: u32 = 20;
pub const SLOT_MINUTES: u32 = 30;

/// Colonna CSS grid per un giorno (1 = colonna etichette orarie, 2 = Lunedì, ..., 6 = Venerdì)
pub fn weekday_to_column(w: Weekday) -> u32 {
    w.num_days_from_monday() + 2 // +2 to account for the first column (time labels)
}

/// Riga CSS grid di partenza per un orario dato (riga 1 = header, riga 2 = prima fascia oraria HOUR_START)
pub fn time_to_row(t: NaiveTime) -> u32 {
    let total_minutes = t.hour() * 60 + t.minute();
    let start_minutes = HOUR_START * 60;
    let minutes_since_start = total_minutes.saturating_sub(start_minutes);
    let row_offset = minutes_since_start / SLOT_MINUTES;
    row_offset + 2
}