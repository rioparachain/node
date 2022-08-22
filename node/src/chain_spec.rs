use cumulus_primitives_core::ParaId;
use frame_support::{traits::ConstU32, BoundedVec};
use parachain_rio_runtime::{
	AccountId, AssetInfo, AuraId, Balance, CurrencyId, EVMConfig, EthereumConfig, RioAssetsConfig,
	RioGatewayConfig, Signature, Text, EXISTENTIAL_DEPOSIT,
};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::{collections::BTreeMap, convert::TryFrom, str::FromStr};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<parachain_rio_runtime::GenesisConfig, Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// Helper function to generate a crypto pair from seed
pub fn get_public_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
	get_public_from_seed::<AuraId>(seed)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_public_from_seed::<TPublic>(seed)).into_account()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn parachain_rio_session_keys(keys: AuraId) -> parachain_rio_runtime::SessionKeys {
	parachain_rio_runtime::SessionKeys { aura: keys }
}

pub fn development_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "UNIT".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
					),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				1000.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		None,
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: 1000,
		},
	)
}

macro_rules! bvec {
	($a:expr) => {
		Text::try_from($a.to_vec()).unwrap()
	};
}
fn assets_init(
) -> Vec<(CurrencyId, AssetInfo, rpallet_assets::Restrictions, Vec<(AccountId, Balance)>)> {
	use rp_protocol as rp;
	use rpallet_assets::{Chain, Restriction, Restrictions};
	vec![
		// asset id defined in protocol
		(
			CurrencyId::from(rp::LOCKED_RFUEL),
			AssetInfo {
				symbol: bvec!(b"LOCKED_RFUEL"),
				name: bvec!(b"Locked Rio Fuel Token"),
				decimals: 12,
				desc: bvec!(b"Locked Rio Fuel Token"),
				chain: Chain::Rio,
			},
			Restriction::Transferable.into(),
			vec![],
		),
		(
			CurrencyId::from(rp::OM),
			AssetInfo {
				symbol: bvec!(b"OM"),
				name: bvec!(b"MANTRA DAO Token"),
				decimals: 12,
				desc: bvec!(b"MANTRA DAO Token"),
				chain: Chain::Rio,
			},
			Restrictions::none(),
			vec![],
		),
		(
			CurrencyId::from(rp::RBTC),
			AssetInfo {
				symbol: bvec!(b"RBTC"),
				name: bvec!(b"RBTC token"),
				decimals: 8,
				desc: bvec!(b"Bitcoin in RioChain"),
				chain: Chain::Bitcoin,
			},
			Restrictions::none(),
			vec![],
		),
		(
			CurrencyId::from(rp::RLTC),
			AssetInfo {
				symbol: bvec!(b"RLTC"),
				name: bvec!(b"RLTC token"),
				decimals: 8,
				desc: bvec!(b"Litecoin in RioChain"),
				chain: Chain::Litecoin,
			},
			Restrictions::none(),
			vec![],
		),
		(
			CurrencyId::from(rp::RETH),
			AssetInfo {
				symbol: bvec!(b"RETH"),
				name: bvec!(b"RETH token"),
				decimals: 18,
				desc: bvec!(b"Ether in RioChain"),
				chain: Chain::Ethereum,
			},
			Restrictions::none(),
			vec![],
		),
		(
			CurrencyId::from(rp::RUSDT),
			AssetInfo {
				symbol: bvec!(b"RUSDT"),
				name: bvec!(b"RUSDT token"),
				decimals: 6,
				desc: bvec!(b"USDT in RioChain"),
				chain: Chain::Ethereum,
			},
			Restrictions::none(),
			vec![],
		),
		(
			CurrencyId::from(rp::STAKING_POOL_MARKER),
			AssetInfo {
				symbol: bvec!(b"SPM"),
				name: bvec!(b"Staking pool marker"),
				decimals: 12,
				desc: bvec!(b"Staking pool marker"),
				chain: Chain::Rio,
			},
			Restrictions::none(),
			vec![],
		),
	]
}

pub fn local_testnet_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "UNIT".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
					),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				1000.into(),
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("parachain-rio-local"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: 1000,
		},
	)
}

pub type TextL = BoundedVec<u8, ConstU32<128>>;
macro_rules! bvec {
	($a:expr) => {
		TextL::try_from($a.to_vec()).unwrap()
	};
}

fn testnet_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> parachain_rio_runtime::GenesisConfig {
	parachain_rio_runtime::GenesisConfig {
		system: parachain_rio_runtime::SystemConfig {
			code: parachain_rio_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: parachain_rio_runtime::BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		parachain_info: parachain_rio_runtime::ParachainInfoConfig { parachain_id: id },
		collator_selection: parachain_rio_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
			..Default::default()
		},
		session: parachain_rio_runtime::SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                      // account id
						acc,                              // validator id
						parachain_rio_session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		polkadot_xcm: parachain_rio_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
		},
		evm: EVMConfig { accounts: BTreeMap::new() },
		ethereum: EthereumConfig {},
		dynamic_fee: Default::default(),
		base_fee: Default::default(),
		rio_gateway: RioGatewayConfig {
			max_deposit_index: 10000,
			initial_supported_currencies: vec![
				(CurrencyId::from(rp_protocol::RFUEL), 10 * 1_000_000_000_000_000_000), // 10 RFUEL
				(CurrencyId::from(rp_protocol::OM), 10 * 1_000_000_000_000_000_000),
				(CurrencyId::from(rp_protocol::RBTC), 5 * 100_000),  // 0.005 BTC
				(CurrencyId::from(rp_protocol::RETH), 5 * 1_000_000_000_000_000_000),  // 0.05 ETH
				(CurrencyId::from(rp_protocol::RUSDT), 5 * 1_000_000), // 5 USDT
			],
			deposit_addr_info: vec![(
				CurrencyId::from(rp_protocol::RBTC),
				rpallet_gateway::DepositAddrInfo::Bip32(
					rpallet_gateway::Bip32 {
						x_pub: bvec!(b"upub5DRdTWfz3NeZwd25HeQ2xMNjnYtYRfZzC6fEDjmPH2AwnxjvTrySjVApEiDufv68gqsZ7TCUcNfb1P4KLjNvZCTsPCaVb68SLedQwPKMLKR"),
						path: bvec!(b"m/49'/1'/0")
					}
				)
			)],
			admins: vec![(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				rpallet_gateway::Auths::all(),
			)],
		},
		rio_assets: RioAssetsConfig { init: assets_init() },
	}
}
