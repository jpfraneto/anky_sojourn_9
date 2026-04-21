#![allow(deprecated)]
#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;
pub mod utils;

pub use instructions::*;
pub(crate) use instructions::{
    __client_accounts_claim_reward, __client_accounts_initialize_season,
    __client_accounts_seal_anky,
};

declare_id!("2VfB7nvV2SZuCpK2DurRgJLfw57TCt2g9VJXACo5h8aK");

#[program]
pub mod sojourn_9 {
    use super::*;

    pub fn initialize_season(
        ctx: Context<InitializeSeason>,
        args: InitializeSeasonArgs,
    ) -> Result<()> {
        instructions::initialize_season::handler(ctx, args)
    }

    pub fn seal_anky(ctx: Context<SealAnky>, args: SealAnkyArgs) -> Result<()> {
        instructions::seal_anky::handler(ctx, args)
    }

    pub fn claim_reward(ctx: Context<ClaimReward>, args: ClaimRewardArgs) -> Result<()> {
        instructions::claim_reward::handler(ctx, args)
    }
}
