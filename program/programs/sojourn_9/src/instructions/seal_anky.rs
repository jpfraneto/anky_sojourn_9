use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke_signed, system_instruction},
    system_program,
};
use mpl_bubblegum::{
    accounts::TreeConfig as BubblegumTreeConfig,
    hash::{hash_creators, hash_metadata},
    types::{
        Collection as BubblegumCollection, Creator as BubblegumCreator, LeafSchema,
        MetadataArgs as BubblegumMetadataArgs, TokenProgramVersion as BubblegumTokenProgramVersion,
        TokenStandard as BubblegumTokenStandard, UseMethod as BubblegumUseMethod,
        Uses as BubblegumUses,
    },
    utils::get_asset_id,
    ID as MPL_BUBBLEGUM_ID,
};
use spl_concurrent_merkle_tree::{concurrent_merkle_tree::ProveLeafArgs, node::Node};

use crate::{
    errors::Sojourn9Error,
    events::AnkySealed,
    state::{DailySeal, SeasonConfig},
    utils::{
        compression::{
            fill_in_proof_from_canopy, merkle_tree_get_size, merkle_tree_prove_leaf,
            ConcurrentMerkleTreeHeader, CONCURRENT_MERKLE_TREE_HEADER_SIZE_V1,
            SPL_ACCOUNT_COMPRESSION_ID,
        },
        pda::{find_daily_seal_pda, utc_day_seed_bytes},
        time::{derive_chamber_index, derive_utc_day},
    },
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SealAnkyArgs {
    /// Transitional V1 argument name retained until `claim_vessel` exists and the seal path can derive
    /// the wallet's single claimed vessel more directly.
    pub selected_vessel_asset_id: Pubkey,
    pub anky_hash_commitment: [u8; 32],
    /// Transitional proof bundle used by V1 while the program remains vessel-keyed at seal time.
    pub selected_vessel_proof: SelectedVesselProof,
}

#[derive(Accounts)]
pub struct SealAnky<'info> {
    #[account(mut)]
    pub current_steward: Signer<'info>,
    pub season_config: Account<'info, SeasonConfig>,
    /// CHECK: V1 still derives and creates the vessel/day PDA in the handler because the canonical UTC
    /// day comes from Clock at execution time and claim state does not exist yet.
    #[account(mut)]
    pub daily_seal: UncheckedAccount<'info>,
    /// CHECK: V1 reads the concurrent merkle tree directly and verifies the provided vessel proof.
    /// This remains a transitional boundary until `claim_vessel` exists.
    pub selected_vessel_merkle_tree: UncheckedAccount<'info>,
    /// CHECK: V1 requires the Bubblegum tree-config PDA for the provided vessel merkle tree.
    pub selected_vessel_tree_config: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SealAnky>, args: SealAnkyArgs) -> Result<()> {
    let sealed_at_unix_ts = Clock::get()?.unix_timestamp;
    let utc_day = derive_utc_day(sealed_at_unix_ts)?;
    let chamber_index =
        derive_active_chamber_index(&ctx.accounts.season_config, sealed_at_unix_ts)?;

    verify_selected_vessel(
        &ctx.accounts.season_config,
        ctx.accounts.current_steward.key(),
        &ctx.accounts.selected_vessel_merkle_tree.to_account_info(),
        &ctx.accounts.selected_vessel_tree_config.to_account_info(),
        &args,
        ctx.remaining_accounts,
    )?;

    let bump = create_daily_seal_account(
        &ctx.accounts.current_steward.to_account_info(),
        &ctx.accounts.daily_seal.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
        &ctx.accounts.season_config.key(),
        &args.selected_vessel_asset_id,
        utc_day,
    )?;

    let daily_seal = DailySeal {
        bump,
        season_config: ctx.accounts.season_config.key(),
        selected_vessel_asset_id: args.selected_vessel_asset_id,
        utc_day,
        chamber_index,
        current_steward: ctx.accounts.current_steward.key(),
        anky_hash_commitment: args.anky_hash_commitment,
        sealed_at_unix_ts,
        reward_eligible: true,
    };

    daily_seal.try_serialize(&mut &mut ctx.accounts.daily_seal.data.borrow_mut()[..])?;

    emit!(AnkySealed {
        daily_seal: ctx.accounts.daily_seal.key(),
        season_config: daily_seal.season_config,
        selected_vessel_asset_id: daily_seal.selected_vessel_asset_id,
        utc_day: daily_seal.utc_day,
        chamber_index: daily_seal.chamber_index,
        current_steward: daily_seal.current_steward,
        anky_hash_commitment: daily_seal.anky_hash_commitment,
        reward_eligible: daily_seal.reward_eligible,
    });

    Ok(())
}

