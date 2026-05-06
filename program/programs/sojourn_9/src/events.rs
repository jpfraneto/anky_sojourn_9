use anchor_lang::prelude::*;

#[event]
pub struct SeasonInitialized {
    pub season_config: Pubkey,
    pub authority: Pubkey,
    pub season_number: u16,
    pub season_start_unix_ts: i64,
    pub chamber_count: u16,
    pub region_count: u16,
    pub chambers_per_region: u16,
    pub vessel_collection: Pubkey,
    pub constitution_revision: u32,
}

#[event]
pub struct AnkySealed {
    pub daily_seal: Pubkey,
    pub season_config: Pubkey,
    /// Transitional V1 field name retained until claim state exists.
    pub selected_vessel_asset_id: Pubkey,
    pub utc_day: i64,
    pub chamber_index: u16,
    pub current_steward: Pubkey,
    pub anky_hash_commitment: [u8; 32],
    pub reward_eligible: bool,
}

#[event]
pub struct RewardClaimed {
    pub daily_seal: Pubkey,
    pub claimant: Pubkey,
}
