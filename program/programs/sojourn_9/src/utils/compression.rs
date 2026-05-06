use std::mem::size_of;

use anchor_lang::{
    error,
    prelude::{
        borsh::{BorshDeserialize, BorshSerialize},
        *,
    },
    solana_program::pubkey,
};
use bytemuck::{cast_slice, Pod};
use spl_concurrent_merkle_tree::{
    concurrent_merkle_tree::{ConcurrentMerkleTree, ProveLeafArgs},
    node::{empty_node_cached, Node, EMPTY},
};

use crate::errors::Sojourn9Error;

pub const SPL_ACCOUNT_COMPRESSION_ID: Pubkey =
    pubkey!("cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK");
pub const CONCURRENT_MERKLE_TREE_HEADER_SIZE_V1: usize = 56;

#[derive(Debug, Copy, Clone, PartialEq, Eq, BorshDeserialize, BorshSerialize)]
#[repr(u8)]
pub enum CompressionAccountType {
    Uninitialized,
    ConcurrentMerkleTree,
}

#[repr(C)]
#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub struct ConcurrentMerkleTreeHeader {
    pub account_type: CompressionAccountType,
    pub header: ConcurrentMerkleTreeHeaderData,
}

#[repr(C)]
#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub enum ConcurrentMerkleTreeHeaderData {
    V1(ConcurrentMerkleTreeHeaderDataV1),
}

#[repr(C)]
#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub struct ConcurrentMerkleTreeHeaderDataV1 {
    max_buffer_size: u32,
    max_depth: u32,
    authority: Pubkey,
    creation_slot: u64,
    _padding: [u8; 6],
}

impl ConcurrentMerkleTreeHeader {
    #[cfg(test)]
    pub fn initialize(
        &mut self,
        max_depth: u32,
        max_buffer_size: u32,
        authority: &Pubkey,
        creation_slot: u64,
    ) {
        self.account_type = CompressionAccountType::ConcurrentMerkleTree;

        match &mut self.header {
            ConcurrentMerkleTreeHeaderData::V1(header) => {
                header.max_buffer_size = max_buffer_size;
                header.max_depth = max_depth;
                header.authority = *authority;
                header.creation_slot = creation_slot;
            }
        }
    }

    pub fn get_max_depth(&self) -> u32 {
        match &self.header {
            ConcurrentMerkleTreeHeaderData::V1(header) => header.max_depth,
        }
    }

    pub fn get_max_buffer_size(&self) -> u32 {
        match &self.header {
            ConcurrentMerkleTreeHeaderData::V1(header) => header.max_buffer_size,
        }
    }

    pub fn assert_valid(&self) -> Result<()> {
        require!(
            self.account_type == CompressionAccountType::ConcurrentMerkleTree,
            Sojourn9Error::InvalidAccounts
        );

        Ok(())
    }

    pub fn assert_valid_leaf_index(&self, leaf_index: u32) -> Result<()> {
        require!(
            leaf_index < (1 << self.get_max_depth()),
            Sojourn9Error::InvalidVessel
        );

        Ok(())
    }
}

