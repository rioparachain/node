#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

mod types;
mod weight_info;

use codec::MaxEncodedLen;
#[allow(unused_imports)]
use codec::{Decode, Encode, Error as codecErr, HasCompact, Input, Output};
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
	pallet_prelude::*, IterableStorageMap,
};
use frame_system::{ensure_root, ensure_signed, pallet_prelude::*};
use orml_traits::{MultiCurrency, MultiReservableCurrency};
pub use pallet::*;
use rp_base::{BBVec, ChainAddress, Memo};
use rp_support::{debug, error, info, warn};
use scale_info::TypeInfo;
use sp_runtime::traits::StaticLookup;
use sp_std::{collections::btree_map::BTreeMap, convert::TryInto, prelude::*};
pub use weight_info::WeightInfo;

use crate::types::WithdrawPhase;
pub use crate::types::{
	Auth, Auths, Bip32, Create2, Deposit, DepositAddrInfo, TxHash, WithdrawInfo, WithdrawItem,
	WithdrawState,
};

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type Currency: MultiCurrency<Self::AccountId> + MultiReservableCurrency<Self::AccountId>;

		type WeightInfo: WeightInfo;
	}

	/// Error for the gateway module.
	#[pallet::error]
	pub enum Error<T> {
		/// already set this asset before
		AssetExisted,
		/// not supported asset
		AssetNotSupported,
		/// UnAuthorized Operation
		UnAuthorized,
		/// Repeated transaction
		TransactionRepeated,
		/// Pending withdraw not found
		WithdrawalRecordNotExisted,
		/// The previously withdraw record is an invalid withdraw state
		InvalidWithdrawalState,
		/// not owner for this withdraw
		CanNotCancelOtherWithdrawals,
		/// already applied for deposit path index
		AlreadyAppliedIndex,
		/// can't assign an index now.
		CanNotAssignIndex,
		/// apply an invalid withdraw
		InvalidWithdraw,
	}

	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type BalanceOf<T> =
		<<T as Config>::Currency as MultiCurrency<<T as frame_system::Config>::AccountId>>::Balance;
	pub type CurrencyIdOf<T> = <<T as Config>::Currency as MultiCurrency<
		<T as frame_system::Config>::AccountId,
	>>::CurrencyId;
	pub type DepositOf<T> = Deposit<<T as frame_system::Config>::AccountId, BalanceOf<T>>;
	pub type WithdrawInfoOf<T> =
		WithdrawInfo<CurrencyIdOf<T>, <T as frame_system::Config>::AccountId, BalanceOf<T>>;
	pub type DAddrInfo = DepositAddrInfo<BBVec>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config> {
		AuthChanged(AccountIdOf<T>, Auths),
		SupportedAssetAdded(AccountIdOf<T>, CurrencyIdOf<T>, BalanceOf<T>),
		SupportedAssetRemoved(AccountIdOf<T>, CurrencyIdOf<T>),
		WithdrawaFeeSetted(AccountIdOf<T>, CurrencyIdOf<T>, BalanceOf<T>),
		NewDepositAddrInfoOfAssetId(CurrencyIdOf<T>, DAddrInfo),
		NewDepositIndex(AccountIdOf<T>, u64),
		MaxDepositCountSetted(u64),
		NewDepositRecord(CurrencyIdOf<T>, DepositOf<T>, TxHash),
		NewPendingWithdrawRecord(u64, WithdrawInfoOf<T>, BalanceOf<T>),
		WithdrawRebroadcasted(u64, AccountIdOf<T>, WithdrawState),
		WithdrawStatusChanged(u64, AccountIdOf<T>, WithdrawState, WithdrawState),
		UnsafeSetWithdrawState(u64, WithdrawState),
		UnsafeRemoveWithdrawRecord(u64),
	}

	#[pallet::storage]
	#[pallet::getter(fn supported_assets)]
	pub type SupportedAssets<T: Config> =
		StorageMap<_, Twox64Concat, CurrencyIdOf<T>, bool, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn admins)]
	pub type Admins<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Auths, ValueQuery>;

	// deposit address
	/// Store gateway deposit addr basic info for an asset.
	#[pallet::storage]
	#[pallet::getter(fn deposit_addr_info_of_asset_id)]
	pub type DepositAddrInfoOfAssetId<T: Config> =
		StorageMap<_, Twox64Concat, CurrencyIdOf<T>, DAddrInfo, OptionQuery>;

	/// Next deposit index
	#[pallet::storage]
	#[pallet::getter(fn next_deposit_index)]
	pub type NextDepositIndex<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// map of bip32 path index of an account. if this account have not apply deposit, it would be None
	#[pallet::storage]
	#[pallet::getter(fn deposit_index_of_account_id)]
	pub type DepoistIndexOfAccountId<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, u64, OptionQuery>;

	/// Current max deposit index, if more than this count, would return error for user.
	#[pallet::storage]
	#[pallet::getter(fn max_deposit_index)]
	pub type MaxDepositIndex<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// keep a history of depoists in case of double spent
	#[pallet::storage]
	#[pallet::getter(fn deposit_history)]
	pub type DepositHistory<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		CurrencyIdOf<T>,
		Identity,
		TxHash,
		Deposit<T::AccountId, BalanceOf<T>>,
		OptionQuery,
	>;

	/// set a fixed withdrawal fee for a asset
	#[pallet::storage]
	#[pallet::getter(fn withdrawal_fee)]
	pub type WithdrawalFee<T: Config> =
		StorageMap<_, Twox64Concat, CurrencyIdOf<T>, BalanceOf<T>, ValueQuery>;

	/// Withdrawal Id
	#[pallet::storage]
	#[pallet::getter(fn next_withdrawal_id)]
	pub type NextWithdrawalId<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// after withdraw req is fired, it will be append here first, waiting for approval
	#[pallet::storage]
	#[pallet::getter(fn pending_withdraws)]
	pub type PendingWithdrawals<T: Config> = StorageMap<
		_,
		Twox64Concat,
		u64,
		WithdrawInfo<CurrencyIdOf<T>, T::AccountId, BalanceOf<T>>,
		OptionQuery,
	>;

	/// Consumed Fee for every Withdrawal Id
	#[pallet::storage]
	#[pallet::getter(fn consumed_fee)]
	pub type ConsumedFee<T: Config> = StorageMap<_, Twox64Concat, u64, BalanceOf<T>, ValueQuery>;

	/// withdrawal status for an id
	#[pallet::storage]
	#[pallet::getter(fn active_withdrawal_states)]
	pub type ActiveWithdrawStates<T: Config> =
		StorageMap<_, Twox64Concat, u64, WithdrawState, OptionQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub initial_supported_currencies: Vec<(CurrencyIdOf<T>, BalanceOf<T>)>,
		pub deposit_addr_info: Vec<(CurrencyIdOf<T>, DAddrInfo)>,
		pub admins: Vec<(T::AccountId, Auths)>,
		pub max_deposit_index: u64,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			GenesisConfig {
				initial_supported_currencies: vec![],
				deposit_addr_info: vec![],
				admins: vec![],
				max_deposit_index: 1000,
			}
		}
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			for (currency_id, withdraw_fee) in self.initial_supported_currencies.iter() {
				SupportedAssets::<T>::insert(currency_id, true);
				WithdrawalFee::<T>::insert(currency_id, withdraw_fee);
			}
			for (currency_id, info) in self.deposit_addr_info.iter() {
				DepositAddrInfoOfAssetId::<T>::insert(currency_id, info);
			}
			for (admin, auths) in self.admins.iter() {
				Admins::<T>::insert(admin, auths);
			}
			MaxDepositIndex::<T>::put(self.max_deposit_index);
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::set_auth())]
		pub fn set_auth(
			origin: OriginFor<T>,
			who: <T::Lookup as StaticLookup>::Source,
			auths: Auths,
		) -> DispatchResult {
			ensure_root(origin)?;
			let who = T::Lookup::lookup(who)?;
			Admins::<T>::insert(who.clone(), auths);
			Self::deposit_event(Event::AuthChanged(who, auths));
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::add_supported_asset())]
		pub fn add_supported_asset(
			origin: OriginFor<T>,
			currency_id: CurrencyIdOf<T>,
			#[pallet::compact] withdrawal_fee: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Self::admins(&who).contains(Auth::Register), Error::<T>::UnAuthorized);
			ensure!(!Self::supported_assets(currency_id), Error::<T>::AssetExisted);

			SupportedAssets::<T>::insert(currency_id, true);
			WithdrawalFee::<T>::insert(currency_id, withdrawal_fee);
			Self::deposit_event(Event::SupportedAssetAdded(who, currency_id, withdrawal_fee));

			Ok(())
		}

		#[pallet::weight(T::WeightInfo::remove_supported_asset())]
		pub fn remove_supported_asset(
			origin: OriginFor<T>,
			currency_id: CurrencyIdOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Self::admins(&who).contains(Auth::Register), Error::<T>::UnAuthorized);
			SupportedAssets::<T>::remove(currency_id);
			WithdrawalFee::<T>::remove(currency_id);

			Self::deposit_event(Event::SupportedAssetRemoved(who, currency_id));
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::set_bip32_info())]
		pub fn set_deposit_addr_info_of_asset_id(
			origin: OriginFor<T>,
			currency_id: CurrencyIdOf<T>,
			addr_info: DepositAddrInfo<BBVec>,
		) -> DispatchResult {
			ensure_root(origin)?;
			DepositAddrInfoOfAssetId::<T>::insert(currency_id, addr_info.clone());
			Self::deposit_event(Event::NewDepositAddrInfoOfAssetId(currency_id, addr_info));
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::set_withdrawal_fee())]
		pub fn set_withdrawal_fee(
			origin: OriginFor<T>,
			currency_id: CurrencyIdOf<T>,
			#[pallet::compact] withdrawal_fee: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Self::admins(&who).contains(Auth::Register), Error::<T>::UnAuthorized);
			ensure!(Self::supported_assets(currency_id), Error::<T>::AssetNotSupported);

			WithdrawalFee::<T>::insert(currency_id, withdrawal_fee);

			Self::deposit_event(Event::WithdrawaFeeSetted(who, currency_id, withdrawal_fee));
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::apply_deposit_address())]
		pub fn apply_deposit_index(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(
				Self::deposit_index_of_account_id(&who).is_none(),
				Error::<T>::AlreadyAppliedIndex
			);

			let index = Self::next_deposit_index();
			let max = Self::max_deposit_index();

			let next_index = index + 1;

			ensure!(next_index <= max, Error::<T>::CanNotAssignIndex);

			DepoistIndexOfAccountId::<T>::insert(who.clone(), index);
			Self::deposit_event(Event::NewDepositIndex(who.clone(), index));

			NextDepositIndex::<T>::put(next_index);
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::set_max_deposit_index())]
		pub fn set_max_deposit_index(origin: OriginFor<T>, new_count: u64) -> DispatchResult {
			ensure_root(origin)?;
			MaxDepositIndex::<T>::try_mutate(|old| -> DispatchResult {
				ensure!(*old < new_count, "new_count must larger then current");
				*old = new_count;
				Self::deposit_event(Event::MaxDepositCountSetted(new_count));
				Ok(())
			})?;
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::deposit())]
		pub fn deposit(
			origin: OriginFor<T>,
			depositor: <T::Lookup as StaticLookup>::Source,
			currency_id: CurrencyIdOf<T>,
			tx_hash: TxHash,
			#[pallet::compact] value: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let depositor = T::Lookup::lookup(depositor)?;

			ensure!(Self::admins(&who).contains(Auth::Deposit), Error::<T>::UnAuthorized);
			ensure!(Self::supported_assets(currency_id), Error::<T>::AssetNotSupported);
			ensure!(
				Self::deposit_history(&currency_id, &tx_hash).is_none(),
				Error::<T>::TransactionRepeated
			);

			let deposit_record = Deposit { account_id: depositor.clone(), amount: value };
			T::Currency::deposit(currency_id, &depositor, value)?;
			DepositHistory::<T>::insert(currency_id, tx_hash, deposit_record.clone());

			Self::deposit_event(Event::NewDepositRecord(currency_id, deposit_record, tx_hash));
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::withdraw())]
		pub fn request_withdraw(
			origin: OriginFor<T>,
			currency_id: CurrencyIdOf<T>,
			#[pallet::compact] value: BalanceOf<T>,
			addr: ChainAddress,
			memo: Memo,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Self::supported_assets(currency_id), Error::<T>::AssetNotSupported);

			let info = WithdrawInfo { currency_id, who: who.clone(), value, addr, memo };
			Self::withdraw_check(&info)?;

			let fee = Self::withdrawal_fee(currency_id);
			let real_value = value + fee;
			T::Currency::reserve(currency_id, &who, real_value)?;

			let next_id = Self::next_withdrawal_id();
			// record fee for this withdraw
			ConsumedFee::<T>::insert(next_id, fee);
			PendingWithdrawals::<T>::insert(next_id, info.clone());
			ActiveWithdrawStates::<T>::insert(next_id, WithdrawState::Pending);

			info!(
				"withdraw apply|who:{:?}|currency_id:{:?}|value:{:?}|withdraw_id:{:?}",
				who, currency_id, value, next_id
			);

			NextWithdrawalId::<T>::put(next(next_id));

			Self::deposit_event(Event::NewPendingWithdrawRecord(next_id, info, fee));
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::revoke_withdraw())]
		pub fn cancel_withdraw(
			origin: OriginFor<T>,
			#[pallet::compact] withdraw_id: u64,
		) -> DispatchResult {
			let operator = ensure_signed(origin)?;

			Self::handle_withdraw(withdraw_id, &operator, WithdrawPhase::First, |state, info| {
				if info.who != operator {
					error!(
						"revoke failed for applicant not current account|applicant:{:?}|now:{:?}",
						info.who, operator
					);
					Err(Error::<T>::CanNotCancelOtherWithdrawals)?
				}
				info!(
					"cancel withdraw success|withdraw_id:{:?}|who:{:?}|currency_id:{:?}|value:{:?}",
					withdraw_id, info.who, info.currency_id, info.value
				);
				Self::refund(withdraw_id, &info);
				*state = WithdrawState::Cancelled;
				Ok(())
			})
		}

		#[pallet::weight(T::WeightInfo::reject_withdraw())]
		pub fn reject_withdraw(
			origin: OriginFor<T>,
			#[pallet::compact] withdraw_id: u64,
		) -> DispatchResult {
			let operator = ensure_signed(origin)?;
			ensure!(Self::admins(&operator).contains(Auth::Withdraw), Error::<T>::UnAuthorized);

			Self::handle_withdraw(withdraw_id, &operator, WithdrawPhase::First, |state, info| {
				info!("reject withdraw|withdraw_id:{:?}|admin:{:?}", withdraw_id, operator);
				Self::refund(withdraw_id, &info);
				*state = WithdrawState::Rejected;
				Ok(())
			})
		}

		#[pallet::weight(T::WeightInfo::approve_withdraw())]
		pub fn approve_withdraw(
			origin: OriginFor<T>,
			#[pallet::compact] withdraw_id: u64,
		) -> DispatchResult {
			let operator = ensure_signed(origin)?;
			ensure!(Self::admins(&operator).contains(Auth::Withdraw), Error::<T>::UnAuthorized);

			Self::handle_withdraw(withdraw_id, &operator, WithdrawPhase::First, |state, _info| {
				info!("approve withdraw|withdraw_id:{:?}|admin:{:?}", withdraw_id, operator);
				*state = WithdrawState::Approved;
				Ok(())
			})
		}

		#[pallet::weight(T::WeightInfo::withdraw_finish())]
		pub fn finish_withdraw(
			origin: OriginFor<T>,
			#[pallet::compact] withdraw_id: u64,
			tx_hash: TxHash,
		) -> DispatchResult {
			let operator = ensure_signed(origin)?;
			ensure!(Self::admins(&operator).contains(Auth::Withdraw), Error::<T>::UnAuthorized);

			Self::handle_withdraw(withdraw_id, &operator, WithdrawPhase::Second, |state, info| {
				info!(
                    "finish withdraw|withdraw_id:{:?}|admin:{:?}|txhash:{:?}|who:{:?}|currency_id:{:?}|value:{:?}",
                    withdraw_id, operator, tx_hash, info.who, info.currency_id, info.value,
                );
				Self::burn(withdraw_id, info.currency_id, &info.who, info.value);
				*state = WithdrawState::Success(tx_hash);
				Ok(())
			})
		}

		#[pallet::weight(T::WeightInfo::rebroadcast())]
		pub fn rebroadcast(
			origin: OriginFor<T>,
			#[pallet::compact] withdraw_id: u64,
			tx_hash: TxHash,
		) -> DispatchResult {
			let operator = ensure_signed(origin)?;
			ensure!(Self::admins(&operator).contains(Auth::Withdraw), Error::<T>::UnAuthorized);
			warn!(
				"do re-broadcast and log tx_hash|admin:{:?}|withdraw_id:{:?}|tx_hash:{:?}",
				operator, withdraw_id, tx_hash
			);
			let state = WithdrawState::ReBroadcasted(tx_hash);
			Self::deposit_event(Event::WithdrawRebroadcasted(withdraw_id, operator, state));
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::modify_withdraw_state())]
		pub fn unsafe_set_withdraw_state(
			origin: OriginFor<T>,
			#[pallet::compact] withdraw_id: u64,
			state: Option<WithdrawState>,
		) -> DispatchResult {
			let operator = ensure_signed(origin)?;
			ensure!(Self::admins(&operator).contains(Auth::Sudo), Error::<T>::UnAuthorized);
			if let Some(s) = state {
				ActiveWithdrawStates::<T>::insert(withdraw_id, s.clone());
				Self::deposit_event(Event::UnsafeSetWithdrawState(withdraw_id, s));
			} else {
				ActiveWithdrawStates::<T>::remove(withdraw_id);
				Self::deposit_event(Event::UnsafeRemoveWithdrawRecord(withdraw_id));
			}
			Ok(())
		}
	}
}

