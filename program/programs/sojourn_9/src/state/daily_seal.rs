use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct DailySeal {
    pub bump: u8,
    pub season_config: Pubkey,
    /// Transitional V1 field name retained until `claim_vessel` can collapse sealing to one claimed vessel per wallet.
    pub selected_vessel_asset_id: Pubkey,
    pub utc_day: i64,
    pub chamber_index: u16,
    pub current_steward: Pubkey,
    pub anky_hash_commitment: [u8; 32],
    pub sealed_at_unix_ts: i64,
    pub reward_eligible: bool,
}