pub fn merkle_tree_get_size(header: &ConcurrentMerkleTreeHeader) -> Result<usize> {
    match (header.get_max_depth(), header.get_max_buffer_size()) {
        (3, 8) => Ok(size_of::<ConcurrentMerkleTree<3, 8>>()),
        (5, 8) => Ok(size_of::<ConcurrentMerkleTree<5, 8>>()),
        (6, 16) => Ok(size_of::<ConcurrentMerkleTree<6, 16>>()),
        (7, 16) => Ok(size_of::<ConcurrentMerkleTree<7, 16>>()),
        (8, 16) => Ok(size_of::<ConcurrentMerkleTree<8, 16>>()),
        (9, 16) => Ok(size_of::<ConcurrentMerkleTree<9, 16>>()),
        (10, 32) => Ok(size_of::<ConcurrentMerkleTree<10, 32>>()),
        (11, 32) => Ok(size_of::<ConcurrentMerkleTree<11, 32>>()),
        (12, 32) => Ok(size_of::<ConcurrentMerkleTree<12, 32>>()),
        (13, 32) => Ok(size_of::<ConcurrentMerkleTree<13, 32>>()),
        (14, 64) => Ok(size_of::<ConcurrentMerkleTree<14, 64>>()),
        (14, 256) => Ok(size_of::<ConcurrentMerkleTree<14, 256>>()),
        (14, 1024) => Ok(size_of::<ConcurrentMerkleTree<14, 1024>>()),
        (14, 2048) => Ok(size_of::<ConcurrentMerkleTree<14, 2048>>()),
        (15, 64) => Ok(size_of::<ConcurrentMerkleTree<15, 64>>()),
        (16, 64) => Ok(size_of::<ConcurrentMerkleTree<16, 64>>()),
        (17, 64) => Ok(size_of::<ConcurrentMerkleTree<17, 64>>()),
        (18, 64) => Ok(size_of::<ConcurrentMerkleTree<18, 64>>()),
        (19, 64) => Ok(size_of::<ConcurrentMerkleTree<19, 64>>()),
        (20, 64) => Ok(size_of::<ConcurrentMerkleTree<20, 64>>()),
        (20, 256) => Ok(size_of::<ConcurrentMerkleTree<20, 256>>()),
        (20, 1024) => Ok(size_of::<ConcurrentMerkleTree<20, 1024>>()),
        (20, 2048) => Ok(size_of::<ConcurrentMerkleTree<20, 2048>>()),
        (24, 64) => Ok(size_of::<ConcurrentMerkleTree<24, 64>>()),
        (24, 256) => Ok(size_of::<ConcurrentMerkleTree<24, 256>>()),
        (24, 512) => Ok(size_of::<ConcurrentMerkleTree<24, 512>>()),
        (24, 1024) => Ok(size_of::<ConcurrentMerkleTree<24, 1024>>()),
        (24, 2048) => Ok(size_of::<ConcurrentMerkleTree<24, 2048>>()),
        (26, 512) => Ok(size_of::<ConcurrentMerkleTree<26, 512>>()),
        (26, 1024) => Ok(size_of::<ConcurrentMerkleTree<26, 1024>>()),
        (26, 2048) => Ok(size_of::<ConcurrentMerkleTree<26, 2048>>()),
        (30, 512) => Ok(size_of::<ConcurrentMerkleTree<30, 512>>()),
        (30, 1024) => Ok(size_of::<ConcurrentMerkleTree<30, 1024>>()),
        (30, 2048) => Ok(size_of::<ConcurrentMerkleTree<30, 2048>>()),
        _ => err!(Sojourn9Error::InvalidAccounts),
    }
}

pub fn fill_in_proof_from_canopy(
    canopy_bytes: &[u8],
    max_depth: u32,
    index: u32,
    proof: &mut Vec<Node>,
) -> Result<()> {
    check_canopy_bytes(canopy_bytes)?;
    let canopy = cast_slice::<u8, Node>(canopy_bytes);
    let path_len = get_cached_path_length(canopy, max_depth)?;
    let empty_node_cache = Box::new([EMPTY; 30]);
    let mut node_idx = ((1 << max_depth) + index) >> (max_depth - path_len);
    let mut inferred_nodes = Vec::new();

    while node_idx > 1 {
        let shifted_index = node_idx as usize - 2;
        let cached_idx = if shifted_index.is_multiple_of(2) {
            shifted_index + 1
        } else {
            shifted_index - 1
        };

        if canopy[cached_idx] == EMPTY {
            let level = max_depth - (31 - node_idx.leading_zeros());
            inferred_nodes.push(empty_node_cached::<30>(level, &empty_node_cache));
        } else {
            inferred_nodes.push(canopy[cached_idx]);
        }

        node_idx >>= 1;
    }

    let overlap = (proof.len() + inferred_nodes.len()).saturating_sub(max_depth as usize);
    proof.extend(inferred_nodes.iter().skip(overlap));

    Ok(())
}

