#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use sp_std::prelude::Vec;

use sp_std::collections::btree_map::BTreeMap;

pub use rp_base::Price;
pub use rpallet_gateway::{WithdrawItem, WithdrawState};
use sp_runtime::traits::MaybeDisplay;

sp_api::decl_runtime_apis! {
	pub trait GatewayApi<CurrencyId, AccountId, Balance> where
		CurrencyId: Codec,
		AccountId: Codec,
		Balance: Codec,
	{
		fn withdraw_list() -> BTreeMap<u64, (WithdrawItem<CurrencyId, AccountId, Balance>, Balance)>;
		fn pending_withdraw_list() -> BTreeMap<u64, (WithdrawItem<CurrencyId, AccountId, Balance>, Balance)>;
		fn try_get_current_strategy_unlocked_rewards() -> Balance;
		fn try_get_unlocked_rewards() -> (Balance, bool);
		fn try_calculate_price() -> Price;
	}
}
