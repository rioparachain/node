use std::collections::BTreeMap;
use std::sync::Arc;

use codec::Codec;
use serde::{Deserialize, Serialize};

use jsonrpsee::{
	core::{async_trait, Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorCode, ErrorObject},
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use rp_base::Price;

pub use rio_gateway_rpc_runtime_api::{
    GatewayApi as GatewayRuntimeApi, WithdrawItem as RuntimeWithdrawItem, WithdrawState,
};

pub struct Gateway<C, B> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<B>,
}

impl<C, B> Gateway<C, B> {
    /// Create new `Contracts` with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Gateway {
            client,
            _marker: Default::default(),
        }
    }
}

/// Error type of this RPC api.
pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::RuntimeError => 1,
			Error::DecodeError => 2,
		}
	}
}

#[rpc(client, server)]
pub trait GatewayApi<BlockHash, CurrencyId, AccountId, Balance> {
    #[method(name = "riogateway_withdrawList")]
    fn withdraw_list(
        &self,
        at: Option<BlockHash>,
    ) -> RpcResult<BTreeMap<u64, WithdrawItem<CurrencyId, AccountId, Balance>>>;

    #[method(name = "riogateway_pendingWithdrawList")]
    fn pending_withdraw_list(
        &self,
        at: Option<BlockHash>,
    ) -> RpcResult<BTreeMap<u64, WithdrawItem<CurrencyId, AccountId, Balance>>>;

    #[method(name = "riogateway_try_get_current_strategy_unlocked_rewards")]
    fn try_get_current_strategy_unlocked_rewards(
        &self,
        at: Option<BlockHash>,
    ) -> RpcResult<Balance>;

    #[method(name = "riogateway_try_get_unlocked_rewards")]
    fn try_get_unlocked_rewards(
        &self,
        at: Option<BlockHash>,
    ) ->  RpcResult<(Balance, bool)>;

    #[method(name = "riogateway_try_calculate_price")]
    fn try_calculate_price(
        &self,
        at: Option<BlockHash>,
    ) ->  RpcResult<Price>;
}

#[async_trait]
impl<C, Block, CurrencyId, AccountId, Balance>
    GatewayApiServer<<Block as BlockT>::Hash, CurrencyId, AccountId, Balance> for Gateway<C, Block>
where
    C: HeaderBackend<Block>,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: GatewayRuntimeApi<Block, CurrencyId, AccountId, Balance>,
    Block: BlockT,
    CurrencyId: Clone + std::fmt::Display + Codec,
    AccountId: Clone + std::fmt::Display + Codec,
    Balance: Clone + std::fmt::Display + Codec + ToString,
{
    fn withdraw_list(
        &self,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<BTreeMap<u64, WithdrawItem<CurrencyId, AccountId, Balance>>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));
        api.withdraw_list(&at)
            .map(|list| {
                list.into_iter()
                    .map(|(i, (item, fee))| (i, WithdrawItem::from_runtime_type(item, fee)))
                    .collect()
            })
			.map_err(|e| {
				CallError::Custom(ErrorObject::owned(
					Error::RuntimeError.into(),
					"Runtime trapped.",
					Some(e.to_string()),
				))
					.into()
			})
    }

    fn pending_withdraw_list(
        &self,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<BTreeMap<u64, WithdrawItem<CurrencyId, AccountId, Balance>>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));
        api.pending_withdraw_list(&at)
            .map(|list| {
                list.into_iter()
                    .map(|(i, (item, fee))| (i, WithdrawItem::from_runtime_type(item, fee)))
                    .collect()
            })
			.map_err(|e| {
				CallError::Custom(ErrorObject::owned(
					Error::RuntimeError.into(),
					"Runtime trapped.",
					Some(e.to_string()),
				))
					.into()
			})
    }

    fn try_get_current_strategy_unlocked_rewards(
        &self,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<Balance> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));
        api.try_get_current_strategy_unlocked_rewards(&at)
			.map_err(|e| {
				CallError::Custom(ErrorObject::owned(
					Error::RuntimeError.into(),
					"Runtime trapped.",
					Some(e.to_string()),
				))
					.into()
			})
    }

    fn try_get_unlocked_rewards(
        &self,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<(Balance, bool)> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));
        api.try_get_unlocked_rewards(&at)
			.map_err(|e| {
				CallError::Custom(ErrorObject::owned(
					Error::RuntimeError.into(),
					"Runtime trapped.",
					Some(e.to_string()),
				))
					.into()
			})
    }

    fn try_calculate_price(
        &self,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<Price> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));
        api.try_calculate_price(&at)
			.map_err(|e| {
				CallError::Custom(ErrorObject::owned(
					Error::RuntimeError.into(),
					"Runtime trapped.",
					Some(e.to_string()),
				))
					.into()
			})
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawItem<CurrencyId, AccountId, Balance> {
    pub currency_id: CurrencyId,
    pub applicant: AccountId,
    pub value: Balance,
    pub addr: String,
    pub memo: String,
    pub state: WithdrawState,
    pub fee: Balance,
}

pub fn try_hex_or_str(src: &[u8]) -> String {
    let should_as_string = src.iter().try_for_each(|c| {
        if b'!' <= *c && *c <= b'~' {
            Ok(())
        } else {
            Err(())
        }
    });
    if should_as_string.is_ok() {
        to_string(src)
    } else {
        to_hex(src)
    }
}

#[inline]
fn to_hex(s: &[u8]) -> String {
    format!("0x{}", hex::encode(s))
}

#[inline]
fn to_string(s: &[u8]) -> String {
    String::from_utf8_lossy(s).into_owned()
}

impl<CurrencyId, AccountId, Balance> WithdrawItem<CurrencyId, AccountId, Balance> {
    fn from_runtime_type(
        item: RuntimeWithdrawItem<CurrencyId, AccountId, Balance>,
        fee: Balance,
    ) -> Self {
        WithdrawItem {
            currency_id: item.currency_id,
            applicant: item.applicant,
            value: item.value,
            addr: try_hex_or_str(&item.addr),
            memo: to_string(&item.memo),
            state: item.state,
            fee,
        }
    }
}
