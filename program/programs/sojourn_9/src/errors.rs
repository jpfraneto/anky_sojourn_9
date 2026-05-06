use anchor_lang::prelude::*;

#[error_code]
pub enum Sojourn9Error {
    #[msg("This instruction surface is scaffolded but not implemented yet.")]
    UnimplementedInstruction,
    #[msg("The season number does not match the canonical Sojourn 9 constant.")]
    InvalidSeasonNumber,
    #[msg("The season start must land on a UTC day boundary.")]
    SeasonStartNotOnUtcBoundary,
    #[msg("The canonical vessel count does not match the constitutional constant.")]
    InvalidCanonicalVesselCount,
    #[msg("UTC day derivation requires a non-negative unix timestamp.")]
    TimestampBeforeUnixEpoch,
    #[msg("The timestamp resolves to a day before the season opens.")]
    TimestampBeforeSeasonStart,
    #[msg("The timestamp resolves outside the 96-chamber Sojourn 9 window.")]
    TimestampOutsideSeasonWindow,
    #[msg("The chamber index falls outside the 96-chamber Sojourn 9 path.")]
    InvalidChamberIndex,
    #[msg("The season exists but is not active for the current canonical UTC day.")]
    InactiveSeason,
    #[msg("The provided vessel proof does not resolve to a valid Sojourn 9 vessel.")]
    InvalidVessel,
    #[msg("The signer is not the current steward of the proved valid vessel.")]
    InvalidSteward,
    #[msg("The proved vessel has already sealed for this UTC day.")]
    AlreadySealedToday,
    #[msg("The provided accounts do not match the V1 seal account boundary.")]
    InvalidAccounts,
}
