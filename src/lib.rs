#![no_std]

mod contracts;

mod circuit_breaker;
mod drips;
mod gas;
mod guardian;
mod reentrancy;
mod reputation;
mod storage;
mod task;
mod timelock;
mod types;
mod vault;
pub mod events;

pub use contracts::proxy_entry::{VeroContract, VeroContractClient};
pub use guardian::{add_guardian, remove_guardian, is_guardian};
pub use task::{get_task, register_tasks};
pub use drips::{get_reward_stream, start_drips_stream};
pub use types::Operation;

const DEFAULT_WEIGHT_THRESHOLD: u64 = 300;
