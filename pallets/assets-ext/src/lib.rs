#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::BoundedVec;
use sp_runtime::DispatchResult;

use frame_system::pallet_prelude::*;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use rpallet_assets::{AssetInfo, Restrictions, WeightInfo};

	#[pallet::config]
	pub trait Config: frame_system::Config + rpallet_assets::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// create a new asset with full permissions granted to whoever make the call
		/// *sudo or proposal approved only*
		#[pallet::weight(T::WeightInfo::create())]
		pub fn create(
			origin: OriginFor<T>,
			currency_id: T::CurrencyId,
			asset_info: AssetInfo<BoundedVec<u8, T::StringLimit>>,
		) -> DispatchResult {
			rpallet_assets::Pallet::<T>::create(origin, currency_id, asset_info)?;
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::update_asset_info())]
		pub fn update_asset_info(
			origin: OriginFor<T>,
			currency_id: T::CurrencyId,
			asset_info: AssetInfo<BoundedVec<u8, T::StringLimit>>,
		) -> DispatchResult {
			rpallet_assets::Pallet::<T>::update_asset_info(origin, currency_id, asset_info)?;
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::update_restriction())]
		pub fn update_restriction(
			origin: OriginFor<T>,
			currency_id: T::CurrencyId,
			restrictions: Restrictions,
		) -> DispatchResult {
			rpallet_assets::Pallet::<T>::update_restriction(origin, currency_id, restrictions)?;
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::offline_asset())]
		pub fn offline_asset(origin: OriginFor<T>, currency_id: T::CurrencyId) -> DispatchResult {
			rpallet_assets::Pallet::<T>::offline_asset(origin, currency_id)?;
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::online_asset())]
		pub fn online_asset(origin: OriginFor<T>, currency_id: T::CurrencyId) -> DispatchResult {
			rpallet_assets::Pallet::<T>::online_asset(origin, currency_id)?;
			Ok(())
		}
	}
}
