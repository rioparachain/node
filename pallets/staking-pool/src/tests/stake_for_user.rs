use super::{mock::*, *};

const STAKE: u128 = 10_u128;
const INVALID_STAKE: u128 = 1_u128;

use frame_support::traits::{Currency as FrameCurrency, ExistenceRequirement};
use rp_base::*;

#[test]
#[rio_syntax]
fn zero_amount() {
  |"testbox!"| {
    assert_noop!(Pallet::stake_for_user(Origin::signed(ROOT), ALICE, 0u128), Error::AmountIsNotPositive);
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
      Pallet::stake_for_user(Origin::signed(ROOT), ALICE, INVALID_STAKE),
      Error::MinimalStakeBalanceShouldBeMoreOrEqualToOneAssetMarker
    );
  };
}

#[test]
#[rio_syntax]
fn all_good() {
  |"testbox!"| {
    |"store!"| {
      PriceStored = Price::from(1);
    };
    StakeCurrency::deposit_creating(&ROOT, 999_u128);
    let balance_before = StakeCurrency::free_balance(&ROOT);
    Pallet::create_new_strategy(Origin::signed(ROOT), 10_u128, 0_u32, 50_u32);
    // frame_system::Pallet::<Test>::set_block_number(10_u32);
    assert_ok!(Pallet::stake_for_user(Origin::signed(ROOT), ALICE, STAKE));
    assert_eq!(MarkerCurrency::free_balance(&ALICE), 10_u128);
    // assert_eq!(Pallet::last_stake_time(Origin::signed(ALICE),BOB), Some(10_u128));
  };
}

#[test]
#[rio_syntax]
fn sixty_by_forty() {
  |"testbox!"| {
    |"store!"| {
      PriceStored = Price::from(1);
    };
    StakeCurrency::deposit_creating(&ALICE, 60_u128);
    StakeCurrency::deposit_creating(&BOB, 40_u128);
    assert_eq!(StakeCurrency::free_balance(&ALICE), 61_u128);
    assert_eq!(StakeCurrency::free_balance(&BOB), 41_u128);

    frame_system::Pallet::<Test>::set_block_number(0_u32);
    assert_ok!(Pallet::initialize_simple(Origin::signed(ROOT), 1_u128, 5_u32, 100_u32));
    frame_system::Pallet::<Test>::set_block_number(4_u32);
    assert_ok!(Pallet::deposit_to_stake(Origin::signed(ALICE), 60_u128));
    frame_system::Pallet::<Test>::set_block_number(6_u32);
    assert_ok!(Pallet::deposit_to_stake(Origin::signed(BOB), 40_u128));

    frame_system::Pallet::<Test>::set_block_number(60_u32);
    assert_ok!(Pallet::update_unstaking_time(Origin::signed(ROOT), 10_u64));
    assert_eq!(StakeCurrency::free_balance(&ALICE), 1_u128);
    assert_eq!(StakeCurrency::free_balance(&BOB), 1_u128);

    assert_ok!((Pallet::unstake(Origin::signed(ALICE), 60_u128)));
    assert_ok!((Pallet::unstake(Origin::signed(BOB), 40_u128)));
    pallet_timestamp::Pallet::<Test>::set_timestamp(200);
    assert_ok!(Pallet::withdraw_from_unstaked(Origin::signed(ALICE)));
    assert_ok!(Pallet::withdraw_from_unstaked(Origin::signed(BOB)));

    assert_eq!(StakeCurrency::free_balance(&ALICE), 61_u128);
    assert_eq!(StakeCurrency::free_balance(&BOB), 41_u128);
  };
}

