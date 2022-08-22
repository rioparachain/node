use super::*;
use crate as rio_gateway;
use frame_support::{
	parameter_types,
	traits::{ConstU32, ConstU64, Contains},
	weights::{
		DispatchClass, DispatchInfo, GetDispatchInfo, PostDispatchInfo, RuntimeDbWeight, Weight,
		WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
	},
	PalletId,
};
use sp_runtime::AccountId32;
pub type StringLimit = ConstU32<128>;
use frame_support::traits::Get;
use orml_traits::parameter_type_with_key;
use rp_base::{Amount, Balance, CurrencyId};
use sp_core::H256;
use sp_runtime::traits::AccountIdConversion;

pub(crate) type GatewayModule = super::Pallet<Test>;
pub(crate) type GatewayErr = super::Error<Test>;
pub const DAVE: AccountId32 = AccountId32::new([3u8; 32]);
use pallet_sudo as sudo;
// use crate::sp_api_hidden_includes_decl_storage::hidden_include::traits::GenesisBuild;
use sp_std::convert::TryFrom;

#[allow(unused_imports)]
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, Block as BlockT, ConvertInto, IdentityLookup, Saturating, StaticLookup},
	MultiSignature, Perbill,
};

pub use crate::types::{
	Auth, Auths, Bip32, Create2, Deposit, DepositAddrInfo, TxHash, WithdrawInfo, WithdrawItem,
	WithdrawState,
};

pub type AccountId = AccountId32;
pub const ROOT: AccountId = AccountId32::new([4u8; 32]);

pub const BOB: AccountId = AccountId32::new([1u8; 32]);
pub const CHRIS: AccountId = AccountId32::new([2u8; 32]);

pub mod constants {
	use super::{Balance, CurrencyId};
	use sp_runtime::AccountId32;
	pub type AccountId = AccountId32;
	pub const DECIMALS: u128 = 100000000; // satoshi
	pub const ALICE: AccountId = AccountId32::new([0u8; 32]);
	pub const BOB: AccountId = AccountId32::new([1u8; 32]);
	pub const BIG_STRING: &[u8; 81] =
		b"1234567890qwertyuiopasdfghjklzxcvmqwertyuioasdfghjklzxcvbnmoiquweiquwegwjhqwejhqw";
	pub const ADDRESS: &[u8; 12] = b"some_address";
	pub const MEMO: &[u8; 9] = b"some_memo";
	pub const CUR1: CurrencyId = 1;
	pub const CUR2: CurrencyId = 2;
	pub const PUB_KEY: &[u8; 6] = b"pubkey";
	pub const PATH_PREFIX: &[u8; 11] = b"path_prefix";
	pub const CASUAL_TRANSFER: Balance = DECIMALS * 10;
	pub const LARGE_TRANSFER: Balance = DECIMALS * 10000;
}

use self::constants::*;
use std::cell::RefCell;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type BlockNumber = u64;

thread_local! {
	static EXTRINSIC_BASE_WEIGHT: RefCell<u64> = RefCell::new(0);
}

pub struct ExtrinsicBaseWeight;
impl Get<u64> for ExtrinsicBaseWeight {
	fn get() -> u64 {
		EXTRINSIC_BASE_WEIGHT.with(|v| *v.borrow())
	}
}

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);
const MAXIMUM_BLOCK_WEIGHT: Weight = 1024;
const AVERAGE_ON_INITIALIZE_WEIGHT: Perbill = Perbill::from_percent(10);
parameter_types! {
	pub RioBlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights::builder()
		.base_block(10)
		.for_class(DispatchClass::all(), |weights| {
			weights.base_extrinsic = ExtrinsicBaseWeight::get();
		})
		.for_class(DispatchClass::Normal, |weights| {
			weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
		})
		.for_class(DispatchClass::Operational, |weights| {
			weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
			// Operational transactions have some extra reserved space, so that they
			// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
			weights.reserved = Some(
				MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
			);
		})
		.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
		.build_or_panic();
	pub RioBlockLength: frame_system::limits::BlockLength =
	frame_system::limits::BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub const BlockHashCount: BlockNumber = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = RioBlockWeights;
	type BlockLength = RioBlockLength;
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 0;
	pub const TransferFee: u64 = 0;
	pub const CreationFee: u64 = 0;
	pub const TransactionBaseFee: u64 = 0;
	pub const TransactionByteFee: u64 = 0;
}

impl pallet_sudo::Config for Test {
	type Event = Event;
	type Call = Call;
}

parameter_types! {
	pub const MinimumPeriod: u64 = 1000;
}

impl timestamp::Config for Test {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_type_with_key! {
	pub ExistentialDeposits: |currency_id: CurrencyId| -> Balance {
		#[allow(clippy::match_ref_pats)] // false positive
		match currency_id {
			&BTC => 1,
			&DOT => 2,
			_ => 0,
		}
	};
}

parameter_types! {
	pub DustReceiver: AccountId = PalletId(*b"orml/dst").into_account_truncating();
}

pub struct MockDustRemovalWhitelist;
impl Contains<AccountId> for MockDustRemovalWhitelist {
	fn contains(a: &AccountId) -> bool {
		*a == DAVE || *a == DustReceiver::get()
	}
}

impl rpallet_assets::Config for Test {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type OnNewTokenAccount = ();
	type OnKilledTokenAccount = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = ();
	type DustRemovalWhitelist = MockDustRemovalWhitelist;
	type StringLimit = StringLimit;
}

impl rio_gateway::Config for Test {
	type Event = Event;
	type Currency = RioAssets;
	type WeightInfo = ();
}

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Sudo: sudo::{Pallet, Call, Config<T>, Storage, Event<T>},
		Gateway: rio_gateway::{Pallet, Call, Storage, Config<T>, Event<T>},
		Timestamp: timestamp::{Pallet, Call, Storage, Inherent},
		RioAssets: rpallet_assets::{Pallet, Call, Storage, Config<T>, Event<T>},
	}
);

pub struct ExtBuilder {
	init: Vec<(
		CurrencyId,
		rpallet_assets::AssetInfo<BoundedVec<u8, StringLimit>>,
		rpallet_assets::Restrictions,
		Vec<(AccountId, Balance)>,
	)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self { init: vec![] }
	}
}
pub type Text = BoundedVec<u8, StringLimit>;
macro_rules! bvec {
	($a:expr) => {
		Text::try_from($a.to_vec()).unwrap()
	};
}

impl ExtBuilder {
	pub fn build(mut self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

		sudo::GenesisConfig::<Test> { key: Some(ROOT) }
			.assimilate_storage(&mut t)
			.unwrap();

		self.init.append(&mut vec![(
			CUR1,
			rpallet_assets::AssetInfo {
				symbol: bvec!(b"CUR1"),
				name: bvec!(b"CUR2 token"),
				decimals: 6,
				desc: bvec!(b"CUR1"),
				chain: rpallet_assets::Chain::Ethereum,
			},
			rpallet_assets::Restrictions::none(),
			vec![],
		)]);

		rpallet_assets::GenesisConfig::<Test> { init: self.init }
			.assimilate_storage(&mut t)
			.unwrap();

		rio_gateway::GenesisConfig::<Test> {
			admins: vec![(ROOT, Auths::all()), (ALICE, Auths::all())],
			deposit_addr_info: vec![],
			initial_supported_currencies: vec![(CUR1, DECIMALS)],
			max_deposit_index: 1000,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}
