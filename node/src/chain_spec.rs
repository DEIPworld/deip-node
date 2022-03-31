use appchain_deip_runtime::{
    currency::{OCTS, UNITS as DEIP},
    opaque::Block, opaque::SessionKeys, Balance, BeefyConfig, ImOnlineConfig, OctopusAppchainConfig,
    AccountId, DeipAssetsConfig, BabeConfig, DeipConfig, DeipDaoConfig,
    DeipPortalConfig, DeipProposalConfig, DeipVestingConfig, GenesisConfig, GrandpaConfig,
    BalancesConfig, Signature, SudoConfig, SystemConfig, DeipUniquesConfig, SessionConfig,
    OctopusLposConfig, WASM_BINARY, DeipEcosystemFundConfig, DeipInvestmentOpportunityConfig,
};

use sc_chain_spec::ChainSpecExtension;
use sc_client_api::{BadBlocks, ForkBlocks};
use sc_service::{ChainType, Properties};
use sc_sync_state_rpc::LightSyncStateExtension;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

use beefy_primitives::crypto::AuthorityId as BeefyId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_octopus_appchain::AuthorityId as OctopusId;
use sp_consensus_babe::AuthorityId as BabeId;
use pallet_deip::{Domain, DomainId};
use core::str::FromStr;


// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    // Block numbers with known hashes.
    pub fork_blocks: ForkBlocks<Block>,
    /// Known bad block hashes.
    pub bad_blocks: BadBlocks<Block>,
    /// The light sync state extension used by the sync-state rpc.
    pub light_sync_state: LightSyncStateExtension,
}

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
    stash_amount: Balance,
) -> (AccountId, BabeId, GrandpaId, ImOnlineId, BeefyId, OctopusId, Balance) {
    (
        get_account_id_from_seed::<sr25519::Public>(s),
        get_from_seed::<BabeId>(s),
        get_from_seed::<GrandpaId>(s),
        get_from_seed::<ImOnlineId>(s),
        get_from_seed::<BeefyId>(s),
        get_from_seed::<OctopusId>(s),
        stash_amount,
    )
}

/// Helper function to generate an properties
pub fn get_properties(symbol: &str, decimals: u32, ss58format: u32) -> Properties {
    let mut properties = Properties::new();
    properties.insert("tokenSymbol".into(), symbol.into());
    properties.insert("tokenDecimals".into(), decimals.into());
    properties.insert("ss58Format".into(), ss58format.into());

    properties
}

/// Helper function to generate appchain config
pub fn get_appchain_config(
    anchor_contract: &str,
    premined_amount: Balance,
    era_payout: Balance,
) -> (String, Balance, Balance) {
	(anchor_contract.to_string(), premined_amount, era_payout)
}

pub fn octopus_mainnet_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../res/octopus-mainnet.json")[..])
}

pub fn octopus_testnet_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../res/octopus-testnet.json")[..])
}

pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
    let properties = get_properties("DEIP", 18, 42);

    Ok(ChainSpec::from_genesis(
        // Name
        "DEIP Development",
        // ID
        "dev",
        ChainType::Development,
        move || {
            genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![
                    authority_keys_from_seed("Alice", 10 * OCTS)
                ],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    (
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        3_600_000_000 * DEIP
                    )
                ],
                // Vestings
                vec![],
                // Domains
                vec![
                    DomainId::from_str("6225314ed224d2b25a22f01a34af16d3354d556c").unwrap(),
                    /* generic */
                ],
                // Appchain
                get_appchain_config(
                  // Appchain Relay Contract
                  "deip-test.registry.test_oct.testnet",
                  // Premined amount
                  0 * DEIP,
                  // Era Payout
                  328_767 * DEIP,
                ),
                // Ecosystem fund account
                get_account_id_from_seed::<sr25519::Public>("Eve"),
                // Enable println
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        Some("deip-development"),
        // Properties
        Some(properties),
        // Extensions
        Extensions::default(),
    ))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
    let properties = get_properties("DEIP", 18, 42);

    Ok(ChainSpec::from_genesis(
        // Name
        "DEIP Local Testnet",
        // ID
        "deip_local_testnet",
        ChainType::Local,
        move || {
            genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![
                    authority_keys_from_seed("Alice", 10 * OCTS),
                    authority_keys_from_seed("Bob", 10 * OCTS)
                ],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    (
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        1_800_000_000 * DEIP
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Bob"),
                        1_800_000_000 * DEIP
                    ),
                ],
                // Vestings
                vec![],
                // Domains
                vec![
                    DomainId::from_str("6225314ed224d2b25a22f01a34af16d3354d556c").unwrap(),
                    /* generic */
                ],
                // Appchain
                get_appchain_config(
                  // Appchain Relay Contract
                  "deip-test.registry.test_oct.testnet",
                  // Premined amount
                  0 * DEIP,
                  // Era Payout
                  328_767 * DEIP,
                ),
                // Ecosystem fund account
                get_account_id_from_seed::<sr25519::Public>("Eve"),
                // Enable println
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        Some("deip-local"),
        // Properties
        Some(properties),
        // Extensions
        Extensions::default(),
    ))
}

/// Configure initial storage state for FRAME modules.
fn genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AccountId, BabeId, GrandpaId, ImOnlineId, BeefyId, OctopusId, Balance)>,
    sudo_key: AccountId,
    endowed_accounts: Vec<(AccountId, u128)>,
    vesting_plans: Vec<(AccountId, u64, u64, u64, u64, u128, u128, bool)>,
    domains: Vec<DomainId>,
    appchain_config: (String, Balance, Balance),
    ecosystem_fund_key: AccountId,
    _enable_println: bool,
) -> GenesisConfig {

    let endowed_addresses: Vec<AccountId> = endowed_accounts.iter().map(|x| x.0.clone()).collect::<Vec<_>>();
    initial_authorities.iter().map(|x| &x.0).for_each(|x| {
        assert!(
            endowed_addresses.contains(x),
            "Initial authority account must be pre-funded"
        );
    });

    let validators = initial_authorities.iter().map(|x| (x.0.clone(), x.6)).collect::<Vec<_>>();

    GenesisConfig {
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
        },
        balances: BalancesConfig {
            balances: endowed_accounts
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
        sudo: SudoConfig { key: sudo_key.clone() },
        babe: BabeConfig {
            authorities: vec![],
            epoch_config: Some(appchain_deip_runtime::BABE_GENESIS_EPOCH_CONFIG),
        },
        im_online: ImOnlineConfig { keys: vec![] },
        grandpa: GrandpaConfig { authorities: vec![] },
        transaction_payment: Default::default(),
        beefy: BeefyConfig { authorities: vec![] },
        octopus_appchain: OctopusAppchainConfig {
            anchor_contract: appchain_config.0,
            asset_id_by_name: vec![("usdc.testnet".to_string(), 0)],
            validators,
            premined_amount: appchain_config.1
        },
        octopus_lpos: OctopusLposConfig { era_payout: appchain_config.2, ..Default::default() },
        deip: DeipConfig {
            domains: domains.iter().cloned().map(|k| (k, Domain { external_id: k })).collect(),
            domain_count: domains.len() as u32,
        },
        deip_assets: DeipAssetsConfig::default(),
        deip_uniques: DeipUniquesConfig::default(),
        deip_proposal: DeipProposalConfig {},
        deip_dao: DeipDaoConfig {},
        deip_portal: DeipPortalConfig {},
        deip_vesting: DeipVestingConfig { vesting: vesting_plans },
        deip_ecosystem_fund: DeipEcosystemFundConfig {
            fee_recipient: ecosystem_fund_key.clone()
        },
        deip_investment_opportunity: DeipInvestmentOpportunityConfig {},
    }
}