pub fn merkle_tree_prove_leaf(
    header: &ConcurrentMerkleTreeHeader,
    tree_bytes: &[u8],
    prove_leaf_args: &ProveLeafArgs,
) -> Result<()> {
    match (header.get_max_depth(), header.get_max_buffer_size()) {
        (3, 8) => prove_leaf_with_depth::<3, 8>(tree_bytes, prove_leaf_args),
        (5, 8) => prove_leaf_with_depth::<5, 8>(tree_bytes, prove_leaf_args),
        (6, 16) => prove_leaf_with_depth::<6, 16>(tree_bytes, prove_leaf_args),
        (7, 16) => prove_leaf_with_depth::<7, 16>(tree_bytes, prove_leaf_args),
        (8, 16) => prove_leaf_with_depth::<8, 16>(tree_bytes, prove_leaf_args),
        (9, 16) => prove_leaf_with_depth::<9, 16>(tree_bytes, prove_leaf_args),
        (10, 32) => prove_leaf_with_depth::<10, 32>(tree_bytes, prove_leaf_args),
        (11, 32) => prove_leaf_with_depth::<11, 32>(tree_bytes, prove_leaf_args),
        (12, 32) => prove_leaf_with_depth::<12, 32>(tree_bytes, prove_leaf_args),
        (13, 32) => prove_leaf_with_depth::<13, 32>(tree_bytes, prove_leaf_args),
        (14, 64) => prove_leaf_with_depth::<14, 64>(tree_bytes, prove_leaf_args),
        (14, 256) => prove_leaf_with_depth::<14, 256>(tree_bytes, prove_leaf_args),
        (14, 1024) => prove_leaf_with_depth::<14, 1024>(tree_bytes, prove_leaf_args),
        (14, 2048) => prove_leaf_with_depth::<14, 2048>(tree_bytes, prove_leaf_args),
        (15, 64) => prove_leaf_with_depth::<15, 64>(tree_bytes, prove_leaf_args),
        (16, 64) => prove_leaf_with_depth::<16, 64>(tree_bytes, prove_leaf_args),
        (17, 64) => prove_leaf_with_depth::<17, 64>(tree_bytes, prove_leaf_args),
        (18, 64) => prove_leaf_with_depth::<18, 64>(tree_bytes, prove_leaf_args),
        (19, 64) => prove_leaf_with_depth::<19, 64>(tree_bytes, prove_leaf_args),
        (20, 64) => prove_leaf_with_depth::<20, 64>(tree_bytes, prove_leaf_args),
        (20, 256) => prove_leaf_with_depth::<20, 256>(tree_bytes, prove_leaf_args),
        (20, 1024) => prove_leaf_with_depth::<20, 1024>(tree_bytes, prove_leaf_args),
        (20, 2048) => prove_leaf_with_depth::<20, 2048>(tree_bytes, prove_leaf_args),
        (24, 64) => prove_leaf_with_depth::<24, 64>(tree_bytes, prove_leaf_args),
        (24, 256) => prove_leaf_with_depth::<24, 256>(tree_bytes, prove_leaf_args),
        (24, 512) => prove_leaf_with_depth::<24, 512>(tree_bytes, prove_leaf_args),
        (24, 1024) => prove_leaf_with_depth::<24, 1024>(tree_bytes, prove_leaf_args),
        (24, 2048) => prove_leaf_with_depth::<24, 2048>(tree_bytes, prove_leaf_args),
        (26, 512) => prove_leaf_with_depth::<26, 512>(tree_bytes, prove_leaf_args),
        (26, 1024) => prove_leaf_with_depth::<26, 1024>(tree_bytes, prove_leaf_args),
        (26, 2048) => prove_leaf_with_depth::<26, 2048>(tree_bytes, prove_leaf_args),
        (30, 512) => prove_leaf_with_depth::<30, 512>(tree_bytes, prove_leaf_args),
        (30, 1024) => prove_leaf_with_depth::<30, 1024>(tree_bytes, prove_leaf_args),
        (30, 2048) => prove_leaf_with_depth::<30, 2048>(tree_bytes, prove_leaf_args),
        _ => err!(Sojourn9Error::InvalidAccounts),
    }
}

