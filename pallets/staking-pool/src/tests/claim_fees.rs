use frame_support::traits::Currency as FrameCurrency;
use rp_base::*;

use super::{mock::*, *};

#[test]
#[rio_syntax]
fn now_owner() {
  |"testbox!"| {
    assert_noop!(Pallet::claim_fees(Origin::signed(ALICE)), BadOrigin);
  };
}

#[test]
#[rio_syntax]
fn no_fees() {
  |"testbox!"| {
    assert_noop!(Pallet::claim_fees(Origin::signed(ROOT)), Error::NoFees);
  };
}

#[test]
#[rio_syntax]
fn fees_exist() {
  |"testbox!"| {
    |"store!"| {
      PriceStored = Price::from(1);
    };
    StakeCurrency::deposit_creating(&ALICE, 999_u128);
    assert_ok!(Pallet::deposit_to_stake(Origin::signed(ALICE), 100_u128));
    //assert_ok!(Module::claim(Origin::signed(BOB), 20_u128));
    //assert_noop!(Module::claim_fees(Origin::signed(ALICE)), Error::NoFees);
  };
}
