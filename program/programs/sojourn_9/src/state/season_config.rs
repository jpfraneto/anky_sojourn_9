use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct SeasonConfig {
    pub bump: u8,
    pub authority: Pubkey,
    pub season_number: u16,
    pub season_start_unix_ts: i64,
    pub chamber_count: u16,
    pub region_count: u16,
    pub chambers_per_region: u16,
    pub canonical_vessel_count: u16,
    pub vessel_collection: Pubkey,
    pub constitution_revision: u32,
    pub constitution_hash: [u8; 32],
}