#[test]
#[rio_syntax]
fn fifty_by_fifty() {
  |"testbox!"| {
    |"store!"| {
      PriceStored = Price::from(1);
    };
    StakeCurrency::deposit_creating(&ROOT, 100_u128);
    StakeCurrency::deposit_creating(&ALICE, 50_u128);
    StakeCurrency::deposit_creating(&BOB, 50_u128);
    assert_eq!(StakeCurrency::free_balance(&ROOT), 100_u128);
    assert_eq!(StakeCurrency::free_balance(&ALICE), 51_u128);
    assert_eq!(StakeCurrency::free_balance(&BOB), 51_u128);

    frame_system::Pallet::<Test>::set_block_number(0_u32);

    assert_ok!(Pallet::initialize_simple(Origin::signed(ROOT), 1_u128, 5_u32, 100_u32));

    frame_system::Pallet::<Test>::set_block_number(2_u32);
    assert_ok!(Pallet::deposit_to_stake(Origin::signed(ALICE), 50_u128));
    frame_system::Pallet::<Test>::set_block_number(4_u32);
    assert_ok!(Pallet::deposit_to_stake(Origin::signed(BOB), 50_u128));
    assert_ok!(Pallet::update_unstaking_time(Origin::signed(ROOT), 10_u64));
    assert_ok!(Pallet::increase_pool(Origin::signed(ROOT), 100_u128));
    StakeCurrency::transfer(
      Origin::signed(ROOT),
      Pallet::get_pallet_account(),
      100,
      //ExistenceRequirement::KeepAlive,
    );

    frame_system::Pallet::<Test>::set_block_number(60_u32);
    assert_ok!(Pallet::create_new_strategy(Origin::signed(ROOT), 1_u128, 105_u32, 100_u32));

    frame_system::Pallet::<Test>::set_block_number(110_u32);

    assert_eq!(StakeCurrency::free_balance(&ALICE), 1_u128);
    assert_eq!(StakeCurrency::free_balance(&BOB), 1_u128);

    assert_ok!((Pallet::unstake(Origin::signed(ALICE), 100_u128)));
    assert_ok!((Pallet::unstake(Origin::signed(BOB), 100_u128)));
    pallet_timestamp::Pallet::<Test>::set_timestamp(200);
    assert_ok!(Pallet::withdraw_from_unstaked(Origin::signed(ALICE)));
    assert_ok!(Pallet::withdraw_from_unstaked(Origin::signed(BOB)));

    assert_eq!(StakeCurrency::free_balance(&ALICE), 101_u128);
    assert_eq!(StakeCurrency::free_balance(&BOB), 101_u128);
    assert_eq!(StakeCurrency::free_balance(&ROOT), 0_u128);
  };
}

#[test]
#[rio_syntax]
fn eighty_by_twenty() {
  |"testbox!"| {
    |"store!"| {
      PriceStored = Price::from(1);
    };
    StakeCurrency::deposit_creating(&ROOT, 100_u128);
    StakeCurrency::deposit_creating(&ALICE, 60_u128);
    StakeCurrency::deposit_creating(&BOB, 40_u128);
    assert_eq!(StakeCurrency::free_balance(&ROOT), 100_u128);
    assert_eq!(StakeCurrency::free_balance(&ALICE), 61_u128);
    assert_eq!(StakeCurrency::free_balance(&BOB), 41_u128);

    frame_system::Pallet::<Test>::set_block_number(0_u32);

    assert_ok!(Pallet::initialize_simple(Origin::signed(ROOT), 1_u128, 5_u32, 100_u32));

    frame_system::Pallet::<Test>::set_block_number(2_u32);
    assert_ok!(Pallet::deposit_to_stake(Origin::signed(ALICE), 60_u128));
    frame_system::Pallet::<Test>::set_block_number(4_u32);
    assert_ok!(Pallet::deposit_to_stake(Origin::signed(BOB), 40_u128));
    assert_ok!(Pallet::update_unstaking_time(Origin::signed(ROOT), 10_u64));
    assert_ok!(Pallet::increase_pool(Origin::signed(ROOT), 100_u128));
    StakeCurrency::transfer(
      Origin::signed(ROOT),
      Pallet::get_pallet_account(),
      100,
      //ExistenceRequirement::KeepAlive,
    );

    frame_system::Pallet::<Test>::set_block_number(55_u32);
    assert_ok!(Pallet::create_new_strategy(Origin::signed(ROOT), 1_u128, 105_u32, 100_u32));
    assert_ok!((Pallet::unstake(Origin::signed(BOB), 60_u128)));
    pallet_timestamp::Pallet::<Test>::set_timestamp(200);
    assert_ok!(Pallet::withdraw_from_unstaked(Origin::signed(BOB)));

    frame_system::Pallet::<Test>::set_block_number(110_u32);
    assert_ok!((Pallet::unstake(Origin::signed(ALICE), 140_u128)));
    pallet_timestamp::Pallet::<Test>::set_timestamp(400);
    assert_ok!(Pallet::withdraw_from_unstaked(Origin::signed(ALICE)));

    assert_eq!(StakeCurrency::free_balance(&ALICE), 141_u128);
    assert_eq!(StakeCurrency::free_balance(&BOB), 61_u128);
    assert_eq!(StakeCurrency::free_balance(&ROOT), 0_u128);
  };
}

