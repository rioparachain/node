use super::{mock::*, *};

const STAKE: u128 = 25_u128;
const UNSTAKE: u128 = 10_u128;

use frame_support::traits::Currency as FrameCurrency;
use rp_base::*;

#[test]
#[rio_syntax]
fn zero() {
  |"testbox!"| {
    assert_noop!(Pallet::unstake(Origin::signed(ROOT), 0_u128), Error::AmountIsNotPositive);
  };
}

#[test]
#[rio_syntax]
fn more_than_balance() {
  |"testbox!"| {
    |"store!"| {
      TotalStaked = 100_u128;
      PriceStored = Price::from(1);
    };
    StakeCurrency::deposit_creating(&ROOT, 999_u128);
    assert_noop!(Pallet::unstake(Origin::signed(ROOT), 1_u128), Error::NotEnoughMarkerUnits);
  };
}

#[test]
#[rio_syntax]
fn less_or_equal_to_balance() {
  |"testbox!"| {
    |"store!"| {
      PriceStored = Price::from(1);
    };
    StakeCurrency::deposit_creating(&ROOT, 999_u128);
    assert_ok!(Pallet::deposit_to_stake(Origin::signed(ROOT), STAKE));
  };
}
