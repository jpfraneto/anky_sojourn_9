pub fn season_number_seed_bytes(season_number: u16) -> [u8; 2] {
    season_number.to_le_bytes()
}

pub fn utc_day_seed_bytes(utc_day: i64) -> [u8; 8] {
    utc_day.to_le_bytes()
}
