use anchor_lang::prelude::*;

use crate::constants::DAILY_SEAL_SEED;

pub fn season_number_seed_bytes(season_number: u16) -> [u8; 2] {
    season_number.to_le_bytes()
}

pub fn utc_day_seed_bytes(utc_day: i64) -> [u8; 8] {
    utc_day.to_le_bytes()
}

/// V1 still keys daily seals by `selected_vessel_asset_id`.
/// A later claim-state pass may simplify the input surface, but the one-seal-per-vessel-per-day law stays the same.
pub fn find_daily_seal_pda(
    season_config: &Pubkey,
    selected_vessel_asset_id: &Pubkey,
    utc_day: i64,
) -> (Pubkey, u8) {
    let utc_day_seed = utc_day_seed_bytes(utc_day);

    Pubkey::find_program_address(
        &[
            DAILY_SEAL_SEED,
            season_config.as_ref(),
            selected_vessel_asset_id.as_ref(),
            &utc_day_seed,
        ],
        &crate::id(),
    )
}