#[inline]
fn next(current: u64) -> u64 {
	match current.checked_add(1) {
		Some(r) => r,
		None => 0,
	}
}

impl<T: Config> Pallet<T> {
	// Handles withdraw at all phases & with all states
	//
	// Phases:
	// - First: withdraw request created and have a Pending state.
	// - Second: request is confirmed, finished and will be marked as succesful.
	//
	// States:
	// - Pending: Created, waiting to be approved/rejected/cancelled by admin.
	// Currency moved from 'free' balance to 'reserved'.
	// - Cancelled: Cancelled by admin. Currency moved from 'reserved' to 'free'.
	// - Rejected: Rejected by admin. Currency moved from 'reserved' to 'free'.
	// - Approved: Approved by admin.
	// - Success(TxHash): Successfully created, approved and finished operation.
	// - ReBroadcasted(TxHash): Rebroadcast initiated to keep valid transaction id on the chain.
	fn handle_withdraw(
		withdraw_id: u64,
		operator: &T::AccountId,
		phase: WithdrawPhase,
		mut handle: impl FnMut(
			&mut WithdrawState,
			WithdrawInfo<CurrencyIdOf<T>, T::AccountId, BalanceOf<T>>,
		) -> DispatchResult,
	) -> DispatchResult {
		// Notice: Can not change status before use handle function
		let info = Self::pending_withdraws(&withdraw_id).ok_or_else(|| {
			error!("withdraw not exist for id:{:?}", withdraw_id);
			"withdraw not exist"
		})?;
		let mut state = ActiveWithdrawStates::<T>::get(&withdraw_id)
			.ok_or(Error::<T>::WithdrawalRecordNotExisted)?;
		let old_state = state.clone();

		debug!(
			"handle withdraw|withdraw_id:{:?}|before_state:{:?}|phase:{:?}",
			withdraw_id, state, phase
		);

		match phase {
			WithdrawPhase::First =>
				if state != WithdrawState::Pending {
					error!("handle withdraw|just allow `Pending` state in withdraw phase 1|withdraw_id:{:?}|state:{:?}", withdraw_id, state);
					Err(Error::<T>::InvalidWithdrawalState)?;
				},
			WithdrawPhase::Second => match state {
				WithdrawState::Approved => {},
				_ => {
					error!("handle withdraw|just allow `Approve` state in withdraw phase 2|withdraw_id:{:?}|state:{:?}", withdraw_id, state);
					Err(Error::<T>::InvalidWithdrawalState)?;
				},
			},
		}

		handle(&mut state, info)?;

		debug!("handle withdraw finish|withdraw_id:{:?}|current_state:{:?}", withdraw_id, state);
		// state may be changed
		// translate state
		match state {
			WithdrawState::Cancelled | WithdrawState::Rejected | WithdrawState::Success(_) => {
				PendingWithdrawals::<T>::remove(withdraw_id);
				// remove recorded fee for this withdraw id
				ConsumedFee::<T>::remove(withdraw_id);
				ActiveWithdrawStates::<T>::remove(withdraw_id);
			},
			WithdrawState::Approved => {
				ActiveWithdrawStates::<T>::insert(withdraw_id, state.clone());
			},
			_ => unreachable!("not expected withdraw state"),
		}
		Self::deposit_event(Event::WithdrawStatusChanged(
			withdraw_id,
			operator.clone(),
			old_state,
			state,
		));
		Ok(())
	}

