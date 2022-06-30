pub(crate) use frame_support::{assert_noop, assert_ok};
pub(crate) use mock::{self, Test};
pub(crate) use sp_runtime::traits::BadOrigin;

pub(crate) use super::*;

pub(crate) type Error = super::Error<Test>;
pub(crate) type Pallet = super::Pallet<Test>;
pub(crate) type StakeCurrency = <Test as Config>::StakeCurrency;
pub(crate) type MarkerCurrency = <Test as Config>::MarkerCurrency;

pub use rio_proc_macro::rio_syntax;
pub use rp_base::store;

mod claim_fees;
mod create_new_strategy;
mod deposit_to_stake;
mod set_claiming_fee_percent;
mod set_unstaking_time;
mod stake_for_user;
mod unstake;

#[macro_export]
macro_rules! GEN_PATH(($A:ident,$b:ident) => { $A::<Test>::$b });

#[macro_export]
macro_rules! testbox {
  ($($t:tt)*) => {
    ExtBuilder::default().build().execute_with(|| {
      $($t)*
    });
  }
}

/*
mod deposit_to_stake;
mod stake_for_user;
mod unstake;
mod set_unstaking_time;
mod set_claiming_fee_percent;
mod decrease_pool;
mod claim_fees;
*/
