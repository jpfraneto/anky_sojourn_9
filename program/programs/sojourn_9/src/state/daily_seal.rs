use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct DailySeal {
    pub bump: u8,
    pub season_config: Pubkey,
    pub vessel_asset_id: Pubkey,
    pub utc_day: i64,
    pub chamber_index: u16,
    pub steward: Pubkey,
    pub sealed_at_unix_ts: i64,
    pub reward_eligible: bool,
}
