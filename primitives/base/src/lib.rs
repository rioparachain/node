#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

pub mod traits;
pub mod types;

pub mod macros;

use sp_core::H160;
use sp_runtime::{
  generic,
  traits::{BlakeTwo256, IdentifyAccount, Verify},
  FixedU128, MultiAddress, MultiSignature, RuntimeDebug,
};
use sp_std::prelude::*;

/// The SignedExtension to the basic transaction logic.
pub type SignedExtra<Runtime> = (
  frame_system::CheckNonZeroSender<Runtime>,
  frame_system::CheckSpecVersion<Runtime>,
  frame_system::CheckTxVersion<Runtime>,
  frame_system::CheckGenesis<Runtime>,
  frame_system::CheckEra<Runtime>,
  frame_system::CheckNonce<Runtime>,
  frame_system::CheckWeight<Runtime>,
  pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic<Call, Runtime> =
  fp_self_contained::UncheckedExtrinsic<Address, Call, Signature, SignedExtra<Runtime>>;

//pub type UncheckedExtrinsic<Call, Runtime> =
//  generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra<Runtime>>;

/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic<Call, Runtime> =
  fp_self_contained::CheckedExtrinsic<AccountId, Call, SignedExtra<Runtime>, H160>;

//pub type CheckedExtrinsic<Call, Runtime> = generic::CheckedExtrinsic<AccountId, Call,
// SignedExtra<Runtime>>;

pub type Percent = sp_runtime::Percent;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of them.
pub type AccountIndex = u32;

/// Balance of an account.
pub type Balance = u128;

/// Type used for expressing timestamp.
pub type Moment = u64;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// A timestamp: milliseconds since the unix epoch.
/// `u64` is enough to represent a duration of half a billion years, when the
/// time scale is milliseconds.
pub type Timestamp = u64;

/// Digest item type.
pub type DigestItem = generic::DigestItem;

/// Header type.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type.
//pub type Block = generic::Block<Header, OpaqueExtrinsic>;
pub type Block<Call, Runtime> = generic::Block<Header, UncheckedExtrinsic<Call, Runtime>>;

/// Block ID.
pub type BlockId<Call, Runtime> = generic::BlockId<Block<Call, Runtime>>;

/// Signed version of Balance
pub type Amount = i128;

pub type CurrencyId = u32;

pub type Text = Vec<u8>;
pub type ChainAddress = Vec<u8>;
pub type Memo = Vec<u8>;
pub type Price = FixedU128;

/// The address format for describing accounts.
pub type Address = MultiAddress<AccountId, ()>;

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum DataProviderId {
  Aggregated = 0,
}

pub trait TruncCeilFloor<A> {
  fn trunc_to(self) -> A;
  fn ceil_to(self) -> A;
  fn floor_to(self) -> A;
}

impl TruncCeilFloor<u128> for FixedU128 {
  /// Returns the integer part.
  fn trunc_to(self) -> u128 {
    use sp_runtime::FixedPointNumber;
    self.into_inner().checked_div(Self::DIV).expect("panics only if DIV is zero, DIV is not zero; qed")
  }

  /// Returns the smallest integer greater than or equal to a number.
  ///
  /// Saturates to `Self::max` (truncated) if the result does not fit.
  fn ceil_to(self) -> u128 {
    use sp_runtime::{
      traits::{One, Saturating, Zero},
      FixedPointNumber,
    };
    if self.frac() == Self::zero() {
      self.trunc_to()
    } else {
      self.saturating_add(Self::one()).trunc_to()
    }
  }

  /// Returns the largest integer less than or equal to a number.
  ///
  /// Saturates to `Self::min` (truncated) if the result does not fit.
  fn floor_to(self) -> u128 { self.trunc_to() }
}
