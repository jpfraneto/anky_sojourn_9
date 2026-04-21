use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct RewardClaim {
    pub bump: u8,
    pub daily_seal: Pubkey,
    pub claimant: Pubkey,
    pub claimed_at_unix_ts: i64,
}
