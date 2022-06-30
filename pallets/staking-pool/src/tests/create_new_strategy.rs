use rp_base::store_get;

use super::{mock::*, *};

#[test]
#[rio_syntax]
fn now_owner() {
  |"testbox!"| {
    assert_noop!(Pallet::create_new_strategy(Origin::signed(ALICE), 10_u128, 10_u32, 10_u32), BadOrigin);
  }
}

#[test]
#[rio_syntax]
fn duration_is_zero() {
  |"testbox!"| {
    assert_noop!(
      Pallet::create_new_strategy(Origin::signed(ROOT), 10_u128, 10_u32, 0_u32),
      Error::VOSpDurationIsZero
    );
  }
}

#[test]
#[rio_syntax]
fn block_less_than_current() {
  |"testbox!"| {
    frame_system::Pallet::<Test>::set_block_number(1_u32);
    |"store!"| {
      LastUpdateBlockNumber = 1_u32;
    };
    assert_eq!(
      Pallet::create_new_strategy(Origin::signed(ROOT), 10_u128, 0_u32, 10_u32),
      Err(Error::VOSpStartBlockNumberLessThanCurrent.into())
    );
  }
}

#[test]
#[rio_syntax]
fn more_than_limit() {
  |"testbox!"| {
    assert_eq!(
      Pallet::create_new_strategy(Origin::signed(ROOT), 1881_u128, 10_u32, 10_u32),
      Err(Error::VOSpPerBlockRewardOverflow.into())
    );
  }
}

#[test]
#[rio_syntax]
fn success() {
  |"testbox!"| {
    assert_ok!(Pallet::create_new_strategy(Origin::signed(ROOT), 10_u128, 0_u32, 10_u32));
  }
}

#[test]
#[rio_syntax]
fn initilaize_simple() {
  |"testbox!"| {
    frame_system::Pallet::<Test>::set_block_number(1000_u32);
    assert_ok!(Pallet::initialize_simple(Origin::signed(ROOT), 1_u128, 1000_u32, 1000_u32));
    frame_system::Pallet::<Test>::set_block_number(1050_u32);
    let l = store_get!(LastUpdateBlockNumber);
    assert_ok!(Pallet::create_new_strategy(Origin::signed(ROOT), 1_u128, 1060_u32, 1000_u32));
    let start_block_number = store_get!(NextStrategy).start_block_number;
    assert_eq!(start_block_number, 1060_u32)
  };
}