#[test]
#[rio_syntax]
fn cancel_unstaking() {
  |"testbox!"| {
    |"store!"| {
      PriceStored = Price::from(1);
    };
    StakeCurrency::deposit_creating(&ROOT, 100_u128);
    StakeCurrency::deposit_creating(&ALICE, 60_u128);
    assert_eq!(StakeCurrency::free_balance(&ROOT), 100_u128);
    assert_eq!(StakeCurrency::free_balance(&ALICE), 61_u128);

    frame_system::Pallet::<Test>::set_block_number(0_u32);

    assert_ok!(Pallet::initialize_simple(Origin::signed(ROOT), 1_u128, 5_u32, 100_u32));

    frame_system::Pallet::<Test>::set_block_number(2_u32);
    assert_ok!(Pallet::deposit_to_stake(Origin::signed(ALICE), 60_u128));
    assert_ok!(Pallet::update_unstaking_time(Origin::signed(ROOT), 0_u64));

    frame_system::Pallet::<Test>::set_block_number(55_u32);
    assert_ok!(Pallet::create_new_strategy(Origin::signed(ROOT), 1_u128, 105_u32, 100_u32));
    assert_ok!((Pallet::unstake(Origin::signed(ALICE), 60_u128)));
    assert_ok!(Pallet::cancel_unstaking(Origin::signed(ALICE), 60_u128));
  };
}

#[test]
#[rio_syntax]
fn claim_fees() {
  |"testbox!"| {
    |"store!"| {
      PriceStored = Price::from(1);
    };
    StakeCurrency::deposit_creating(&ROOT, 100_u128);
    StakeCurrency::deposit_creating(&ALICE, 60_u128);
    assert_eq!(StakeCurrency::free_balance(&ROOT), 100_u128);
    assert_eq!(StakeCurrency::free_balance(&ALICE), 61_u128);

    frame_system::Pallet::<Test>::set_block_number(0_u32);

    assert_ok!(Pallet::initialize_simple(Origin::signed(ROOT), 1_u128, 5_u32, 100_u32));

    frame_system::Pallet::<Test>::set_block_number(2_u32);
    assert_ok!(Pallet::deposit_to_stake(Origin::signed(ALICE), 60_u128));
    assert_ok!(Pallet::update_unstaking_time(Origin::signed(ROOT), 0_u64));
    assert_ok!(Pallet::set_claiming_fee_percent(Origin::signed(ROOT), Percent::from_percent(5)));

    frame_system::Pallet::<Test>::set_block_number(55_u32);
    assert_ok!(Pallet::create_new_strategy(Origin::signed(ROOT), 1_u128, 105_u32, 100_u32));
    assert_ok!((Pallet::claim(Origin::signed(ALICE), 60_u128)));
    assert_eq!(StakeCurrency::free_balance(&ALICE), 58_u128);
  };
}

