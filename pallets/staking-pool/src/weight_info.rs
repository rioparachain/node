use frame_support::weights::{
  constants::{RocksDbWeight as DbWeight, WEIGHT_PER_MICROS},
  Weight,
};

pub trait WeightInfo {
  fn default_weight() -> Weight {
    WEIGHT_PER_MICROS.saturating_mul(88).saturating_add(DbWeight::get().reads_writes(4, 2))
  }
  fn deposit_to_stake() -> Weight { Self::default_weight() }
  fn claim() -> Weight { Self::default_weight() }
  fn claim_fees() -> Weight { Self::default_weight() }
  fn decrease_pool() -> Weight { Self::default_weight() }
  fn cancel_unstaking() -> Weight { Self::default_weight() }
  fn withdraw_from_unstaked() -> Weight { Self::default_weight() }
  fn update_unstaking_time() -> Weight { Self::default_weight() }
  fn increase_pool() -> Weight { Self::default_weight() }
  fn stake_for_user() -> Weight { Self::default_weight() }
  fn unstake() -> Weight { Self::default_weight() }
  fn create_new_strategy() -> Weight { Self::default_weight() }
  fn set_claiming_fee_percent() -> Weight { Self::default_weight() }
  fn initialize_simple() -> Weight { Self::default_weight() }
}

impl WeightInfo for () {}