#[cfg(test)]
pub fn merkle_tree_initialize_empty(
    header: &ConcurrentMerkleTreeHeader,
    tree_bytes: &mut [u8],
) -> Result<()> {
    match (header.get_max_depth(), header.get_max_buffer_size()) {
        (3, 8) => initialize_empty_with_depth::<3, 8>(tree_bytes),
        _ => err!(Sojourn9Error::InvalidAccounts),
    }
}

#[cfg(test)]
pub fn merkle_tree_append_leaf(
    header: &ConcurrentMerkleTreeHeader,
    tree_bytes: &mut [u8],
    leaf: &[u8; 32],
) -> Result<()> {
    match (header.get_max_depth(), header.get_max_buffer_size()) {
        (3, 8) => append_leaf_with_depth::<3, 8>(tree_bytes, leaf),
        _ => err!(Sojourn9Error::InvalidAccounts),
    }
}

fn check_canopy_bytes(canopy_bytes: &[u8]) -> Result<()> {
    require!(
        canopy_bytes.len().is_multiple_of(size_of::<Node>()),
        Sojourn9Error::InvalidAccounts
    );

    Ok(())
}

fn get_cached_path_length(canopy: &[Node], max_depth: u32) -> Result<u32> {
    let closest_power_of_2 = (canopy.len() + 2) as u32;
    require!(
        closest_power_of_2 & (closest_power_of_2 - 1) == 0,
        Sojourn9Error::InvalidAccounts
    );
    require!(
        closest_power_of_2 <= (1 << (max_depth + 1)),
        Sojourn9Error::InvalidAccounts
    );

    Ok(closest_power_of_2.trailing_zeros() - 1)
}

fn prove_leaf_with_depth<const MAX_DEPTH: usize, const MAX_BUFFER_SIZE: usize>(
    tree_bytes: &[u8],
    prove_leaf_args: &ProveLeafArgs,
) -> Result<()> {
    let merkle_tree =
        load_tree_bytes::<ConcurrentMerkleTree<MAX_DEPTH, MAX_BUFFER_SIZE>>(tree_bytes)?;
    merkle_tree
        .prove_leaf(prove_leaf_args)
        .map_err(|_| error!(Sojourn9Error::InvalidVessel))?;

    Ok(())
}

#[cfg(test)]
fn initialize_empty_with_depth<const MAX_DEPTH: usize, const MAX_BUFFER_SIZE: usize>(
    tree_bytes: &mut [u8],
) -> Result<()> {
    let merkle_tree =
        load_tree_bytes_mut::<ConcurrentMerkleTree<MAX_DEPTH, MAX_BUFFER_SIZE>>(tree_bytes)?;
    merkle_tree
        .initialize()
        .map_err(|_| error!(Sojourn9Error::InvalidAccounts))?;

    Ok(())
}

#[cfg(test)]
fn append_leaf_with_depth<const MAX_DEPTH: usize, const MAX_BUFFER_SIZE: usize>(
    tree_bytes: &mut [u8],
    leaf: &[u8; 32],
) -> Result<()> {
    let merkle_tree =
        load_tree_bytes_mut::<ConcurrentMerkleTree<MAX_DEPTH, MAX_BUFFER_SIZE>>(tree_bytes)?;
    merkle_tree
        .append(*leaf)
        .map_err(|_| error!(Sojourn9Error::InvalidAccounts))?;

    Ok(())
}

fn load_tree_bytes<T: Pod>(data: &[u8]) -> Result<&T> {
    let size = size_of::<T>();
    bytemuck::try_from_bytes(&data[..size]).map_err(|_| error!(Sojourn9Error::InvalidAccounts))
}

#[cfg(test)]
fn load_tree_bytes_mut<T: Pod>(data: &mut [u8]) -> Result<&mut T> {
    let size = size_of::<T>();
    bytemuck::try_from_bytes_mut(&mut data[..size])
        .map_err(|_| error!(Sojourn9Error::InvalidAccounts))
}
