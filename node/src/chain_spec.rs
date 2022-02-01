use appchain_deip_runtime::{
    AccountId, AssetsConfig, BabeConfig, BalancesConfig, DeipConfig, DeipDaoConfig,
    DeipPortalConfig, DeipProposalConfig, DeipVestingConfig, GenesisConfig, GrandpaConfig,
    Signature, SudoConfig, SystemConfig, WASM_BINARY,
};
use sc_service::ChainType;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

use appchain_deip_runtime::{
    opaque::SessionKeys, Balance, BeefyConfig, ImOnlineConfig, OctopusAppchainConfig,
    OctopusLposConfig, SessionConfig, DOLLARS,
};
use beefy_primitives::crypto::AuthorityId as BeefyId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_octopus_appchain::AuthorityId as OctopusId;
use sp_consensus_babe::AuthorityId as BabeId;

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
                ]),
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
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
                ]),
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
        if !endowed_accounts.contains(x) {
            endowed_accounts.push(x.clone())
        }
    });
    let validators = initial_authorities.iter().map(|x| (x.0.clone(), STASH)).collect::<Vec<_>>();

    const ENDOWMENT: Balance = 1_000_000_000_000 * DOLLARS;
    const STASH: Balance = 100 * DOLLARS;

    GenesisConfig {
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
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
        transaction_payment: Default::default(),
        beefy: BeefyConfig { authorities: vec![] },
        octopus_appchain: OctopusAppchainConfig {
            anchor_contract: "octopus-anchor.testnet".to_string(),
            asset_id_by_name: vec![("usdc.testnet".to_string(), 0)],
            validators,
            premined_amount: 1024 * DOLLARS, //@TODO is it ok to use value from barnacle template
        },
        deip: DeipConfig { domains: vec![], domain_count: 0u32 },
        assets: AssetsConfig {
            core_asset_admin: root_key,
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, SerializableAssetBalance((1u64 << 60).into())))
                .collect(),
            ..Default::default()
        },
        deip_proposal: DeipProposalConfig {},
        deip_dao: DeipDaoConfig {},
        deip_portal: DeipPortalConfig {},
        deip_vesting: DeipVestingConfig { vesting: vec![] },
    }
}
