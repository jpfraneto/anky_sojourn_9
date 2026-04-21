use anchor_lang::prelude::*;

use crate::{errors::Sojourn9Error, state::DailySeal};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct ClaimRewardArgs {}

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub claimant: Signer<'info>,
    pub daily_seal: Account<'info, DailySeal>,
    /// CHECK: This becomes the reward-claim receipt PDA once the settlement surface is finalized.
    #[account(mut)]
    pub reward_claim: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<ClaimReward>, _args: ClaimRewardArgs) -> Result<()> {
    /*
    TODO: first constitutional implementation pass

    - define what reward asset or accounting primitive is being claimed
    - require the referenced DailySeal to be reward-eligible
    - bind settlement to the steward who earned the reward at seal time
    - create or validate a claim receipt account so claims cannot be replayed
    - make sure later vessel transfers do not move already-earned rewards
    - emit RewardClaimed when reward settlement is recorded
    */
    err!(Sojourn9Error::UnimplementedInstruction)
}