fn derive_active_chamber_index(
    season_config: &SeasonConfig,
    sealed_at_unix_ts: i64,
) -> Result<u16> {
    let before_start_code = u32::from(Sojourn9Error::TimestampBeforeSeasonStart);
    let outside_window_code = u32::from(Sojourn9Error::TimestampOutsideSeasonWindow);

    derive_chamber_index(season_config.season_start_unix_ts, sealed_at_unix_ts).map_err(|error| {
        match error {
            anchor_lang::error::Error::AnchorError(anchor_error)
                if matches!(
                    anchor_error.error_code_number,
                    x if x == before_start_code || x == outside_window_code
                ) =>
            {
                error!(Sojourn9Error::InactiveSeason)
            }
            other => other,
        }
    })
}

fn verify_selected_vessel(
    season_config: &SeasonConfig,
    current_steward: Pubkey,
    selected_vessel_merkle_tree: &AccountInfo,
    selected_vessel_tree_config: &AccountInfo,
    args: &SealAnkyArgs,
    proof_accounts: &[AccountInfo],
) -> Result<()> {
    require_keys_eq!(
        *selected_vessel_merkle_tree.owner,
        SPL_ACCOUNT_COMPRESSION_ID,
        Sojourn9Error::InvalidAccounts
    );
    require_keys_eq!(
        *selected_vessel_tree_config.owner,
        MPL_BUBBLEGUM_ID,
        Sojourn9Error::InvalidAccounts
    );

    let (expected_tree_config, _) =
        BubblegumTreeConfig::find_pda(&selected_vessel_merkle_tree.key());
    require_keys_eq!(
        selected_vessel_tree_config.key(),
        expected_tree_config,
        Sojourn9Error::InvalidAccounts
    );
    BubblegumTreeConfig::try_from(&selected_vessel_tree_config.to_account_info())
        .map_err(|_| error!(Sojourn9Error::InvalidAccounts))?;

    let metadata = args.selected_vessel_proof.metadata.to_bubblegum_metadata();
    let collection = metadata
        .collection
        .as_ref()
        .ok_or_else(|| error!(Sojourn9Error::InvalidVessel))?;
    require!(collection.verified, Sojourn9Error::InvalidVessel);
    require_keys_eq!(
        collection.key,
        season_config.vessel_collection,
        Sojourn9Error::InvalidVessel
    );

    let expected_asset_id = get_asset_id(
        &selected_vessel_merkle_tree.key(),
        args.selected_vessel_proof.nonce,
    );
    require_keys_eq!(
        expected_asset_id,
        args.selected_vessel_asset_id,
        Sojourn9Error::InvalidVessel
    );

    let data_hash = hash_metadata(&metadata).map_err(|_| error!(Sojourn9Error::InvalidVessel))?;
    let creator_hash = hash_creators(&metadata.creators);
    let selected_vessel_leaf = LeafSchema::V1 {
        id: expected_asset_id,
        owner: args.selected_vessel_proof.owner,
        delegate: args.selected_vessel_proof.delegate,
        nonce: args.selected_vessel_proof.nonce,
        data_hash,
        creator_hash,
    };

    verify_selected_vessel_leaf(
        selected_vessel_merkle_tree,
        args.selected_vessel_proof.merkle_root,
        selected_vessel_leaf.hash(),
        args.selected_vessel_proof.index,
        proof_accounts,
    )?;

    require_keys_eq!(
        args.selected_vessel_proof.owner,
        current_steward,
        Sojourn9Error::InvalidSteward
    );

    Ok(())
}

