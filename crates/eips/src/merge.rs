//! Constants related to the beacon chain consensus.

use std::time::Duration;

/// An EPOCH is a series of 32 slots.
pub const EPOCH_SLOTS: u64 = 32;

/// The duration of a slot in seconds.
///
/// This is the time period of 12 seconds in which a randomly chosen validator has time to propose a
/// block.
pub const SLOT_DURATION_SECS: u64 = 12;

/// An EPOCH is a series of 32 slots (~6.4min).
pub const EPOCH_DURATION_SECS: u64 = EPOCH_SLOTS * SLOT_DURATION_SECS;

/// The duration of a slot in seconds.
///
/// This is the time period of 12 seconds in which a randomly chosen validator has time to propose a
/// block.
pub const SLOT_DURATION: Duration = Duration::from_secs(SLOT_DURATION_SECS);

/// An EPOCH is a series of 32 slots (~6.4min).
pub const EPOCH_DURATION: Duration = Duration::from_secs(EPOCH_DURATION_SECS);

/// The default block nonce in the beacon consensus
pub const BEACON_NONCE: u64 = 0u64;

/// The number of blocks to unwind during a reorg that already became a part of canonical chain.
///
/// In reality, the node can end up in this particular situation very rarely. It would happen only
/// if the node process is abruptly terminated during ongoing reorg and doesn't boot back up for
/// long period of time.
///
/// Unwind depth of `3` blocks significantly reduces the chance that the reorged block is kept in
/// the database.
pub const BEACON_CONSENSUS_REORG_UNWIND_DEPTH: u64 = 3;

/// Max seconds from current time allowed for blocks, before they're considered future blocks.
///
/// This is only used when checking whether or not the timestamp for pre-merge blocks is in the
/// future.
///
/// See:
/// <https://github.com/ethereum/go-ethereum/blob/a196f3e8a22b6ad22ced5c2e3baf32bc3ebd4ec9/consensus/ethash/consensus.go#L227-L229>
pub const ALLOWED_FUTURE_BLOCK_TIME_SECONDS: u64 = 15;
