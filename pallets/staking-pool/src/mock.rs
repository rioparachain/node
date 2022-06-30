use frame_support::{
  ord_parameter_types, pallet_prelude::Get, parameter_types, traits::GenesisBuild, PalletId,
};
use frame_system::{EnsureRoot, EnsureSignedBy};
use rp_base::{Amount, Balance, BlockNumber, CurrencyId, Header, Price};
use rp_protocol::{RFUEL, STAKING_POOL_MARKER};
//use frame_system as system;
use sp_core::H256;
use sp_runtime::traits::{AccountIdConversion, BlakeTwo256, IdentityLookup, One};

use crate as rpallet_staking_pool;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub const MILLISECS_PER_BLOCK: u64 = 2000;
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

pub type AccountId = u64;

pub const ROOT: AccountId = 0;
pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;

parameter_types! {
    pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

/*
impl rio_root::Config for Test {
  type Currency = RioAssets;
  type Event = Event;
  type RootOrigin = EnsureRootOrManager;
  type WeightInfo = ();
}
*/

parameter_types! {
  pub const BlockHashCount: BlockNumber = 250;
  pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
  type AccountData = pallet_balances::AccountData<Balance>;
  type AccountId = AccountId;
  type BaseCallFilter = frame_support::traits::Everything;
  type BlockHashCount = BlockHashCount;
  type BlockLength = ();
  type BlockNumber = BlockNumber;
  type BlockWeights = ();
  type Call = Call;
  type DbWeight = ();
  type Event = Event;
  type Hash = H256;
  type Hashing = BlakeTwo256;
  type Header = Header;
  type Index = u64;
  type Lookup = IdentityLookup<Self::AccountId>;
  type MaxConsumers = frame_support::traits::ConstU32<16>;
  type OnKilledAccount = ();
  type OnNewAccount = ();
  type OnSetCode = ();
  type Origin = Origin;
  type PalletInfo = PalletInfo;
  type SS58Prefix = SS58Prefix;
  type SystemWeightInfo = ();
  type Version = ();
}

pub struct DefaultPrice;
impl Get<Price> for DefaultPrice {
  fn get() -> Price { Price::one() }
}

//type EnsureRootOrManager = EnsureOneOf<AccountId, EnsureRoot<AccountId>, EnsureSignedBy<RioRoot,
// AccountId>>;

impl pallet_timestamp::Config for Test {
  type MinimumPeriod = MinimumPeriod;
  /// A timestamp: milliseconds since the unix epoch.
  type Moment = u64;
  type OnTimestampSet = ();
  type WeightInfo = ();
}

impl rpallet_staking_pool::Config for Test {
  type DefaultPrice = DefaultPrice;
  type Event = Event;
  type MarkerCurrency = rpallet_assets::CurrencyAdapter<Test, StakingPoolMarker>;
  type MaximumPerBlockReward = MaximumPerBlockReward;
  type MinimumStakeBalance = MinimumStakeBalance;
  //type OwnerOrigin = EnsureRootOrManager;
  type OwnerOrigin = EnsureSignedBy<OwnerAcc, AccountId>;
  type PalletId = RioStakingPoolPalletId;
  //type StakeCurrency = rio_assets::frame_currency::Compat<Test,Rfuel>;
  type StakeCurrency = Balances;
  type WeightInfo = ();
}

ord_parameter_types! {
  pub const OwnerAcc: u64 = 0;
}

parameter_types! {
    pub const RioStakingPoolPalletId: PalletId = PalletId(*b"py/riosp");
    pub const MinimumStakeBalance: Balance = 10_u128;
    pub const MaximumPerBlockReward: Balance = MinimumStakeBalance::get() * 188_u128;
    //pub const DefaultPrice: Price = Price::one();
    pub const Rfuel: CurrencyId = RFUEL;
    pub const StakingPoolMarker: CurrencyId = STAKING_POOL_MARKER;
}

use orml_traits::parameter_type_with_key;
use sp_std::convert::{TryFrom, TryInto};

pub struct DustRemovalWhitelist;
impl frame_support::traits::Contains<AccountId> for DustRemovalWhitelist {
  fn contains(a: &AccountId) -> bool { *a == DustReceiver::get() }
}

parameter_type_with_key! {
    pub ExistentialDeposits: |currency_id: CurrencyId| -> Balance {
        #[allow(clippy::match_ref_pats)] // false positive
        match currency_id {
            //&BTC => 1,
            //&DOT => 2,
            _ => 0,
        }
    };
}

parameter_types! {
    pub DustReceiver: AccountId = PalletId(*b"rioc/dst").into_account_truncating();
    pub StringLimit: u32 = 128;
}

impl rpallet_assets::Config for Test {
  type Amount = Amount;
  type Balance = Balance;
  type CurrencyId = CurrencyId;
  type DustRemovalWhitelist = DustRemovalWhitelist;
  type Event = Event;
  type ExistentialDeposits = ExistentialDeposits;
  type MaxLocks = frame_support::traits::ConstU32<2>;
  type MaxReserves = frame_support::traits::ConstU32<2>;
  type OnDust = ();
  type OnKilledTokenAccount = ();
  type OnNewTokenAccount = ();
  type ReserveIdentifier = [u8; 8];
  type StringLimit = StringLimit;
  type WeightInfo = ();
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 0;
    pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Test {
  type AccountStore = System;
  /// The type for recording an account's balance.
  type Balance = Balance;
  type DustRemoval = ();
  /// The ubiquitous event type.
  type Event = Event;
  type ExistentialDeposit = ExistentialDeposit;
  type MaxLocks = MaxLocks;
  type MaxReserves = ();
  type ReserveIdentifier = [u8; 8];
  type WeightInfo = pallet_balances::weights::SubstrateWeight<Test>;
}

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
  pub enum Test where
    Block = Block,
    NodeBlock = Block,
    UncheckedExtrinsic = UncheckedExtrinsic,
  {
    System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
    Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
    Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
    RioAssets: rpallet_assets::{Pallet, Call, Storage, Config<T>, Event<T>},
    //RioRoot: rio_root::{Pallet, Call, Storage, Config<T>, Event<T>},
    RioStakingPool: rpallet_staking_pool::{Pallet, Call, Storage, Event<T>},
    //TemplateModule: pallet_template::{Pallet, Call, Storage, Event<T>},
  }
);

/*
// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
  frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
*/

pub struct ExtBuilder;

impl Default for ExtBuilder {
  fn default() -> Self { ExtBuilder }
}

use frame_support::BoundedVec;
pub type Text = BoundedVec<u8, StringLimit>;

macro_rules! bvec {
  ($a:expr) => {
    Text::try_from($a.to_vec()).unwrap()
  };
}

impl ExtBuilder {
  pub fn build(self) -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
    pallet_balances::GenesisConfig::<Test> { balances: vec![(ALICE, 1_u128), (BOB, 1_u128)] }
      .assimilate_storage(&mut t)
      .unwrap();
    rpallet_assets::GenesisConfig::<Test> {
      init: vec![
        (
          rp_protocol::RFUEL,
          rpallet_assets::AssetInfo {
            symbol:   bvec!(b"RFUEL"),
            name:     bvec!(b"Rfuel"),
            decimals: 12,
            desc:     bvec!(b"Rfuel"),
            chain:    rpallet_assets::Chain::Rio,
          },
          rpallet_assets::Restrictions::none(),
          vec![],
        ),
        (
          rp_protocol::STAKING_POOL_MARKER,
          rpallet_assets::AssetInfo {
            symbol:   bvec!(b"STAKING_POOL_MARKER"),
            name:     bvec!(b"Staking pool marker"),
            decimals: 12,
            desc:     bvec!(b"Staking pool marker"),
            chain:    rpallet_assets::Chain::Rio,
          },
          rpallet_assets::Restrictions::none(),
          vec![],
        ),
      ],
    }
    .assimilate_storage(&mut t)
    .unwrap();
    t.into()
  }
}
