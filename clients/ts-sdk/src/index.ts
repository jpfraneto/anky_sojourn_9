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

/**
 * V1 mirrors the current onchain field name.
 * Under the soulbound season model, this should be the wallet's single claimed vessel.
 */
export interface DailySealRecord {
  selectedVesselAssetId: string;
  utcDay: number;
  chamberIndex: number;
  currentSteward: string;
  ankyHashCommitmentHex: string;
  sealedAtUnixTs: number;
  rewardEligible: boolean;
}

export interface VesselCollection {
  verified: boolean;
  key: string;
}

export type SelectedVesselCollection = VesselCollection;

export interface VesselCreator {
  address: string;
  verified: boolean;
  share: number;
}

export type SelectedVesselCreator = VesselCreator;

export interface VesselMetadata {
  name: string;
  symbol: string;
  uri: string;
  sellerFeeBasisPoints: number;
  primarySaleHappened: boolean;
  isMutable: boolean;
  editionNonce: number | null;
  tokenStandard:
    | "NonFungible"
    | "FungibleAsset"
    | "Fungible"
    | "NonFungibleEdition"
    | null;
  collection: VesselCollection | null;
  tokenProgramVersion: "Original" | "Token2022";
  creators: VesselCreator[];
}

export type SelectedVesselMetadata = VesselMetadata;

export interface VesselProof {
  merkleRootHex: string;
  nonce: number;
  index: number;
  owner: string;
  delegate: string;
  metadata: VesselMetadata;
}

export type SelectedVesselProof = VesselProof;

export interface SealAnkyV1Args {
  /**
   * Transitional V1 argument name retained for compatibility with the current instruction shape.
   */
  selectedVesselAssetId: string;
  ankyHashCommitmentHex: string;
  selectedVesselProof: VesselProof;
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
