use super::{mock::*, *};

#[test]
#[rio_syntax]
fn now_owner() {
  |"testbox!"| {
    assert_noop!(Pallet::update_unstaking_time(Origin::signed(ALICE), 10_u64), BadOrigin);
  };
}
