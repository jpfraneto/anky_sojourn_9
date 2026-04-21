use anchor_lang::prelude::*;

use crate::{errors::Sojourn9Error, state::SeasonConfig};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SealAnkyArgs {
    pub vessel_asset_id: Pubkey,
}

#[derive(Accounts)]
pub struct SealAnky<'info> {
    #[account(mut)]
    pub steward: Signer<'info>,
    pub season_config: Account<'info, SeasonConfig>,
    /// CHECK: This will become the vessel-day seal PDA once the public proof layout is finalized.
    #[account(mut)]
    pub daily_seal: UncheckedAccount<'info>,
    /// CHECK: This will be constrained to the cNFT asset or proof path used for stewardship verification.
    pub vessel_asset: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<SealAnky>, _args: SealAnkyArgs) -> Result<()> {
    /*
    TODO: first constitutional implementation pass

    - derive utc_day from Clock::get()?.unix_timestamp
    - derive the current chamber_index from the configured season start
    - validate the day is inside the configured 96-chamber season window
    - validate only the current chamber can be sealed
    - validate the vessel belongs to the season's canonical collection or registry
    - validate the current steward owns or validly controls the vessel at seal time
    - initialize a vessel-day record so transfer cannot create a second seal that day
    - mark reward eligibility on the resulting record
    - emit AnkySealed for indexers and downstream reward systems

    Private writing content and the returned `.anky` stay offchain. The record should capture
    only minimal public proof.
    */
    err!(Sojourn9Error::UnimplementedInstruction)
}