#[test]
#[rio_syntax]
fn sixty_by_forty_large_values() {
  |"testbox!"| {
    |"store!"| {
      PriceStored = Price::from(1);
    };
    StakeCurrency::deposit_creating(&ALICE, 1_u128 << 68);
    StakeCurrency::deposit_creating(&BOB, 1u128 << 68);

    frame_system::Pallet::<Test>::set_block_number(0_u32);
    assert_ok!(Pallet::initialize_simple(Origin::signed(ROOT), 1_u128, 5_u32, 100_u32));
    frame_system::Pallet::<Test>::set_block_number(4_u32);
    assert_ok!(Pallet::deposit_to_stake(Origin::signed(ALICE), 1_u128 << 68));
    frame_system::Pallet::<Test>::set_block_number(6_u32);
    assert_ok!(Pallet::deposit_to_stake(Origin::signed(BOB), 1_u128 << 68));

    frame_system::Pallet::<Test>::set_block_number(60_u32);
    assert_ok!(Pallet::update_unstaking_time(Origin::signed(ROOT), 10_u64));

    assert_ok!((Pallet::unstake(Origin::signed(ALICE), 1_u128 << 68)));
    assert_ok!((Pallet::unstake(Origin::signed(BOB), 1_u128 << 68)));
    pallet_timestamp::Pallet::<Test>::set_timestamp(200);
    assert_ok!(Pallet::withdraw_from_unstaked(Origin::signed(ALICE)));
    assert_ok!(Pallet::withdraw_from_unstaked(Origin::signed(BOB)));

    assert_eq!(StakeCurrency::free_balance(&ALICE), (1_u128 << 68) + 1);
    assert_eq!(StakeCurrency::free_balance(&BOB), (1_u128 << 68) + 1);
  };
}

#[test]
#[rio_syntax]
fn eighty_by_twenty_large_values() {
  |"testbox!"| {
    |"store!"| {
      PriceStored = Price::from(1);
    };
    StakeCurrency::deposit_creating(&ROOT, 1_u128 << 68);
    StakeCurrency::deposit_creating(&ALICE, 60_u128);
    StakeCurrency::deposit_creating(&BOB, 40_u128);

    frame_system::Pallet::<Test>::set_block_number(0_u32);

    assert_ok!(Pallet::initialize_simple(Origin::signed(ROOT), 1_u128, 5_u32, 100_u32));

    frame_system::Pallet::<Test>::set_block_number(2_u32);
    assert_ok!(Pallet::deposit_to_stake(Origin::signed(ALICE), 60_u128));
    frame_system::Pallet::<Test>::set_block_number(4_u32);
    assert_ok!(Pallet::deposit_to_stake(Origin::signed(BOB), 40_u128));
    assert_ok!(Pallet::update_unstaking_time(Origin::signed(ROOT), 10_u64));
    assert_ok!(Pallet::increase_pool(Origin::signed(ROOT), 1_u128 << 68));

    StakeCurrency::transfer(
      Origin::signed(ROOT),
      Pallet::get_pallet_account(),
      1_u128 << 68,
      //ExistenceRequirement::KeepAlive,
    );

    frame_system::Pallet::<Test>::set_block_number(55_u32);
    assert_ok!(Pallet::create_new_strategy(
      Origin::signed(ROOT),
      <Test as Config>::MaximumPerBlockReward::get(),
      105_u32,
      100_u32
    ));
    assert_ok!((Pallet::unstake(Origin::signed(BOB), 60_u128)));
    pallet_timestamp::Pallet::<Test>::set_timestamp(200);
    assert_ok!(Pallet::withdraw_from_unstaked(Origin::signed(BOB)));

    assert_eq!(store_get!(PriceStored), Price::from_float(1.5f64));

    frame_system::Pallet::<Test>::set_block_number(110_u32);
    assert_ok!((Pallet::unstake(Origin::signed(ALICE), 9000_u128)));
    pallet_timestamp::Pallet::<Test>::set_timestamp(400);
    assert_ok!(Pallet::withdraw_from_unstaked(Origin::signed(ALICE)));

    assert_eq!(StakeCurrency::free_balance(&ALICE), 9001_u128);
    assert_eq!(StakeCurrency::free_balance(&BOB), 61_u128);
    assert_eq!(StakeCurrency::free_balance(&ROOT), 0_u128);
  };
}
