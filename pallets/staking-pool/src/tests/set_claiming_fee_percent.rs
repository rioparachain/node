use super::{mock::*, *};

#[test]
#[rio_syntax]
fn now_owner() {
  |"testbox!"| {
    assert_noop!(
      Pallet::set_claiming_fee_percent(Origin::signed(ALICE), sp_runtime::Percent::one()),
      BadOrigin
    );
  };
}