fn verify_selected_vessel_leaf(
    selected_vessel_merkle_tree: &AccountInfo,
    merkle_root: [u8; 32],
    leaf_hash: [u8; 32],
    index: u32,
    proof_accounts: &[AccountInfo],
) -> Result<()> {
    let merkle_tree_bytes = selected_vessel_merkle_tree
        .try_borrow_data()
        .map_err(|_| error!(Sojourn9Error::InvalidAccounts))?;
    let (header_bytes, rest) = merkle_tree_bytes.split_at(CONCURRENT_MERKLE_TREE_HEADER_SIZE_V1);
    let header = <ConcurrentMerkleTreeHeader as anchor_lang::prelude::borsh::BorshDeserialize>::try_from_slice(header_bytes)
        .map_err(|_| error!(Sojourn9Error::InvalidAccounts))?;
    header
        .assert_valid()
        .map_err(|_| error!(Sojourn9Error::InvalidAccounts))?;
    header
        .assert_valid_leaf_index(index)
        .map_err(|_| error!(Sojourn9Error::InvalidVessel))?;

    let merkle_tree_size =
        merkle_tree_get_size(&header).map_err(|_| error!(Sojourn9Error::InvalidAccounts))?;
    let (tree_bytes, canopy_bytes) = rest.split_at(merkle_tree_size);
    let mut proof = proof_accounts
        .iter()
        .map(|node| node.key().to_bytes())
        .collect::<Vec<Node>>();
    fill_in_proof_from_canopy(canopy_bytes, header.get_max_depth(), index, &mut proof)
        .map_err(|_| error!(Sojourn9Error::InvalidVessel))?;

    let prove_leaf_args = ProveLeafArgs {
        current_root: merkle_root,
        leaf: leaf_hash,
        proof_vec: proof,
        index,
    };

    merkle_tree_prove_leaf(&header, tree_bytes, &prove_leaf_args)?;

    Ok(())
}

