use frame_support::traits::Currency as FrameCurrency;
use rp_base::*;

use super::{mock::*, *};

const STAKE: u128 = 10_u128;
const INVALID_STAKE: u128 = 9_u128;

#[test]
#[rio_syntax]
fn zero_amount() {
  |"testbox!"| {
    assert_noop!(Pallet::deposit_to_stake(Origin::signed(ROOT), 0u128), Error::AmountIsNotPositive);
  };
}

#[test]
#[rio_syntax]
fn less_than_lower() {
  |"testbox!"| {
    |"store!"| {
      PriceStored = Price::from(1);
    };
    StakeCurrency::deposit_creating(&ROOT, 999_u128);
    assert_noop!(
      Pallet::deposit_to_stake(Origin::signed(ROOT), INVALID_STAKE),
      Error::MinimalStakeBalanceShouldBeMoreOrEqualToOneAssetMarker
    );
  }
}

#[test]
#[rio_syntax]
fn all_good() {
  |"testbox!"| {
    |"store!"| {
      PriceStored = Price::from(1);
    };
    Pallet::create_new_strategy(Origin::signed(ROOT), 10_u128, 0_u32, 50_u32);
    StakeCurrency::deposit_creating(&ROOT, 999_u128);
    let balance_before = StakeCurrency::free_balance(&ROOT);
    // frame_system::Pallet::<Test>::set_block_number(10_u32);
    assert_ok!(Pallet::deposit_to_stake(Origin::signed(ROOT), STAKE));
    assert_eq!(MarkerCurrency::free_balance(&ROOT), 10u128);
    // assert_eq!(Pallet::last_stake_time(ALICE).unwrap(), 10_u64);
  };
}

#[test]
#[rio_syntax]
fn checked_from_interger_failed_direct_overflow() {
  |"testbox!"| {
    |"store!"| {
      PriceStored = Price::from(1);
    };
    Pallet::create_new_strategy(Origin::signed(ROOT), 10_u128, 0_u32, 50_u32);
    StakeCurrency::deposit_creating(&ROOT, 340282366920938463463374607431768211455u128);
    assert_noop!(
      Pallet::deposit_to_stake(Origin::signed(ROOT), 340282366920938463464u128),
      Error::CheckedFromIntegerFailed,
    );
  };
}

#[test]
#[rio_syntax]
fn checked_from_interger_failed_overflow_by_marker_balance() {
  |"testbox!"| {
    |"store!"| {
      PriceStored = Price::from(1);
    };
    Pallet::create_new_strategy(Origin::signed(ROOT), 10_u128, 0_u32, 50_u32);
    StakeCurrency::deposit_creating(&ROOT, 340282366920938463463374607431768211455u128);
    // log 340282366920938463463 / log 2 = 68.20529429202747
    assert_ok!(Pallet::deposit_to_stake(Origin::signed(ROOT), 340282366920938463462u128));
    assert_ok!(Pallet::deposit_to_stake(Origin::signed(ROOT), 1u128));
    assert_noop!(Pallet::deposit_to_stake(Origin::signed(ROOT), 1u128), Error::CheckedFromIntegerFailed,);
  };
}
