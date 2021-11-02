use core::str::FromStr;

use appchain_deip_runtime::{
	AccountId, BabeConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SudoConfig,
	SystemConfig, WASM_BINARY, DeipConfig,
	DeipAssetsConfig,
    DeipProposalConfig,
    DeipDaoConfig,
    DeipPortalConfig,
};
use sc_service::ChainType;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

use appchain_deip_runtime::BeefyConfig;
use appchain_deip_runtime::{
	opaque::SessionKeys, Balance, ImOnlineConfig, SessionConfig, DOLLARS,
};
use appchain_deip_runtime::{OctopusAppchainConfig, OctopusLposConfig};
use beefy_primitives::crypto::AuthorityId as BeefyId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_octopus_appchain::AuthorityId as OctopusId;
use sp_consensus_babe::AuthorityId as BabeId;

use pallet_deip::{DomainId, Domain};
use pallet_deip_assets::SerializableAssetBalance;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

fn session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	beefy: BeefyId,
	octopus: OctopusId,
) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online, beefy, octopus }
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(
	s: &str,
) -> (AccountId, BabeId, GrandpaId, ImOnlineId, BeefyId, OctopusId) {
	(
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<BabeId>(s),
		get_from_seed::<GrandpaId>(s),
		get_from_seed::<ImOnlineId>(s),
		get_from_seed::<BeefyId>(s),
		get_from_seed::<OctopusId>(s),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				Some(vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
				]),
                vec![
					DomainId::from_str("6c4bb3bcf1a88e3b51de88576d592f1f980c5bbb").unwrap(), // Common
					DomainId::from_str("7c3d37cbfea2513a7e03e674448bbeee8ae3d862").unwrap(), // Biology
					DomainId::from_str("9f0224709d86e02b9625b5ebf2786b80ba6bed17").unwrap(), // Physics
					DomainId::from_str("6a8b20f002a7dedf7b873dbc86e0b0051d4fa898").unwrap(), // Chemistry
					DomainId::from_str("a47bf84ac30d0843accb737d5924434ef3ed0517").unwrap(), // Earth sciences
					DomainId::from_str("8e2a3711649993a87848337b9b401dcf64425e2d").unwrap(), // Space sciences
					DomainId::from_str("721e75eb0535e152669b0c3fbbb9e21675483553").unwrap(), // Medicine and health
					DomainId::from_str("2519ef55e1b69f1a7e13275e3273950cce7e26a8").unwrap(), // Aquaculture
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				Some(vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				]),
				vec![
					DomainId::from_str("6c4bb3bcf1a88e3b51de88576d592f1f980c5bbb").unwrap(), // Common
					DomainId::from_str("7c3d37cbfea2513a7e03e674448bbeee8ae3d862").unwrap(), // Biology
					DomainId::from_str("9f0224709d86e02b9625b5ebf2786b80ba6bed17").unwrap(), // Physics
					DomainId::from_str("6a8b20f002a7dedf7b873dbc86e0b0051d4fa898").unwrap(), // Chemistry
					DomainId::from_str("a47bf84ac30d0843accb737d5924434ef3ed0517").unwrap(), // Earth sciences
					DomainId::from_str("8e2a3711649993a87848337b9b401dcf64425e2d").unwrap(), // Space sciences
					DomainId::from_str("721e75eb0535e152669b0c3fbbb9e21675483553").unwrap(), // Medicine and health
					DomainId::from_str("2519ef55e1b69f1a7e13275e3273950cce7e26a8").unwrap(), // Aquaculture
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, BabeId, GrandpaId, ImOnlineId, BeefyId, OctopusId)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
    domains: Vec<DomainId>,
	_enable_println: bool,
) -> GenesisConfig {
	let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
		]
	});

	initial_authorities.iter().map(|x| &x.0).for_each(|x| {
		if !endowed_accounts.contains(&x) {
			endowed_accounts.push(x.clone())
		}
	});
	let validators = initial_authorities.iter().map(|x| (x.0.clone(), STASH)).collect::<Vec<_>>();

	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	const STASH: Balance = 100 * DOLLARS;

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|x| (x, ENDOWMENT)).collect(),
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(
							x.1.clone(),
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		octopus_lpos: OctopusLposConfig { era_payout: 1024, ..Default::default() },
		sudo: SudoConfig { key: root_key.clone() },
		babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(appchain_deip_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		im_online: ImOnlineConfig { keys: vec![] },
		grandpa: GrandpaConfig { authorities: vec![] },
		beefy: BeefyConfig { authorities: vec![] },
		octopus_appchain: OctopusAppchainConfig {
			appchain_id: "".to_string(),
			anchor_contract: "octopus-anchor.testnet".to_string(),
			asset_id_by_name: vec![("usdc.testnet".to_string(), 0)],
			validators,
		},
        deip: DeipConfig {
            domains: domains.iter().cloned().map(|k|(k, Domain { external_id: k })).collect(),
            domain_count: domains.len() as u32,
        },
        deip_assets: DeipAssetsConfig {
            core_asset_admin: root_key,
            balances: endowed_accounts.iter().cloned().map(|k|(k, SerializableAssetBalance((1u64 << 60).into()))).collect(),
            ..Default::default()
        },
        deip_proposal: DeipProposalConfig {},
        deip_dao: DeipDaoConfig {},
        deip_portal: DeipPortalConfig {},
	}
}
