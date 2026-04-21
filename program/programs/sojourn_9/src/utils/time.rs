use anchor_lang::prelude::*;

use crate::{
    constants::{CHAMBER_COUNT, CHAMBERS_PER_REGION, UTC_DAY_SECONDS},
    errors::Sojourn9Error,
};

pub fn derive_utc_day(unix_timestamp: i64) -> Result<i64> {
    require!(unix_timestamp >= 0, Sojourn9Error::TimestampBeforeUnixEpoch);

    Ok(unix_timestamp / UTC_DAY_SECONDS)
}

pub fn is_utc_day_boundary(unix_timestamp: i64) -> bool {
    unix_timestamp.rem_euclid(UTC_DAY_SECONDS) == 0
}

pub fn derive_chamber_index(season_start_unix_ts: i64, unix_timestamp: i64) -> Result<u16> {
    let season_start_utc_day = derive_utc_day(season_start_unix_ts)?;
    let utc_day = derive_utc_day(unix_timestamp)?;
    let chamber_index = utc_day - season_start_utc_day;

    require!(
        chamber_index >= 0,
        Sojourn9Error::TimestampBeforeSeasonStart
    );
    require!(
        chamber_index < CHAMBER_COUNT as i64,
        Sojourn9Error::TimestampOutsideSeasonWindow
    );

    Ok(chamber_index as u16)
}

pub fn derive_region_index(chamber_index: u16) -> Result<u16> {
    require!(
        chamber_index < CHAMBER_COUNT,
        Sojourn9Error::InvalidChamberIndex
    );

    Ok(chamber_index / CHAMBERS_PER_REGION)
}

#[cfg(test)]
mod tests {
    use super::{derive_chamber_index, derive_region_index, derive_utc_day, is_utc_day_boundary};

    #[test]
    fn derives_first_day() {
        assert_eq!(derive_utc_day(0).unwrap(), 0);
        assert_eq!(derive_utc_day(86_399).unwrap(), 0);
    }

    #[test]
    fn derives_second_day_boundary() {
        assert_eq!(derive_utc_day(86_400).unwrap(), 1);
        assert_eq!(derive_utc_day(172_800).unwrap(), 2);
    }

    #[test]
    fn rejects_negative_timestamps() {
        assert!(derive_utc_day(-1).is_err());
    }

    #[test]
    fn recognizes_utc_day_boundaries() {
        assert!(is_utc_day_boundary(0));
        assert!(is_utc_day_boundary(86_400));
        assert!(!is_utc_day_boundary(1));
        assert!(!is_utc_day_boundary(86_399));
    }

    #[test]
    fn derives_chamber_index_inside_window() {
        assert_eq!(derive_chamber_index(0, 0).unwrap(), 0);
        assert_eq!(derive_chamber_index(0, 86_400).unwrap(), 1);
        assert_eq!(derive_chamber_index(0, 95 * 86_400).unwrap(), 95);
    }

    #[test]
    fn rejects_timestamp_before_season_start() {
        assert!(derive_chamber_index(86_400, 0).is_err());
    }

    #[test]
    fn rejects_timestamp_after_season_window() {
        assert!(derive_chamber_index(0, 96 * 86_400).is_err());
    }

    #[test]
    fn derives_region_index_from_chamber_index() {
        assert_eq!(derive_region_index(0).unwrap(), 0);
        assert_eq!(derive_region_index(7).unwrap(), 0);
        assert_eq!(derive_region_index(8).unwrap(), 1);
        assert_eq!(derive_region_index(95).unwrap(), 11);
        assert!(derive_region_index(96).is_err());
    }
}
