export const PROGRAM_NAME = "sojourn_9";
export const SOJOURN_NUMBER = 9;
export const CHAMBER_COUNT = 96;
export const REGION_COUNT = 12;
export const CHAMBERS_PER_REGION = 8;
export const SOJOURN_DAYS = CHAMBER_COUNT;
export const CANONICAL_VESSEL_COUNT = 3_456;
export const UTC_DAY_SECONDS = 86_400;

export interface SeasonConfigSnapshot {
  authority: string;
  seasonNumber: number;
  seasonStartUnixTs: number;
  chamberCount: number;
  regionCount: number;
  chambersPerRegion: number;
  canonicalVesselCount: number;
  vesselCollection: string;
  constitutionRevision: number;
  constitutionHashHex: string;
}

export interface DailySealRecord {
  vesselAssetId: string;
  utcDay: number;
  chamberIndex: number;
  steward: string;
  sealedAtUnixTs: number;
  rewardEligible: boolean;
}

export function deriveUtcDay(unixTimestamp: number): number {
  if (!Number.isInteger(unixTimestamp)) {
    throw new Error("unixTimestamp must be an integer number of seconds");
  }

  return Math.floor(unixTimestamp / UTC_DAY_SECONDS);
}

export function deriveChamberIndex(
  seasonStartUnixTs: number,
  unixTimestamp: number,
): number {
  const seasonStartUtcDay = deriveUtcDay(seasonStartUnixTs);
  const utcDay = deriveUtcDay(unixTimestamp);
  const chamberIndex = utcDay - seasonStartUtcDay;

  if (chamberIndex < 0 || chamberIndex >= CHAMBER_COUNT) {
    throw new Error(
      "unixTimestamp must resolve to a chamber inside the 96-chamber Sojourn 9 window",
    );
  }

  return chamberIndex;
}

export function deriveRegionIndex(chamberIndex: number): number {
  if (!Number.isInteger(chamberIndex)) {
    throw new Error("chamberIndex must be an integer");
  }

  if (chamberIndex < 0 || chamberIndex >= CHAMBER_COUNT) {
    throw new Error("chamberIndex must be inside the 96-chamber Sojourn 9 path");
  }

  return Math.floor(chamberIndex / CHAMBERS_PER_REGION);
}