	fn refund(withdraw_id: u64, info: &WithdrawInfo<CurrencyIdOf<T>, T::AccountId, BalanceOf<T>>) {
		let fee = Self::consumed_fee(withdraw_id);
		debug!(
			"withdraw refund|currency_id:{:?}|who:{:?}|sub:{:?}|fee:{:?}",
			info.currency_id, info.who, info.value, fee
		);
		T::Currency::unreserve(info.currency_id, &info.who, info.value + fee);
	}

	fn burn(
		withdraw_id: u64,
		currency_id: CurrencyIdOf<T>,
		who: &T::AccountId,
		value: BalanceOf<T>,
	) {
		let fee = Self::consumed_fee(withdraw_id);
		// asset should be burnt
		debug!(
			"withdraw burn|currency_id:{:?}|who:{:?}|value:{:?}|fee:{:?}",
			currency_id, who, value, fee
		);
		T::Currency::slash_reserved(currency_id, who, value + fee);
	}

	fn withdraw_check(
		info: &WithdrawInfo<CurrencyIdOf<T>, T::AccountId, BalanceOf<T>>,
	) -> DispatchResult {
		// todo check by chain in future
		if info.addr.len() > 80 {
			Err(Error::<T>::InvalidWithdraw)?
		}
		if info.memo.len() > 80 {
			Err(Error::<T>::InvalidWithdraw)?
		}
		Ok(())
	}
}

impl<T: Config> Pallet<T> {
	pub fn withdraw_list(
	) -> BTreeMap<u64, (WithdrawItem<CurrencyIdOf<T>, T::AccountId, BalanceOf<T>>, BalanceOf<T>)> {
		PendingWithdrawals::<T>::iter()
			.map(|(index, info)| {
				let state =
					ActiveWithdrawStates::<T>::get(index).unwrap_or(WithdrawState::Cancelled);
				(
					index,
					(
						WithdrawItem {
							currency_id: info.currency_id,
							applicant: info.who,
							value: info.value,
							addr: info.addr,
							memo: info.memo,
							state,
						},
						Self::consumed_fee(index),
					),
				)
			})
			.collect()
	}

	pub fn pending_withdraw_list(
	) -> BTreeMap<u64, (WithdrawItem<CurrencyIdOf<T>, T::AccountId, BalanceOf<T>>, BalanceOf<T>)> {
		Self::withdraw_list()
			.into_iter()
			.filter(
				|(_index, item)| {
					if let WithdrawState::Pending = item.0.state {
						true
					} else {
						false
					}
				},
			)
			.collect()
	}
}