fn create_daily_seal_account<'info>(
    current_steward: &AccountInfo<'info>,
    daily_seal: &AccountInfo<'info>,
    system_program_account: &AccountInfo<'info>,
    season_config: &Pubkey,
    selected_vessel_asset_id: &Pubkey,
    utc_day: i64,
) -> Result<u8> {
    let (expected_daily_seal, bump) =
        find_daily_seal_pda(season_config, selected_vessel_asset_id, utc_day);
    require_keys_eq!(
        daily_seal.key(),
        expected_daily_seal,
        Sojourn9Error::InvalidAccounts
    );

    if *daily_seal.owner == crate::id() {
        return err!(Sojourn9Error::AlreadySealedToday);
    }

    require_keys_eq!(
        *daily_seal.owner,
        system_program::ID,
        Sojourn9Error::InvalidAccounts
    );
    require!(
        daily_seal.lamports() == 0 && daily_seal.data_is_empty(),
        Sojourn9Error::InvalidAccounts
    );

    let daily_seal_space = (8 + DailySeal::INIT_SPACE) as u64;
    let daily_seal_rent = Rent::get()?.minimum_balance(daily_seal_space as usize);
    let utc_day_seed = utc_day_seed_bytes(utc_day);
    let signer_seeds: [&[u8]; 5] = [
        crate::constants::DAILY_SEAL_SEED,
        season_config.as_ref(),
        selected_vessel_asset_id.as_ref(),
        &utc_day_seed,
        &[bump],
    ];

    invoke_signed(
        &system_instruction::create_account(
            current_steward.key,
            daily_seal.key,
            daily_seal_rent,
            daily_seal_space,
            &crate::id(),
        ),
        &[
            current_steward.clone(),
            daily_seal.clone(),
            system_program_account.clone(),
        ],
        &[&signer_seeds],
    )?;

    Ok(bump)
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SelectedVesselProof {
    pub merkle_root: [u8; 32],
    pub nonce: u64,
    pub index: u32,
    pub owner: Pubkey,
    pub delegate: Pubkey,
    pub metadata: SelectedVesselMetadataArgs,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SelectedVesselMetadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
    pub edition_nonce: Option<u8>,
    pub token_standard: Option<SelectedVesselTokenStandard>,
    pub collection: Option<SelectedVesselCollection>,
    pub uses: Option<SelectedVesselUses>,
    pub token_program_version: SelectedVesselTokenProgramVersion,
    pub creators: Vec<SelectedVesselCreator>,
}

impl SelectedVesselMetadataArgs {
    fn to_bubblegum_metadata(&self) -> BubblegumMetadataArgs {
        BubblegumMetadataArgs {
            name: self.name.clone(),
            symbol: self.symbol.clone(),
            uri: self.uri.clone(),
            seller_fee_basis_points: self.seller_fee_basis_points,
            primary_sale_happened: self.primary_sale_happened,
            is_mutable: self.is_mutable,
            edition_nonce: self.edition_nonce,
            token_standard: self
                .token_standard
                .as_ref()
                .map(SelectedVesselTokenStandard::to_bubblegum),
            collection: self
                .collection
                .as_ref()
                .map(SelectedVesselCollection::to_bubblegum),
            uses: self.uses.as_ref().map(SelectedVesselUses::to_bubblegum),
            token_program_version: self.token_program_version.to_bubblegum(),
            creators: self
                .creators
                .iter()
                .map(SelectedVesselCreator::to_bubblegum)
                .collect(),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SelectedVesselCollection {
    pub verified: bool,
    pub key: Pubkey,
}

impl SelectedVesselCollection {
    fn to_bubblegum(&self) -> BubblegumCollection {
        BubblegumCollection {
            verified: self.verified,
            key: self.key,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SelectedVesselCreator {
    pub address: Pubkey,
    pub verified: bool,
    pub share: u8,
}

impl SelectedVesselCreator {
    fn to_bubblegum(&self) -> BubblegumCreator {
        BubblegumCreator {
            address: self.address,
            verified: self.verified,
            share: self.share,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SelectedVesselUses {
    pub use_method: SelectedVesselUseMethod,
    pub remaining: u64,
    pub total: u64,
}

impl SelectedVesselUses {
    fn to_bubblegum(&self) -> BubblegumUses {
        BubblegumUses {
            use_method: self.use_method.to_bubblegum(),
            remaining: self.remaining,
            total: self.total,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum SelectedVesselUseMethod {
    Burn,
    Multiple,
    Single,
}

impl SelectedVesselUseMethod {
    fn to_bubblegum(&self) -> BubblegumUseMethod {
        match self {
            Self::Burn => BubblegumUseMethod::Burn,
            Self::Multiple => BubblegumUseMethod::Multiple,
            Self::Single => BubblegumUseMethod::Single,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum SelectedVesselTokenProgramVersion {
    Original,
    Token2022,
}

impl SelectedVesselTokenProgramVersion {
    fn to_bubblegum(&self) -> BubblegumTokenProgramVersion {
        match self {
            Self::Original => BubblegumTokenProgramVersion::Original,
            Self::Token2022 => BubblegumTokenProgramVersion::Token2022,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum SelectedVesselTokenStandard {
    NonFungible,
    FungibleAsset,
    Fungible,
    NonFungibleEdition,
}

impl SelectedVesselTokenStandard {
    fn to_bubblegum(&self) -> BubblegumTokenStandard {
        match self {
            Self::NonFungible => BubblegumTokenStandard::NonFungible,
            Self::FungibleAsset => BubblegumTokenStandard::FungibleAsset,
            Self::Fungible => BubblegumTokenStandard::Fungible,
            Self::NonFungibleEdition => BubblegumTokenStandard::NonFungibleEdition,
        }
    }
}

#[cfg(test)]
mod tests {
    use anchor_lang::solana_program::pubkey::Pubkey;
    use spl_concurrent_merkle_tree::concurrent_merkle_tree::ConcurrentMerkleTree;
    use spl_merkle_tree_reference::{MerkleTree, Node};

    use super::*;

    const TEST_TREE_MAX_DEPTH: usize = 3;
    const TEST_TREE_MAX_BUFFER_SIZE: usize = 8;

    #[test]
    fn seal_v1_happy_path_accepts_active_valid_vessel() {
        let owner = Pubkey::new_unique();
        let vessel_collection = Pubkey::new_unique();
        let fixture = ValidVesselFixture::new(owner, vessel_collection);
        let season_config = sample_season_config(0, vessel_collection);

        let chamber_index = derive_active_chamber_index(&season_config, 0).unwrap();
        assert_eq!(chamber_index, 0);

        verify_selected_vessel(
            &season_config,
            *fixture.current_steward.account.key,
            &fixture.selected_vessel_merkle_tree.account,
            &fixture.selected_vessel_tree_config.account,
            &fixture.seal_args,
            &fixture.proof_accounts,
        )
        .unwrap();
    }

    #[test]
    fn seal_v1_rejects_already_sealed_today() {
        let season_config = Pubkey::new_unique();
        let selected_vessel_asset_id = Pubkey::new_unique();
        let utc_day = 42;
        let (daily_seal_key, _) =
            find_daily_seal_pda(&season_config, &selected_vessel_asset_id, utc_day);
        let existing_daily_seal = TestAccount::program_owned(
            daily_seal_key,
            crate::id(),
            vec![0; 8 + DailySeal::INIT_SPACE],
        );
        let payer = TestAccount::signer(Pubkey::new_unique());
        let system_program_account =
            TestAccount::program_owned(system_program::ID, system_program::ID, Vec::new());

        let error = create_daily_seal_account(
            &payer.account,
            &existing_daily_seal.account,
            &system_program_account.account,
            &season_config,
            &selected_vessel_asset_id,
            utc_day,
        )
        .unwrap_err();

        assert_eq!(error, error!(Sojourn9Error::AlreadySealedToday));
    }

    #[test]
    fn seal_v1_rejects_inactive_season() {
        let season_config = sample_season_config(86_400, Pubkey::new_unique());

        let error = derive_active_chamber_index(&season_config, 0).unwrap_err();
        assert_eq!(error, error!(Sojourn9Error::InactiveSeason));
    }

    #[test]
    fn seal_v1_rejects_invalid_vessel_collection() {
        let owner = Pubkey::new_unique();
        let vessel_collection = Pubkey::new_unique();
        let invalid_collection = Pubkey::new_unique();
        let fixture = ValidVesselFixture::new(owner, invalid_collection);
        let season_config = sample_season_config(0, vessel_collection);

        let error = verify_selected_vessel(
            &season_config,
            *fixture.current_steward.account.key,
            &fixture.selected_vessel_merkle_tree.account,
            &fixture.selected_vessel_tree_config.account,
            &fixture.seal_args,
            &fixture.proof_accounts,
        )
        .unwrap_err();

        assert_eq!(error, error!(Sojourn9Error::InvalidVessel));
    }

    #[test]
    fn seal_v1_rejects_wrong_steward() {
        let owner = Pubkey::new_unique();
        let vessel_collection = Pubkey::new_unique();
        let fixture = ValidVesselFixture::new(owner, vessel_collection);
        let season_config = sample_season_config(0, vessel_collection);
        let wrong_steward = TestAccount::signer(Pubkey::new_unique());

        let error = verify_selected_vessel(
            &season_config,
            *wrong_steward.account.key,
            &fixture.selected_vessel_merkle_tree.account,
            &fixture.selected_vessel_tree_config.account,
            &fixture.seal_args,
            &fixture.proof_accounts,
        )
        .unwrap_err();

        assert_eq!(error, error!(Sojourn9Error::InvalidSteward));
    }

    fn sample_season_config(season_start_unix_ts: i64, vessel_collection: Pubkey) -> SeasonConfig {
        SeasonConfig {
            bump: 0,
            authority: Pubkey::new_unique(),
            season_number: crate::constants::SOJOURN_NUMBER,
            season_start_unix_ts,
            chamber_count: crate::constants::CHAMBER_COUNT,
            region_count: crate::constants::REGION_COUNT,
            chambers_per_region: crate::constants::CHAMBERS_PER_REGION,
            canonical_vessel_count: crate::constants::CANONICAL_VESSEL_COUNT,
            vessel_collection,
            constitution_revision: 9,
            constitution_hash: [7; 32],
        }
    }

    struct ValidVesselFixture {
        current_steward: TestAccount,
        selected_vessel_merkle_tree: TestAccount,
        selected_vessel_tree_config: TestAccount,
        proof_accounts: Vec<AccountInfo<'static>>,
        seal_args: SealAnkyArgs,
    }

    impl ValidVesselFixture {
        fn new(owner: Pubkey, vessel_collection: Pubkey) -> Self {
            let tree_key = Pubkey::new_unique();
            let tree_config_key = BubblegumTreeConfig::find_pda(&tree_key).0;
            let delegate = Pubkey::new_unique();
            let metadata = sample_metadata(vessel_collection);
            let data_hash = hash_metadata(&metadata.to_bubblegum_metadata()).unwrap();
            let creator_hash = hash_creators(
                &metadata
                    .creators
                    .iter()
                    .map(SelectedVesselCreator::to_bubblegum)
                    .collect::<Vec<_>>(),
            );
            let asset_id = get_asset_id(&tree_key, 0);
            let leaf = LeafSchema::V1 {
                id: asset_id,
                owner,
                delegate,
                nonce: 0,
                data_hash,
                creator_hash,
            };

            let mut proof_tree =
                MerkleTree::new(vec![Node::default(); 1 << TEST_TREE_MAX_DEPTH].as_slice());
            proof_tree.add_leaf(leaf.hash(), 0);

            let selected_vessel_merkle_tree = compression_tree_account(tree_key, leaf.hash());
            let selected_vessel_tree_config = bubblegum_tree_config_account(tree_config_key);
            let proof_accounts = proof_tree
                .get_proof_of_leaf(0)
                .into_iter()
                .map(TestAccount::proof_node)
                .collect::<Vec<_>>();
            let current_steward = TestAccount::signer(owner);

            Self {
                current_steward,
                selected_vessel_merkle_tree,
                selected_vessel_tree_config,
                proof_accounts,
                seal_args: SealAnkyArgs {
                    selected_vessel_asset_id: asset_id,
                    anky_hash_commitment: [9; 32],
                    selected_vessel_proof: SelectedVesselProof {
                        merkle_root: proof_tree.root,
                        nonce: 0,
                        index: 0,
                        owner,
                        delegate,
                        metadata,
                    },
                },
            }
        }
    }

    fn sample_metadata(vessel_collection: Pubkey) -> SelectedVesselMetadataArgs {
        SelectedVesselMetadataArgs {
            name: "Sojourn Vessel".to_string(),
            symbol: "S9".to_string(),
            uri: "https://example.com/vessel.json".to_string(),
            seller_fee_basis_points: 0,
            primary_sale_happened: false,
            is_mutable: false,
            edition_nonce: Some(0),
            token_standard: Some(SelectedVesselTokenStandard::NonFungible),
            collection: Some(SelectedVesselCollection {
                verified: true,
                key: vessel_collection,
            }),
            uses: None,
            token_program_version: SelectedVesselTokenProgramVersion::Original,
            creators: vec![SelectedVesselCreator {
                address: Pubkey::new_unique(),
                verified: true,
                share: 100,
            }],
        }
    }

    fn compression_tree_account(key: Pubkey, leaf_hash: [u8; 32]) -> TestAccount {
        let mut account_data = vec![
            0;
            CONCURRENT_MERKLE_TREE_HEADER_SIZE_V1
                + core::mem::size_of::<
                    ConcurrentMerkleTree<TEST_TREE_MAX_DEPTH, TEST_TREE_MAX_BUFFER_SIZE>,
                >()
        ];
        let (header_bytes, rest) = account_data.split_at_mut(CONCURRENT_MERKLE_TREE_HEADER_SIZE_V1);
        let mut header = <ConcurrentMerkleTreeHeader as anchor_lang::prelude::borsh::BorshDeserialize>::try_from_slice(header_bytes)
            .unwrap();
        header.initialize(
            TEST_TREE_MAX_DEPTH as u32,
            TEST_TREE_MAX_BUFFER_SIZE as u32,
            &Pubkey::new_unique(),
            0,
        );
        anchor_lang::prelude::borsh::BorshSerialize::serialize(&header, &mut &mut header_bytes[..])
            .unwrap();
        let tree_size = merkle_tree_get_size(&header).unwrap();
        let (tree_bytes, _) = rest.split_at_mut(tree_size);
        crate::utils::compression::merkle_tree_initialize_empty(&header, tree_bytes).unwrap();
        crate::utils::compression::merkle_tree_append_leaf(&header, tree_bytes, &leaf_hash)
            .unwrap();

        TestAccount::program_owned(key, SPL_ACCOUNT_COMPRESSION_ID, account_data)
    }

    fn bubblegum_tree_config_account(key: Pubkey) -> TestAccount {
        let mut data = Vec::new();
        anchor_lang::prelude::borsh::BorshSerialize::serialize(
            &BubblegumTreeConfig {
                discriminator: [0; 8],
                tree_creator: Pubkey::new_unique(),
                tree_delegate: Pubkey::new_unique(),
                total_mint_capacity: 8,
                num_minted: 1,
                is_public: false,
                is_decompressible: mpl_bubblegum::types::DecompressibleState::Disabled,
                version: mpl_bubblegum::types::Version::V1,
            },
            &mut data,
        )
        .unwrap();

        TestAccount::program_owned(key, MPL_BUBBLEGUM_ID, data)
    }

    struct TestAccount {
        account: AccountInfo<'static>,
    }

    impl TestAccount {
        fn signer(key: Pubkey) -> Self {
            Self::new(key, system_program::ID, Vec::new(), true, true, 0)
        }

        fn program_owned(key: Pubkey, owner: Pubkey, data: Vec<u8>) -> Self {
            Self::new(key, owner, data, false, true, 0)
        }

        fn proof_node(node: Node) -> AccountInfo<'static> {
            Self::new(
                Pubkey::new_from_array(node),
                system_program::ID,
                Vec::new(),
                false,
                false,
                0,
            )
            .account
        }

        fn new(
            key: Pubkey,
            owner: Pubkey,
            data: Vec<u8>,
            is_signer: bool,
            is_writable: bool,
            lamports: u64,
        ) -> Self {
            let key = Box::leak(Box::new(key));
            let owner = Box::leak(Box::new(owner));
            let lamports = Box::leak(Box::new(lamports));
            let data = Box::leak(data.into_boxed_slice());

            Self {
                account: AccountInfo::new(
                    key,
                    is_signer,
                    is_writable,
                    lamports,
                    data,
                    owner,
                    false,
                    0,
                ),
            }
        }
    }
}
