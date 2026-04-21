use anchor_lang::prelude::*;

use crate::{
    constants::{
        CANONICAL_VESSEL_COUNT, CHAMBER_COUNT, CHAMBERS_PER_REGION, REGION_COUNT, SEASON_SEED,
        SOJOURN_NUMBER, UTC_DAY_SECONDS,
    },
    errors::Sojourn9Error,
    events::SeasonInitialized,
    state::SeasonConfig,
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct InitializeSeasonArgs {
    pub season_number: u16,
    pub season_start_unix_ts: i64,
    pub canonical_vessel_count: u16,
    pub vessel_collection: Pubkey,
    pub constitution_revision: u32,
    pub constitution_hash: [u8; 32],
}

#[derive(Accounts)]
#[instruction(args: InitializeSeasonArgs)]
pub struct InitializeSeason<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + SeasonConfig::INIT_SPACE,
        seeds = [SEASON_SEED, &args.season_number.to_le_bytes()],
        bump,
    )]
    pub season_config: Account<'info, SeasonConfig>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeSeason>, args: InitializeSeasonArgs) -> Result<()> {
    require!(
        args.season_number == SOJOURN_NUMBER,
        Sojourn9Error::InvalidSeasonNumber
    );
    require!(
        args.season_start_unix_ts.rem_euclid(UTC_DAY_SECONDS) == 0,
        Sojourn9Error::SeasonStartNotOnUtcBoundary
    );
    require!(
        args.canonical_vessel_count == CANONICAL_VESSEL_COUNT,
        Sojourn9Error::InvalidCanonicalVesselCount
    );

    let season_config = &mut ctx.accounts.season_config;
    season_config.bump = ctx.bumps.season_config;
    season_config.authority = ctx.accounts.authority.key();
    season_config.season_number = args.season_number;
    season_config.season_start_unix_ts = args.season_start_unix_ts;
    season_config.chamber_count = CHAMBER_COUNT;
    season_config.region_count = REGION_COUNT;
    season_config.chambers_per_region = CHAMBERS_PER_REGION;
    season_config.canonical_vessel_count = args.canonical_vessel_count;
    season_config.vessel_collection = args.vessel_collection;
    season_config.constitution_revision = args.constitution_revision;
    season_config.constitution_hash = args.constitution_hash;

    emit!(SeasonInitialized {
        season_config: season_config.key(),
        authority: season_config.authority,
        season_number: season_config.season_number,
        season_start_unix_ts: season_config.season_start_unix_ts,
        chamber_count: season_config.chamber_count,
        region_count: season_config.region_count,
        chambers_per_region: season_config.chambers_per_region,
        vessel_collection: season_config.vessel_collection,
        constitution_revision: season_config.constitution_revision,
    });

    Ok(())
}
