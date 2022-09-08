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
use sp_core::crypto::Ss58Codec;

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
                get_endowed_accounts(),
                // Vestings
                get_vesting_plans(),
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



pub fn get_endowed_accounts() -> Vec<(AccountId, u128)> {
    vec![
        (
          get_account_id_from_seed::<sr25519::Public>("Alice"),
          100 * DEIP
        ),
        /* TEAM ================================================================================== */
        (
            AccountId::from_ss58check("5Db6cnUaq5h9CLUeYxcbfqGhVaV5hFaXf71WFHRUwKcQUBdn").unwrap(), // ALEX SHKOR
            150725000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HoyzrbEtJzoLa4SuPrvTBcDphKFuKhdKey1E15f4FnyCqi7").unwrap(), // ALEXEY KULIK
            127100000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HC9am1sP8zjC69Pjcx1bqJHbwBPPtRRZ77tuN7V9HWGHF4e").unwrap(), // YAHOR TSARYK
            96800000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DZaWv9HQZCRiASBcB6363nj6xYYwTpKCHV96NTbxt1HNZrU").unwrap(), // DIMITRI SIDOROVITCH
            75375000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Hmt1AxMWF91dAKVcNWhMfiSyLQ7Ki1hWPbW8SspfMtMY8aE").unwrap(), // NIKOLAY SYUSKO
            50000000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HpPDX8PjNgY8iTxpsk4L5N1pqTyVd8sHvdE8yZs8T4x5vyb").unwrap(), // ADAM YATES
            1000000 * DEIP
        ),
        (
            AccountId::from_ss58check("5EPLFFo6WbZeGDEASPJGEbttX9mvUtLmeDAPk5HL7tC62Rn1").unwrap(), // YURI BOKACH
            3000000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CXQhTzBXrbwrNWNM4WVFhd6jaV8Z8VvgrykdzsND1PRiWjP").unwrap(), // DANA MALAYEVA
            1250000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FFTeqMRXkwYP7mqMyyCgaznzGp1pUF2r7ts6vqZXk7Eywm9").unwrap(), // EUHENY BONDAROVICH
            2500000 * DEIP
        ),
        (
            AccountId::from_ss58check("5C8awaFMLtcByzqKb9Vow7HjogFCNeD8V7MyFd1iFt9byNyZ").unwrap(), // VADIM SIAMRO
            250000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CJvjfoaFbU7H6bg3YVeVQUPzpvu1DL3XvxBUd4bdyXW3poW").unwrap(), // ANN REVYAKOVA
            250000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Dc7H96uJfqobyq6jmcFNbhHGyMRy6zV24qR4hd7js7nqnP8").unwrap(), // ANASTSIA LOPAREVA
            250000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HBFw8FGnWGR2xaRVcF3HeyxzA32kyJFnmcrhJ4PaxnvjN1k").unwrap(), // KIRILL DONCHENKO
            750000 * DEIP
        ),
        (
            AccountId::from_ss58check("5E81jQhukbEQejzMiFjieCcU9LQJnNCYM3wKqkzAeKtpSuej").unwrap(), // JULIA SHINKEVICH
            3000000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GGnoW31ByXs2JsPNXnox3v3D1ohuQyNFth4PX9cVHPnQJvr").unwrap(), // YAHOR VIARBITSKI
            500000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CY2FdXfoa6d488avA6Pse4q7LLG3Ve3cSgWNziYJHgywBxZ").unwrap(), // DMITRY LYMAREV
            250000 * DEIP
        ),
        (
            AccountId::from_ss58check("5C7qQK6vVcGDdddhtcrgaefLoCr4eGxsXn2AhFc8KPzWg4xc").unwrap(), // DARYA MARKEVICH
            250000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CcQeujdaVf3q4Hisc1WixeTVVJePucg1yGeiytZJtosMJ7v").unwrap(), // YAROSLAV MITROFANOF
            3000000 * DEIP
        ),
        (
            AccountId::from_ss58check("5EjiFe2G9m6GX8dqMWwG2ag7pcga5aUYfUUp1dJ6JJRTxUwo").unwrap(), // YAHOR ANIKEI
            1250000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HC9VNrbXoATKmQMSgGmCu1dQ6wSENMh1bzA6Lbpmv26hhCi").unwrap(), // JULIA NEKHAI
            750000 * DEIP
        ),
        (
            AccountId::from_ss58check("5G6rNhewyF4AuzFfmo71VMsYnFqwWqneUFqxvDsgiot8k4yj").unwrap(), // ALEXANDRA LUZAN & ALEXANDRA SERYKH
            2000000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Gx37xJrXEG6U9agztqkEeCP814v1kzF1AfWjoiaiw6Pucpk").unwrap(), // SHISHKI
            1250000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GEsc8PsQpxW4k4anHShJnu1DMJE7faj3rpf7ubNRYa9VFom").unwrap(), // SVYATOSLAV IGNATSEVICH
            750000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FC9GuwzwhNAvMGHXS4i5JLFJGBVTZAWskCSgtfP3geXoM9s").unwrap(), // PETER FARBEY
            750000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GjQJFSBxkLhGB4mUhPCVFrqw2XxEVfqRVTKHRPu91JNHvk6").unwrap(), // VITALIY SHALAK
            1500000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HWTkfyQ5aREGYjAomdHn41yfnfjzvZ3mnq8HwaTATtE3drM").unwrap(), // STAS DASHCHINSKIY
            1000000 * DEIP
        ),
        (
            AccountId::from_ss58check("5EeJUD76GrG1H9gDWYqt54XP3r7yfSXiXUeLSUfhM6FeKJh9").unwrap(), // YULIYA CHIKILINOVA
            250000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FjJC6w1tFb8dqTB3Ke9uWZa8Kdv3nEEacCbxQxBZm7V6bEX").unwrap(), // ALENA KRASINSKIENE
            250000 * DEIP
        ),
        (
            AccountId::from_ss58check("5H8e3DPYYUg3eij6xLeePYxwxtbztCVkcDemWSHiLKNYbTPj").unwrap(), // TATYANA BARDASHEVICH
            1500000 * DEIP
        ),
        (
            AccountId::from_ss58check("5H1YZ7jhJK4CgzPoCmMfp5SYFDa74GeVgN8q5uEPxHmB2WBn").unwrap(), // MIROSLAV MILOVANOVIC
            500000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CK7Nzhf4a1HWqjAuFiyoq2RGzTtrH75mPyeBuf8m9bnsRMd").unwrap(), // ANNA YADLOVSKAYA
            1500000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HbeSwjQs1xWbQ6M35VdoMbK1NPAwyS8thk2cTNr2khzeFqj").unwrap(), // NIKITA AKSYUTIN
            1000000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Cm571HB9WQ1MA4cVmwGDZvZARdjyLAZTgLs7vJmXgxUP4y5").unwrap(), // NICK HAVRYLIAK
            2000000 * DEIP
        ),
        (
            AccountId::from_ss58check("5G3eg2UqYcA141fbed1V1XZfuSxktY4v3da93i31JjvHHtim").unwrap(), // ANASTASIYA BOGOMOLOVA
            750000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CiRKcwip5QVLT6Ec9HCXfgJbyMd24YyGWqyCZLyRrqGM8EU").unwrap(), // JULIETTA VOSKANYAN
            250000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DXGDisfj9jgp62wL6VjDs4RTmUVzsBJeGeva1UHk5pcrQk5").unwrap(), // ALEXANDR SHMIDT 
            250000 * DEIP
        ),
        (
            AccountId::from_ss58check("5H73xJdkPepNHyrRDSKbw8AtauZn3wt9nCP8k3i1xq1GoNfH").unwrap(), // ELENA VLASOVA
            500000 * DEIP
        ),
        (
            AccountId::from_ss58check("5F1RoDYEgAsCALL9ZXX9B5ctXTSi2Fm4ryzdFEUQnjvATh9Q").unwrap(), // PAVEL LAKUSHIN
            2000000 * DEIP
        ),
        /* ANGELS AND ADVISORS =================================================================== */
        (
            AccountId::from_ss58check("5FUBk8tjkXUauTwKDvhjj3ujBGNswQH5evY3WkCFd1TxrUct").unwrap(), // IGOR CHEBATARENOK
            18750000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GRdTZyJYjUM53ZC1FkrzTaNqSmw1N8WShTxDdKaWCbUe4su").unwrap(), // DAVID BOWILL
            12500000 * DEIP
        ),
        (
            AccountId::from_ss58check("5F2BhPMUZjU39PCGr4CRrgXTp2bxenvXYdUMNeHTrx5EWEfL").unwrap(), // LEONID LOZNER
            34625000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Ebi4kE8kBwuuYvJH9GQmfmbF6v2trviVQS86Hv4PEpoDiML").unwrap(), // SERGE DZERANOV
            7500000 * DEIP
        ),
        (
            AccountId::from_ss58check("5H13CpAwvMUTeNcTxfGmi6YdoSDo6kAHJfde3H4YoGyq5WeL").unwrap(), // ELLIOTT TEISSONNIERE
            16000000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Gq4YB8A4keRLFu84cKQNizWo93RLCyt3TZqENMeFxxXpmTJ").unwrap(), // PARALECT
            20000000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GHNyHmqtBUoDDxrZiYn5btVx2axrt7Yxaf9LxkjWJiXm2Ev").unwrap(), // GOTBIT
            2000000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GQzK874N16bUULNd6wbeWMZovq4kZi64hJyAWbdse5gq126").unwrap(), // ANGELS ADVISORS UNALLOCATED BALANCE
            132000000 * DEIP
        ),
        /* PRE SALE 1 ============================================================================ */
        (
            AccountId::from_ss58check("5GEsCZbCvPYWpTkHrJTRYZFQZSTrBSat8bPLWA21prM1Kkfm").unwrap(), // ERINC ATILLA
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DHZfJSmAA88jg7o882GNn7PdcQ97BxEQzz5orDY9uvqFGKF").unwrap(), // MUHAMMAD SHAFQUAT HOS_AIN
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DDSbcYtRQjDtrFRkrnbGv5apMw8jCweArAjRtvdnNi9jGnY").unwrap(), // BUSRA KABAOGLU
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5Cfqza9NQX4hjXqoYeo5HtWGSF6mQ9U43X9iCP1GEPCuPzwX").unwrap(), // MIKITA HRYVITSKI
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5H6AZtYUqLcYX7YHD5Wq2CPEvGktXS8ZNh8Z9pHJPD8CAAKJ").unwrap(), // KIRT SCHELLHAAS
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DjwpJNWAfmKByGaKZRXAWQ6Lu6TrDEmeVvMSrbyhTb91CAr").unwrap(), // RYAN JACOB BEECHER
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5FTPxZcTuz5t96kD1jgfwDoP8Ya7XTbve6CMWncfPotn1fHR").unwrap(), // SYAIFUL ULUM
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5D1zyTaQmMPR9bL8zXN9Ws25ESStgHKaSwPT8Yc8hsJ12qLN").unwrap(), // WILLIAM ERIC VAMANADEVA JACKSON
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5CCsqkL72QRb1qEpuLiATq4cQeKaRmMyp69Su2tuXukrvGyh").unwrap(), // HARIS HONDZO
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5C5KUz3uYhn8kz7toUg5D3R9P9CkFahdV6Cs2GXBCXLzBPHy").unwrap(), // YAOTSE A EDAH
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5FW2Udq1VmExRw4MhRiEA3uUhCTxv6Q8ZZZbvGXYqSQRWHJk").unwrap(), // PATRICK MUNYANEZA
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5E1ko47DKbfkn7wKsRpBWJ4qUkUZ4SbmnN7PaoKyBKM4RnU2").unwrap(), // BAPTISTE RAPHAEL LOUIS DUMORTIER
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5CcPbph6M1sALoGj5NmH9u2SQa9JeGr9anzECv9qT8Dc12Ak").unwrap(), // ALEXANDRE BERNARD DONY
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5FCebSFxH7Be21xEp2FTchMTekHBhZfpTVsdGLiqAEeMEq8Y").unwrap(), // Y�CEL T�RK
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5ChZRhziKx8iHWHqPWixbxSiyvEDieeMMSCXZzuszTVht96N").unwrap(), // XAVIER MAGRI
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DfZS7hwHRr1Dv5Moyf7KurJFKcnq9GqBUtHP2gsnEnbJkiN").unwrap(), // VINCENT ZUFFEREY
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5He6kffATaaeZNhUYefX461zP2PGoAMUB9me6AWeHwsEhWyY").unwrap(), // RADOSLAW ANTONI MOTEK
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5EvBdDfJP1ZM2FrrVMkoNsN82GdsHGEHVwZKrtPPreBWF5Lb").unwrap(), // DANIEL CHAVARRI ANDREU
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DRjne1xdsGCNK2N4EVTBiaBKH9PDDmX8EP19bVscYohPwzY").unwrap(), // NURIDDIN JAMOLIDDINOVICH GAIBOV
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DqaWGntH95mrKDrf2r4CWFshg2hS6MP1icPCmRuDAmi9QXL").unwrap(), // AHMED ALI M IDRIS
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5FHwVWT9eJGknuwYEk7VDcn8N7oWTKBSPvrraQhXdWnC5amF").unwrap(), // SEBASTIAN KULCZEWSKI
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5EXE67p4eLmZ75esAuyhPExBMEJKT6ZLRHnbUzGxxecZsxKQ").unwrap(), // FILIP PIOTR KOSZUTA
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DNq57BKzighP1ihFfBjPhMZPF8S71HoSWuPtdoVQUenUbwg").unwrap(), // DOMINIK WINCENTY FR_CZEK
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5Gpye9H772hyRYoRVywV9eHJDpDhCJgwyaxJaBGL8PJugu4P").unwrap(), // DAMIAN KUCHARCZYK
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5FudVGoyhwqBYMQpuZK7Jgg19n9KhjLZhhMesHcKyzX8hWVn").unwrap(), // ROMAIN AUR�LIEN L�ONCE MILLON
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5Ccvzpdvh3Wdh4Btxa9L95ADfWz3W2MZnRDPE6Qcg9nTnu4q").unwrap(), // ALEKSANDR ALEKSANDROVICH SHUTILKOV
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5FcCPCz2tByawPgbB3irkwXKxCrEyfR9eb91TZMd8gPHfcfB").unwrap(), // MAGAR KHEM B THAPA
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5CPt2UtBquYxwa7HaU5eHvQa1wSZLte7V3psthHqiNeddvsQ").unwrap(), // HAKAN DEVIREN
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DJXK4UP7suN2nQpU6fHNrsgygnPjxSg35EtDnyqRkFEHfSr").unwrap(), // NICOLAS R�MI ROGER LASALLE
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DhpnzKCG1A3UHXtw4qFJn3TNMLZdrLZ558XGmiYLnr9uMD4").unwrap(), // MARGAUX ROSE MARIE MARIE MADELE BORGEY
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5F9VhfFoGCQaGoKmPF8wKxowVWBPTsZHPySKfWoNH62yDd2B").unwrap(), // DMITRII NIKOLAEVICH LEDENEV
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5HMeurL1piUBN5CjX88yYckcLtBSuWDZGjh5bzqSeo8u72nS").unwrap(), // DERYA VURALGIL
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5Gk3UwL8hiR3zk4CHXh9WTKArr85UaNEypjo9epCjWQXnmeH").unwrap(), // JEMIL BIHA
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5Hivc2wnjFbprGWX38Cr33dC4kTb4M6g9j9axsnW23DEgntw").unwrap(), // ASHLEY LUKE ROZARIO
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DPCjbRXWPK4127AquKm2DEHMGeXVT9TqqCiVSGBieZuuCCp").unwrap(), // WALID SAYADA
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5HbH9dZT9nsBaEqwwNTnW5BjfNHMwY3GgGJzN5qK94NbtkzE").unwrap(), // SZYMON JOBKIEWICZ
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5G4UL8LP5bfp9VSXrWYD2JPta1tcNGpRmD2LkTPJ8Q33rrAU").unwrap(), // JALAL
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DLg7qwTjqaTRQ59EqZV6hKVVaMBqsxk8NH4Z6ev8p7DtTcF").unwrap(), // NILS CLAUDE JEAN FRANCOIS POUET BOCARD
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5CFTFgDxcoCf8eefpYwuJhtswt688quPZzAhb71EzopfF8df").unwrap(), // RESTY CHORYANDA PUTRI
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5FH3k5DBadPwS8XZmRPBPQXTbAxRAAHxUcRAZmM2KoHvd4L1").unwrap(), // _BRAHIM KORAY _ENG�L
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5GKQ1dsf1KUm8QvPqFvwsHZX3LYCsbTEenVJDeZg92kNqMkX").unwrap(), // RAPHA�L MATHIAS PIERRE GAVA
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5G1hfJDpNpTAWV8j467qKr54yJnRBXST581k7gsaHCh3iFDP").unwrap(), // ALBERT ILYASOVICH KARIMOV
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5GeRzXJwcjyEe8uPPzBWmDaHiMadcwjPHJi6rPmkVR8yhMtT").unwrap(), // FARID OLA AKIN IBRAHIM
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5GEQ2c9CmuamDxxgmiNwfQAD6h2VnKTGtNPA4qv2J29qRsKM").unwrap(), // DUYGU BURAN
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5ELiJmmsV6BcN9MUgzBL2GZQYqTnkkZ8Xa4zFCznFivHdHHJ").unwrap(), // IVAN ANATOLYEVICH LAPTEV
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5Dsd5VVNfY5jNnZke9FCed87LYAnsCXRC8fiZ34x8guyUVv2").unwrap(), // AHMET CAGINE
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DkBf5CqqYYofPjR3578hsJNqRNKzNVZs2YSjXaUp3wbChRp").unwrap(), // _UKASZ KAROL ROGOZI_SKI
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5HT2rjX6rYAwpH6vEQx8hNGkYft5b8pE1qKkAQG49mBmD5fS").unwrap(), // PIOTR JAN PI_TKA
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5G4N1E1CVPnfSfTTLwJomHQguVkaLygkv8omuXzuRREuym6n").unwrap(), // BAHATTIN AYHAN YILMAZ
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5G1dtLuJgFnvGJrUCe3izYUGvjtEvSUe3fsMuysqa2C8S2tq").unwrap(), // PATRYK SIEK
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5D5P4uCAoeTLGvipLzF4LGXT5qabtNdwfL2v1rp5SjrjcCeU").unwrap(), // KRYSTIAN KARLI_SKI
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5Gv91dEkixMuZrFY8urnd7mbfjHHza1SzJ8NGeLTbh4j5wCJ").unwrap(), // EMIN EDIZ OEZ
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5EUo17DKM6h2H8JoWoAs4kRqsb9JH2X1PHFPrU6GNUMMsBVu").unwrap(), // WOJCIECH DEPUT
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5FgxZH2LuQcjiCWU74XveLnRnxABfU52797fsVNvUTQacKjW").unwrap(), // YUSUF POLAT
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5GjbPbgZTKyLvqp8ZdDBpsveMY5nU7ngMgWZf1TdhEy4Uhp1").unwrap(), // ARKADIUSZ KRZYSZTOF KSI__KI
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5FFVfmN8hiKbaEpu3mzyu6TskfRQvYte2nersKbPB4HWS1se").unwrap(), // OLEKSANDR ZOLOTAROV
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5CkxPn5wfJtqWLFsTzmTahUD4ygeaJMtpUiRhKadjN9FmPVa").unwrap(), // FRANCESCO BERARDI
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5GZQPjQjnwwhnRAGjeTj2hwP3RQEoGrpmtTyshwyopSZTLfb").unwrap(), // TARIK BUGRA UYSAL
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5FHtQXTUzkpXYVDrNUX6Sbm3Fy57HU4LDLz8sBjgQwm5hbDv").unwrap(), // RUESTEM YI_IT
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DkdcpXFZG97hyTjdr8mbq8DNJbPLk1VVyTR9h1jYQVx12iy").unwrap(), // TR_N-QU_C-VI_T
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5D51Eu39GMApmGFaHGXLA6a8MFXkrTegaELy38rtgdUEq5XL").unwrap(), // ILDEFONS FERRANDIS CASTILLEJO
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DFSP7D28gCk9m6wjz8q9vwEk8mXsx2E1UNcPqyYQbv8pFhK").unwrap(), // IHOR VOLOBUIEV
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5G7Lvu2qoesT5gskfS548reXXRtAmWnUqCAGBAZQNWUHLfV5").unwrap(), // LANCE ATKINS ANDERSON
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5HE6pvU24j1VWFgvsE5xHUqCkN4JrmCM4bsM36v9gvDBBnwN").unwrap(), // MERT �AKIR
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DFjQAs5vwt1dEazTbar3k5KG2TijBKfBP7uynvmL5ZJT6sB").unwrap(), // RHOMAD ANTIONIO JORDAN
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5FbHrKHPUwMrgNxZnMEPBxf1N4vPduWr6CjjGUEgR5g8HryZ").unwrap(), // LANCE A ANDERSON JR.
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5G6JLkKd1icetzbzoKC1CLd8GJVbwRyo93J7LEZNnhJCV2JP").unwrap(), // IGOR VALER'EVICH ZAETS
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5FA82P8eQJMLuYPhDkiactaNtyiC4BYEuHnwA5LEY4NeHDhe").unwrap(), // PAWE_ ADRIAN MA_ECKI
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5EcEmmuFibsEZfY238F2wgyTz32X1bV6HHvxzhL3jSiziFXp").unwrap(), // BRANDI LEE PORCHE
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DoAjJ4Wpf9nmeFgGo5fMm9kNQha436vJgRAmjBhignvndEY").unwrap(), // FADI GASMI
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5CGGWwgB42qJPhJD5pCwKuKwVXvnyfEpLRAaWreQRGmSjihr").unwrap(), // LUKASZ TADEUSZ JAKIEL
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5FUSXEc2x5uj7X45TTnhAZ3PDprciNLiyToHiW2PZKrkMNrd").unwrap(), // SLAWOMIR MARIUSZ LESZCZY_SKI
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5CB1R4sBRgAPU2fEt9R1JyqfcKB7dee4MUUXYLVbHF8uUJ9X").unwrap(), // PRIYA RAVIKUMAR
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5EqrWxrCq2pj6gqzLkPeRUUuJkJDeyVNkXSKgnfnAqurCrBS").unwrap(), // ADAM WOJCIECH PI_TKA
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5G1D7FjJoCJBZ1mYxUAbPz8yMCUa3nrcC9FPXkoqZSNat7gi").unwrap(), // MARCIN RAFA_ BIENIASZEWSKI
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5CDWpi55MD1b8ZWAu1AMnqaoVzKT3ATJiZXsPnyaroqys6FB").unwrap(), // DAVID ADOLPHE RIMBAUT
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5Gy7BKPaZdXCoNPPTbAaZ7hGxRhvWxyFfo37RXPHTekCeLS6").unwrap(), // CIHAT CELIK
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5CFk6txbsc9ENtpuqaCcybkWUxuV9pg1qyrKkiZ2nw3nz2na").unwrap(), // MOHD SYAZWAN BIN JUMAAN
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5Gq934hmtcwJDXbp83aKkRYMGvQt8FRtsAuzZaph3rfYAhrw").unwrap(), // ALBERT OKO_SKI
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5F1vYv1fzakeKw3y3F1BT24zVWqYyngG8qqh8gXjFcBvDhm1").unwrap(), // JAKUB BRUNON SOBIESZEK
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5E7a16dmk6yxUa32yefyJXVv6kWfee3FJeSxEDE84EZeJQVB").unwrap(), // TAYGUN YAVUZ
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5FR1dQJKMEyVWiwJLnq1N3a9cfWcj8C9jRnUfykKtKNHJZXp").unwrap(), // GURKAN YILMAZ
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5CfsErYV7jxshMysgF6A8sFQQ5xTkW81VTx96zH8F1amanuq").unwrap(), // BARIS ALTUN
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5GZSa6ckrP1epyP1arv24Zf1fA3b9T7tMUkj675xcKAEuAVY").unwrap(), // ERTUGRUL SEZER
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5GKe9KugAmMcVUos68JG7Wcj5fhsH65BBLsLhPkdXFGtemSi").unwrap(), // HAKKI KORAY SULUNBE
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5CK59QH7W3RGtrgpQmcSwfEfnhhrmWZSwoFow2RkGDM1Up8N").unwrap(), // BAYRAM ALPER YAPICI
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5E4WSQxJctZRuss27XP4s8veCVCtMzN7qFBjQtLPZsGfDBqA").unwrap(), // MEHMET EMRE DEMIRYILMAZ
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5EA8HZ8eCN8Gu3K3KGP14ZtJpbvx85ZzoRmCVByF9xU9zR8m").unwrap(), // BRUNO MICHAEL SYLVAIN HULIN
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5FPH7x7DPQiyvjAXRABvciPeYMVCTrAw2gKykXzF6dqZGSmQ").unwrap(), // ANTON SEMENOV
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5GCVKWA4iNsJp5eAbe22KthcsQ3QfsjMeQuyM6sUCWppbN9C").unwrap(), // SOF'YA ANDREYEVNA DORONINA
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5G9r8ybBJysCt7XzwyH4StG2hADnZurcapsCqCVciiHBJ6E7").unwrap(), // OLEKSII GOLUB
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5CDWtH3VKa73pcPicwYwcUeCyyuPZosyEiaPS1n9gpxApDSd").unwrap(), // PAUL ANTHONY ROGER FERNAND BODIN
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DyHyJqtTVmPAN9mNkzfeVaQT1pDnYTZVMFRJ2s5o7cwbzZe").unwrap(), // WUSHOUER XIAOKELATI
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5EHijRE1oU3kmdAhudEf5fzf4zhkdNVnwfGWiARyBcY4ZsyK").unwrap(), // MARCIN MARIUSZ LESNA_
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5F4UrfjG1hho9fWUnQWCH8GxUyYXoBJn42nv6A5yUnZA6647").unwrap(), // MARCIN CYBICHOWSKI
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DPbmEstsfnN1gYDb8ThwQqAZWhgFrYqAustj7tMJ9C47H2L").unwrap(), // EVGENII DENISOV
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5FPGhKucgRbLS6nV4bnP85sa56AUfcYi9LaD4W3bvVNpd6Ts").unwrap(), // JEAZRIEL DAVIS
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5FZVgDaL98RKtpqQcfSfZoBjbA3tiaUQoqDfQvNYtFySNeJH").unwrap(), // MICHEL WU
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5Ebrru89ZaCuzfxR6hU1hXoTxd5CPYWqdCUzVgqbEen1WxPP").unwrap(), // ROBERT _LIWI_SKI
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DtZKzQiQjcKX7YYyuj4jvBrL1oQBQNfFVYw44Y4JENTJQPX").unwrap(), // FLORIAN LOUIS ALEXANDRE DUMAS
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5FxZRZivo1a6xnRvsLt1BX1ZY7WKuHbMPhRS9nMmuJSYn9L5").unwrap(), // VLADIMIR ALEKSANDROVICH TERENTYAEV
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5E48pbGgtccLYDG3qdCLViBwCpjdnGrNgkxwsrX5vxtyxhst").unwrap(), // ANDRZEJ PY_
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5Higt9RGuMpLmPGPdqbN3WPX13ktep2zjnTSUQWME5NyYXwL").unwrap(), // TOMASZ SOBARNIA
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5Ef4y5z69kqyCsBFDLLfum66AxHqPau3eqEqXYmu1ZQvg7Ea").unwrap(), // MAKSYMILIAN ROBERT JARO_
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5G7PDu2dJds796o6YYP7iaHdb4SSo4TFuyBDdAnhKUdqPWMv").unwrap(), // SAMET CAN SOYLEMEZ
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5ENfyfBnuRedkoC5y2RbN1PPt6DT36dgzkq3doCHMTTXyNAa").unwrap(), // KRZYSZTOF ANDRZEJ WI_NIEWSKI
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5FWHW4JVyocygybNTK4zFW3tqaeM665M4YzZLXx6xzMcDvsA").unwrap(), // MARIUSZ MACIEJ KASPRZYK
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5ELSTVVms4AbBLmcKhPzsr7W8seFz8syVtxx5TuHLPNcNYkA").unwrap(), // LESZEK PIECUCH
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5GNTpYPo48vw2J3vNdg5X8d2Z8ru4K15DpnYLhWkosUgHFvb").unwrap(), // KAROL RADOS_AW WAWSZCZAK
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5CidYAncvMYAuTWVnQvUqun9nqyRBJBKx17AkMxbxWJJ8Pei").unwrap(), // PAVEL SERGEEVI3 KOROLEV
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5FHYAHqqKALf77VD3u1Qcf5sggZhg1fLpH7iUUi9h5UUxPyv").unwrap(), // MEHMET ATILGAN
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5CifMwqxP9VbkisrE4ydLZQhY1n1dBfb8eznczpi8CsmL7pe").unwrap(), // GRZEGORZ ADAM KACZKOWSKI
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5FpobETv8QZKjLJKKqCbRsUcAdXym65KESsopV1VeQBdqcog").unwrap(), // THEOPHILE JOUANNO
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5E4vByiKUxgkLH52psitfAhZQxgDdZM9BC88HZ65rz8wZrZJ").unwrap(), // ROBERT JAROSLAW BLACHEWICZ
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5Caps6mVq2VXhRCb16Jzn3QmhVehZ5JbHUWWGrjUa7r8SPdk").unwrap(), // MEHMET AKG�N
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5Ct2tgkW1dp43bz3y5NTueMwcEqs5a7hVLMtVvxX5nBExdwJ").unwrap(), // OZLEM ATILGAN
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5GioisoF8wA1Gnjf1LBTVQaZ5JpJ8bvGw75M9a1jRrFK5DSX").unwrap(), // HENRIK HOLDMANN
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5GQxj5yHQ1qkKdMshJ423T8KJujrFMXBC1YTb6vD5cZMHsnA").unwrap(), // WILLIAM RICHARD WEBSTER
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DSCYHHMxKtCKGcHAPZjN8TGrdiz9xrh1L66psMkki7NAogv").unwrap(), // EVGENIY DENISOV
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5CrYhYDYYtNeQZyP5at2v8vL2dJ4X9R3kSdT1LzoBnHxjxZT").unwrap(), // HO�NG V_N D_NG
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5GNsNVuvW8oExaVKHv9qSTYEAcqfAZukXETeJc7TqkrsU5ZK").unwrap(), // ROMAIN OLIVIER GERARD GAGU
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5GMsifhQ67jGTiGwtvL74jEEsTiY8VzouNKzHaVXftpjZX2m").unwrap(), // WALDEMAR PIOTR GRACZYK
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5EbdGHSdfbdJJsxohWDk9bGxKbsttp4fr2dofevfZu9W4x6K").unwrap(), // MEHMET AKIN AYHAN
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5GTrVcFpJTuWeyLh8H2eQCW9ogPSavUSsSMusMiZFPPz9y6w").unwrap(), // MUZAFFER CAN YIGIT
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5ComRrjZcPnBuRbNJPB1A3LVSYoB6bqeW8XjzVZpWgdZMTA8").unwrap(), // MARTYNA WLAZ_O
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5H3tTfTJpKqNTJhiXjZKtwAa17EYjeTW6h2whzvocDeALw4J").unwrap(), // �MIT ALDEM_R
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5EYwPNwfXaf4juWw2Nre8iQNNDeYQEhBuvvC5sFcHViDbuKv").unwrap(), // ANDRZEJ SZABLEWSKI
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5F72r6GzvddtYg9NEhUjY7Q1vnz71UUfHUVBHnQXZPd2MspH").unwrap(), // MARIUSZ KONRAD KLATKA
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5CVC1azsa5wwTyWqjdy4f5YEX45NhYW7dkUYBqciQKbkdevp").unwrap(), // TERIIVAEA STEVE FAETA PIERRE HANDERSON
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5EwBSkbtu4MatmxPQPkXtf2YQYVmSZ6UAHTJ5WztMTt88U68").unwrap(), // QUENTIN CL�MENT ALAIN PIOGER
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5GnDDL8Wx8GfbixRobMFwbkkmAc9bHkyoKGYrzak7vtd8vX2").unwrap(), // JACQUES CLAUDE ROBERT WENDLING
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DPuMNh2rnf6Lu6wkCpuu938R6wSYz7u3NXAHjih9uABfuKv").unwrap(), // AMEER HAMZA
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5GKbdZyYYXE1afJEEFhJdGjRxXJgxyehhzKEKhufAT4jj4Pz").unwrap(), // ALIX ANGEL OLIVIER SARTORE
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5Fh566w4yt87jtu5RQc1Ndc4uhJsacRUWRC3JkmFGSv8CNCr").unwrap(), // ANTOINE PAUL MARIE CHARVERIAT
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5HotcMk6sLcgSBhvT9Xis1hHLsBgHBodPfD9UqYB55Bm2kHs").unwrap(), // BERTRAND MICHEL PAUL LEMAN
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5GbA4M1wsKNvuxhVH6gEwSb9X4t37XzCC1wYSypcAMu8eRFN").unwrap(), // HALIL DASKESEN
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DPDK1kss1NroqNB5FMcBGyULQvstwPz94q1eeubh2vMwTL5").unwrap(), // FRANCOIS NICOLAS VANHOUTTE
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5CA2f2F9wyXjvTbaypa2QeLpYKGhghid9JTafbPLJK1ozPxV").unwrap(), // RENAUD GALAAD PERRIER
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5GRE9ZjqMj7RZnJVKrrTt7em3eRtMtZMbFHWPmQFazvnk5rj").unwrap(), // MICHAIL AL BERJAOUI
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5GgXXgLCmAdyWMehHgTNj2AHtuQhFMfRFSGE24RHDcqgHzYG").unwrap(), // SUEKRUE YILDIRIM
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DLaaySmgFw7B9dp7174Y9bvf76Qe9kRP1uf6tfpLm1WgbX2").unwrap(), // YAROSLAV ALEKSANDROVICH GRIDNEV
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5H64HE5CoyDmtpt658RsrREJL3ovBonpb4JR4mierfymiBMc").unwrap(), // TARKAN OZDEMIR
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5GULF6rJaibL2xqN87xLnPMtPPFNVdUZaMNx58uWoFQ8r6br").unwrap(), // KURSED KHAN
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5HY3zft4dQSgGhtwsubPbwSDyXgBeNx6b1fM28QfpVhB2i9D").unwrap(), // YAVUZ AVC�
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5EJC4N2mCm6t8rM5Qvhi6xVvR59bMf6iaauwhEYEnJu5R1Tp").unwrap(), // MICHAEL FRIEDBERG
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5GWqnB7oVXVwFL7Xu2cXVkXhfY4LhpTQTDbYZTY19Mn2PKQE").unwrap(), // JEAN-HUBERT JACOUES GEORG�S GENEVAY
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5FWwovjmjyRtmCFPrQJx5pGGNNanFUCrz5uKPcAGYPjD6ZSV").unwrap(), // GAETAN ANDRE ALAIN CHAULOUX
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5HmJzti1FXbfurC8ASLtJQNhhJhyWH7ibjgmJPW2YxhSX43S").unwrap(), // VO DANG HUY MAI
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5HRJdRcge8SDny2Zod54P8abtcvCzXDyijhVNkvp24S5AXo1").unwrap(), // ILHAN GOERGEN
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5HdWmM3E6B6mhqy8JGE2kdQ3LkE9UX5e31sP8mAkTg5cQme8").unwrap(), // DAVIDE STIMOLI
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5HTGBD87z9iwkZ4dcySRHNuty9QMXsWbS5VdVUkEZM9ofVEE").unwrap(), // BENJAMIN JEAN CLAUDE ROCER PICARD
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5whQV6WUtavNKNgHDeCbbRoxoHMTkHQbqCAWpAEoJeEYW4PU").unwrap(), // JEREMY ASTA VOLA
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5ECk1sb1cHy8AjYrEG2sR76e5YiFUiBKy9qnLgVZdufNr8cX").unwrap(), // PAWE_ ARTUR JANOSIK
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DAtnPuWS9sagNeNjFxExS4tzKdMx4g2Aa1arZJs6w8jwptu").unwrap(), // AHMED DJELLALI
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5E7NVxP8jQcGvZh3DsvYs1atCaGDrjBJUsUz6rN4Gzu4Z5XT").unwrap(), // ADRIAN LOIC ALEXANDRE CALTAGIRONE
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5HCDAhDhdz8b5hCVZSkhrFhiwaidK965eLqG8UMCdFsZU7yd").unwrap(), // _LKER KARASU
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5Ft7StW2W8JUmqs689dc9B15MwAvPhrkHVrcaqJGWbrvibu7").unwrap(), // RENAUD PATRICK JOSEPH LEVEQUE
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5EecAEaFmh3GVpe2yvJbeqX9pN3noRH3zsSFSnmB3CqyZxMe").unwrap(), // HERVE GRYCZKA DANTHONY
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5HSyAGdNa54q5mWGPB6itYbJ9uyEtWDxjhqo6wgrdE6Amsyc").unwrap(), // GUILLAUME GR�GORY KOCHKANIAN
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5GYtaj4acQ4T1AKNPkc1nfi3Bb4HaJpUTLqrhTdWjpG9sM3M").unwrap(), // ANTONIN GILLES MARLIER
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5Fxzg3y2S4cE87WyZn6VpHjEfcv3swDAaDMyMAD5eZi4BivP").unwrap(), // MICHA�LS SAVIOZ
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5DhXackC2obWqeNfd8Gnzs6ZCpX4STMuLSyCLik9c8tn4Sec").unwrap(), // JOHN STEVE TOURET
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5Hov9Dp11Ub9TiziwV9HjSmpyxTfFkhmdVPCRLz7vUYMAxNP").unwrap(), // GLORIUS MARTINUS SINAGA
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5E1j5rocTTzLjw7dYVhRsgL4x7QoS8WG4k1mEfbwUun394wr").unwrap(), // ABDULBAK_ ERTA_
            37500 * DEIP
        ),
        (
            AccountId::from_ss58check("5G1bY9HW7gafVafKhVDNYrtfzYB9qTJm6Y6kJ6wj26jjMGyX").unwrap(), // RABAH HAMISSI
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HKTMzbWrzfCYU3B6rUeQ4Kp3RcyjbgtXrXVA7npQkq4RHiA").unwrap(), // MOHAMMAD BASHIR KATTAN
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CXSGV4Ymfo1CyESzsYDQE2uY3vBwbqFToqM3vC9mQ9A9pFh").unwrap(), // DOMINIKA JANINA KUBOSZEK
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FetVhu8xRYaHRxMh7n6UzWxYzjzhgihwoqotHbanJKY3gyD").unwrap(), // JOHN P SILVA
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Coq6v2FRoMycVbGFXnwhrJAhdLPVTEWAC1V2g4D4G69AuXV").unwrap(), // HILARY DUGAS
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CFkDL51P3MLQV4Go66nJ24UAjZKguKgZorejsn8o7NXiZ9o").unwrap(), // SERGE JACQUES DURR
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DnkzidtPueVa3G3uhF2AaJEVZAoDE5r6FYFNYspPxwFVoCX").unwrap(), // B�I NG_C TH_Y VY
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Fsiq8JxVvgLkLLCLa6qXmRyckeVM8VaHDhaPg3BnFqGhpDX").unwrap(), // ISMAIL BIRICIK
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FvEEg1UNuAWNVwYNNKfRvMxjGAP2khzgbUgLNbHjadafRcR").unwrap(), // BARTOSZ RADOS_AW KUSSOWSKI
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FYp4JDjAipuygfAP2iJdAzWvLL4cQkzMQmiDSH67Bg1uF39").unwrap(), // BURAK GOCMEN
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FdtudHXTwd7CgTV8MLvqkXtK5xYF7Yht7d4Y6pTvPEw5NvS").unwrap(), // JULIAN MADDOCKS
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5EZcJue6Rj4t4P9sPE36HMSTspgr3jvdXyt4SECh25Rim6yD").unwrap(), // IAN GRENVILLE JONES
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HnPZGebXciqUQioe1x7MMmKW9w7XMoqPAp4zH8nMe5FsR2i").unwrap(), // ESAIAH MAKANA GIDEON
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Gj5d8gqYi7y7MbDbd3RBp2JATZt4bfMbQwiZBbCCaKbz26j").unwrap(), // JAROS_AW J�ZEF BRZYSKI
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GbDK1asBajZ24J2z49zbvW7NT4m87sP1zjxmcv3RTNYnYup").unwrap(), // JEAN BAPTISTE DAVY
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GCUMmR9cpHD4uYfRc3nDSv4346yKHhPs75Tey7o5na2HWDT").unwrap(), // LIAM PATRICK NADEN
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HbJv4X8Y8ahS5zz4DhTA3sWkB3BZoUkkMTbD5Ar1M9vV919").unwrap(), // N/A STANILAUS CHOO WEI EN
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GmXNpKo5GfE9kHg7Hsx6ALsLEdHBZ9H87HswUMMDbGvUkVp").unwrap(), // SANGITIANA FARARANO RAKOTOZAFY HARISON
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CqTaFsVWNWpw28JKtmbXiEBhYcgGaQZetLvVKMdYQCXW21j").unwrap(), // DARIUSZ PIOTR MIKO_AJCZYK
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5C7cZNnFEtUogFbJiAHan3NEmBm97bWeoXd2uPS7Q1Cj5Vaf").unwrap(), // IGOR OLEGOVICH IUD�N
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GeRi2bcbmTtH8CCGFggc14dW4cUCe2knzKkNNW292SN9i6J").unwrap(), // DAWID BOGON
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Gk5CoBvGYr8X5MfRtTKybvDxqHgGTbHzsW83o53SkdTSpPD").unwrap(), // N/A NG BOON HOW
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DZvatEE12h5cKSCpR74E71YR4Bt2778HBuhFKEf9zuU4hv7").unwrap(), // _UKASZ NOWAK
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GGqs1p77sagQzmCkkCPorESZjcwwi7s6YhQzQwfPcpGCjkF").unwrap(), // THI KIEU NGUYEN
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GzpZSQnoBEC2p2btH7JVTSBudEf9jGoKR2snTik7hmnnbXz").unwrap(), // LAURENT THOUILLEZ
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Gs2YUSyVBBKDNQ4f5c5xntMBszeDczf9FiwTFqA6raCWxPz").unwrap(), // ELENA ZORINA
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CVCRPP5bDgN8Yx5MhCWRk88fi1TcLxGK3pk3H751m9e6LSv").unwrap(), // ERIC HERV� ARCZYNSKI
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GLVFopWE8nsFNqmaTnVG38qVz5ckujC2dLPHqRyHcLm6vUv").unwrap(), // PIOTR SZYMON FERENS
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5EqUhi3gUAsVq7M6Aw87hpRoG5RqA8BURyXYvdNS2HM6d7Kr").unwrap(), // MARK JOSEPH MAYAO ORAJAY
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GEEgdk9Dc3b8higzm4sKGizBhccyeo56HDzrMLsGryPmi1E").unwrap(), // VINCENT MICHEL SYLVAIN BUISSON
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Dz4xfTyhfQSLuo3jUuenYbYjNQ654W8xoxttmvsTQAFSNCR").unwrap(), // ADILIE SEYT
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5E2iyD6QoPtbEi3WzqLkqRiwm69PGcxQukSYGJB7Ppzzs1mo").unwrap(), // PREMOMTZ BAPTISTE PASCAL BERTHELON
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GuRxnCQLgGFYEd3ntBFNJ9b5FXkb8m3Tx6ditjMN1uVxmJo").unwrap(), // DAMIAN PAWE_ SOBIK
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HTRrpYbecYNze8rkj7DdVtU73Nfny3wU5zsEoBTvzUXc8K7").unwrap(), // OMER ASLANHAN
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FHh2WkAvHtoRSncZ3fL3aYh3ZUHVrXnS1aWQchyfGG1quuA").unwrap(), // KONSTANTIN VALER'EVICH LAPIN
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FdxFD3xLvWH4N22eEnKoSXUncvrxxTzr1kY5ikvCc5ng1fc").unwrap(), // AQDAR MINEGUSMANOVI3 AHMETOV
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5EAMu9fp4t1WSX8v1SCFnqH7FMxQ58cGN7HVhqVRp4VfbjFm").unwrap(), // KENTA NISHIYAMA
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HMeLFNfmTw54aqJcBxbCBfAtbjrpu1qweR1XCCwVoAmTGdy").unwrap(), // MAIKEL JOHANNES AAN DE STEGGE
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DMSKMEh12ht7s2rFE1my7E2RytGtxteuNXeALHcCdQX6DNw").unwrap(), // GOKHAN ATALAY
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5C4y2LNJ56yEbvuv1ZRfpHrfVjnE6J5yEKjzd7j7YDfdGyST").unwrap(), // DANIEL CHARLES RANSOM
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DaRGevDLQud3ibceEtryCU5WCeiAGniZhuExu4pfAtnpo8C").unwrap(), // THOMAS BAUER
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GVteyLCA8xdW3HcvjJ28GVNLaEJeMKxdYprNYBMJxtwz4fu").unwrap(), // EMIN BADEM
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GESrLcrHjoPhHgh3cCBAsLVeVLFb2C28cP8w2c6MphPeXqK").unwrap(), // MIKOLAJ ANDRZEJ RYBAK
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5E47tcu2YQmvkXeFn7JCNJErYLjS9hwXh1wBeUJuvtXi8PVm").unwrap(), // SANTOSH KHAREL
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5G4DQvdUkE44MihjAs4BYn8qugjTrHUN5BFne6SadHUDEwab").unwrap(), // KEVIN WONG JUN JIE
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5G1DNz4d59vXXb5KXFJWVthdiJH2KqQLnHiLbdotqcZv5wYa").unwrap(), // KARANDEEP SINGH
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FkMwUcPSWsuzFu89a7bD5BL9NzUZCfmZu5KQrx8g3vNhbjx").unwrap(), // ABDUL MALEK MALALLA
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HpSm9Tr5tAdHouQdeJUVTSP3dBwnXWsPJyhC2hSThZg4D9p").unwrap(), // HOSSAN
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HbjQALeKB34c8AQf5skL9xkV6wnB6X293TtsRhDDzNNkP8Y").unwrap(), // MATEUSZ PIOTR KLI_
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5EyqKHCHKSyJQQ7nzdKnu1JNUt1r8c9fDP4BkHKR1ucbzwJo").unwrap(), // VAN TUAN VU
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5EfFRWfTWTCeMXkRADvJ7BmwBzRnyDGvpLKb9ycfajuLS9tu").unwrap(), // MAGESHWARAN RAJENDRAN
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GQbNUTYdqThDURZt9cAuYvgUh8ekq8VVhoGwrF34hSa8k6m").unwrap(), // DAMIEN LOUIS VAN BRAGT
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GnGrfSQ5g83mhWKboH17jUZ7WwJxeBbxkkC5eB1Ur5EyWLd").unwrap(), // N/A FONG KEAN LIM
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Hp4zcVnCxjCUotQwymZFG1prbn8KwPE1dtMaHJ4CGYK9GTN").unwrap(), // PRZEMYS_AW ANDRZEJ SZURYN
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HJzvkrcAvj7PgqyoMMr1KByCXL47mfbVPx2aWn3RnHAge7f").unwrap(), // _SMAIL BARI_ SALMAN
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GCU7AuxsGbKAiYzfeqFSmYMqVezPQRrV8khacg7TNKU6fVc").unwrap(), // LOIK OTTO MAURICE HOMMET
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5EvzgXUfUU74zhUJy1vXTQeV1nsBfVMcaYbGUsTV1vxYgdBC").unwrap(), // ALEJANDRO GUI BASCUNANA SORDO
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5E7cHFXWqPTif4x2fNj2Cn3hR1DzcogyTB46ba9UWW4jikt3").unwrap(), // LISA MIREILLE CHARLETTE COGNET
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FCYQwkz6gLpm2zn6cecQrex4d8wtH8tTh5GeGHjSUrYRcpw").unwrap(), // BARAN ALTAN
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HCFBPBhiyCi4gH1j7tWQjQ5cR46fWvya7f5nT2g4L1d1EjP").unwrap(), // NASSIM MOHAMED KHITER
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HQ9TEqdC2PstXxX4UL5kLSUftEw7C9YZhHGNhqZs8mpnSJv").unwrap(), // VALENTIN PIERRE CHAPONNAY
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HmMrpbqkdqEsaSuW8PfVqPi37Jk7pGSLzHKpJTSy7xFsCG6").unwrap(), // JAN BAPTISTE VALERY DE BRUYCKER
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GuytJKogtzZnehCrw2grrJDXSCM8oHqC7sQE9kqsejxvMkq").unwrap(), // PHAN TR_N TH_Y HO�NG
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5EWfj38RieR6c438imVhUyrqN1e3xupqjmMco4hqYepUvRJM").unwrap(), // TOMASZ MICHAL SOLAREWICZ
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CwcJG3AU8YhoZsdH78j2B8MCDhqxj4hkPstuB6b3VyWkWsj").unwrap(), // JEREMY SCOTT SNIDER
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CFpD4Bzr4C6ne9TGTXrtWFyYV8C7i3jgku8vQdfpH2Cv2DX").unwrap(), // MICHA_ TOMASZ SZAFRON
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HmB6hQw96E3a4pEkvnTTDts9NfPYS2Ef3h5x95LXLrf648u").unwrap(), // AHMED MOHAMED ELSHERIF
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5EvDqZaMu7ay3zB7xqHWihj6GgTD6Q42hoVfGeddAcJwEB2C").unwrap(), // JACEK MACIEJEWSKI
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Fgud3XB4cmLXzfBrRpb8rWx5Zy5ekrsk54KgrjboKRB5FgD").unwrap(), // BAHADIR KURNAZ
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FWP4y7g5LSXvVQT6esuAT5uVTTLqVHUduwY3VmmvVkRhuAD").unwrap(), // CRISTEL FONTAINE
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FqUpd7Cbej5kK9CynMn1s3qyKtkDw927HdxxS3bs68v94HU").unwrap(), // MARCIN EDMUND IMA_SKI
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DDVNLHmhTgAJQEgN97kQ7RHdRPhya8S2SFa8PoPwoom3fJt").unwrap(), // CIHAN KARATAS
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Hpzw48GDUtc2X8jfNEaVQ7nA1hNvn7qHcS11btxW9QQJMLb").unwrap(), // HUYNH THI MY HANH HO
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CASDXMydcH7kGrCb2cVwwPiN7RviXTTdxnnVDYL29U1wbMw").unwrap(), // JACEK KRZYSZTOF JANIK
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HBHaYWWgaTr19a6k9Go188JYzVn5KZACN2VAGdjkjaASJLc").unwrap(), // MICHA_ MAREK BORCZYK
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CA1UYPyPGS3kcBS6vJ44BRzXJYmN7AGD2ULNucmRkzgVeU8").unwrap(), // SOM BAHADUR TAMANG
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FtHtRtL7WbTMKJWufBXsVm9YWKX7Cs5B3q5uWqQ2GpWLZQW").unwrap(), // CATALINA ANDREEA ROMILA
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DUrurNNX2CkD6stiQCvMEFu1Fo4tzyrefK8iVCKgSxq64dg").unwrap(), // KRZYSZTOF _WI_TKOWSKI
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CVTW4StguUEHWmreq4Gi4j8TYco9cu81UvzXBvXT99P3sws").unwrap(), // KONRAD KULIG
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CJo8o56fqH16EK9cYG2jZtEjQubNDRE2XTHVLMVcC3zYeoT").unwrap(), // LUKASZ PAC
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5ECh6snLLitBHEFjtBpCYfwUVC3rT9rEi9jzusCY1kBhzK8A").unwrap(), // ALEKSANDER FRANCISZEK SWINCZYK
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HB3NoD5a38or7i7jFpJnNB9sjqHdDxp61qGdHjPHQt3zPcx").unwrap(), // ERIC TAN JIE JIAN
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HnNA6rGjrTL72Bia6MozSA6u7FF4Eo2VKt7AdUGHVMLss96").unwrap(), // R�GIS BOLIS
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Gn5ZGaZnExKSBgz3N2u697pt46mnXk81L8bQo3Rm3kw5u7v").unwrap(), // SAMY RIZCALLAH
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DSep7DZop3iZ1BH9S6nR8gqt6PaebKoJDv4kTNvMbYn2aod").unwrap(), // SERGEY GUNKO
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GEEUip4kYDtQwYFfaZuSVBFZjopND24SS2ja652j6GrCSVS").unwrap(), // THUSHARA SAMPATH ADIRIYAN MESTRIGE
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FjdQuqJaYwSCfz7PUYp1hCcrJT6K9xjvtECi3V1YSRcaWqM").unwrap(), // __ TU_N B�NH
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HicdXFYJ1iSgE9V8qvvwFKKiwr9aNpEoyihqMndDn8nEXWV").unwrap(), // AHMET DEMIR
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Gk6ApfG2d33cT1UCvb625qfkpG76wFoSNCJH8wn9Ss5LzAT").unwrap(), // AYELEN FABIANA SALAZAR
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FBwGUuN4V8w4BQFtBUZg56TN9jtKdNH7xS5kkDcrbVamwGz").unwrap(), // KONRAD SEBASTIAN CICHOSZ
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Cr9k1YzQ52atu5z6BFbnotTTHT8KeXUqDA4hhwdHWTib84v").unwrap(), // GOUALI MARTIAL ERIC-GUSTAVE DIGBEU
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5H8yMjWuzEGEmhNyFbg5JhmbwtU4pY6XDn9oNwr9dnfo4BzH").unwrap(), // SALAHEDDINE BEN BOUAZZA
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HENNyFZuhGjTKfo7foz58Zs4BKcbGAaMqSKCCV5oUA2ZVAf").unwrap(), // ROBERT JERZY MERGNER
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5H48J6WELGUXsQhjcrfsrNBkNdyoQ5vBKG7uYtuqFrj9zWGP").unwrap(), // TR_N NG_C L_NH
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DHbzqsv4w3CQ2yMpshJHzgMo4pohoFZ8yuTbnpEVu5Syw82").unwrap(), // PHAN DINH QUAN
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5H1cmQwd1jRKYHspCiMRNx9pVqaJCEa4qTdw4dxVWZx8EHQv").unwrap(), // DMYTRO KUZMIN
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CfPHH5GtGkL5D9WZB7RTgCrhxfstpxKwmuGxoZc8tUxKQhn").unwrap(), // WIJETHUNGA ARACHCHIGE MITHUN HASARU
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FLFZeJec63MY9npda9AwKVJNfRJoc2kJ4kC9ngXaUCc4rWt").unwrap(), // KAMIL D K�_ECZKO
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GHLMzsx6mWYwbWcBSFqnQQVP7DccbQjgMNxJtceQ4piSRio").unwrap(), // OJO-KOLAWOLE OLUMIDE ALFRED
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HbBy9xHxY844Dm9whzhzn1nSF92E5knz6jYCsUipR4SfKX9").unwrap(), // ARKADIUSZ KO_EK
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DcsDuTHLfFmGWtmZW9EEvWWvZEW3C6zQbgNUiADTJiuMfgS").unwrap(), // TAN THANH VAN
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GUDK338Kkwj5tqzsgYq8nCqFupR8PYPrcDKTFpReBJnd4uu").unwrap(), // NDWABA ELIZABETH OSOBU
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Hp5WQx2RmJwk1W14xUbjYfppdvCiFE1RzPjuRfX1GykMa7T").unwrap(), // KAROL MAREK KULASIEWICZ
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GgvWf633Eokey5wSaJ4LMUxdDFgxLWhCcxyEbCWe4q7hhRq").unwrap(), // THOR H�KON WANG
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5F2RRb5XNyLkndVhDBgG178PPprr7tLTumGhCmMCX7FY6BLd").unwrap(), // IOANNIS TROULIS
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CfLNHsqMydp3MQ5LC6bCnTQu4nTD2nqgNWNb5NkurVUxjch").unwrap(), // ARRIS BOYACIYAN
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5E9pNsPCCjr8MvY1o3Katzx3v1NgsaXVP14vpvkp64z34Km5").unwrap(), // OLUSHINA A BASHIRU
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5C8ZRhWGiAQEjnPkm27zKY6KMD8r4BZaWEiEwMQTkVgj18vV").unwrap(), // MURAT F__NEC_
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DLFCSjPe89FLn75G28vxphTWEfwyEZvEJQXDAW3yZ5EjTkB").unwrap(), // LINAR LATYPOV
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Gc9SnsHkyqMX37oHWwgS9hBtuuSLujhsth6dXhdZkQgnD9u").unwrap(), // EMAM JAFFER SADIQ MOHIDEEN ABDUL KADER
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Dc489UukEDKmRLwNsUvuWbRyjT7ErQ6gvZtDX9wRfX5esQc").unwrap(), // JULIEN BAUER
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CorQdsXpjvQj6KfMa1BMya3n9WVYgxpeBiCEoETrToLfQFn").unwrap(), // ANDREI YEMIALYANCHYK
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CPUwRy36yHJU3hMr3xbbKcRh9QNKg6mLhhJiGajhuBQE8vH").unwrap(), // EROL SENOL
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5EReZcJHjuQ2J8SJBpbUeBBWpuUhkjKnZmaCVqh2RvgCxvJY").unwrap(), // L� V_N PHUONG
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Gps3QiEBkr51e4YRJeE3FxZ2LbxpQLuA1x35zPQPb86Ubzi").unwrap(), // ROBERT K_KOL
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5C7zcdThasnLciuxH7BwWk7BGRqsMrbEg6hEd5Dbdg3SNBGR").unwrap(), // EMIN GUR
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Gsiyd4qCxv9q8w1F3VKZS8VizLKRUhqV9JJtEqz6DjgH7RV").unwrap(), // FELICE EMANUELE VALENTE
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DebCxcwKUKNUSooMvHbKVkCMqG89uEFGcMcRhA78o4x4Hz4").unwrap(), // CONSTANTIN DANIEL ARPASANU
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GEtQCQkA8ZRpSS9uA2TtoFEsxSoy3hnzpJFZPvjp2qcDDJp").unwrap(), // VLADLEN CHERNOGOR
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CDmvGjUMbUYowos5HmmGuHAaHbisgw7We3vc6a1fpEc3wHP").unwrap(), // GILLES PHILIPPE GHISLAIN COGNET
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DqVZo7msUazKrdwWhaxQoTEz35EqTn7G8d32WAZvzmUuAmr").unwrap(), // ERIK FRANCOIS EJNER SCHJOTH
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5D2vsuXN2B41LyvVLjDAxczRj7UDKxg8vZ85awK4JXpDWHxm").unwrap(), // DANIEL PENA
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5E7WRmoVhFMNiMsrfMDi9twcrzeqsh1StpaJ5Qmnpq1DAriz").unwrap(), // ROBERT ADAM PRASKI
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5ERCXojWmTGrSqSgHML4dZkHV1mbB8gybmSe2uWMb3BnvNbg").unwrap(), // JER�ME BERNARD JEANSON
            75000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GR3hLLqPm7t3e9acKPcUgFivA5tG4wb7HUkZqxze4DJsS9M").unwrap(), // CHANGMIN AN
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5EFD9ppXaxmXiQUqwvQ5yetCU7W9AA6YMdMC4cENCsM1w4cL").unwrap(), // BENIAMIN _UKASZ FOJCIK
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FxwgVjHiK5fxp5CBeHSTVLP6H2qSKPLvvCdfMh7gcJdYspk").unwrap(), // WASSIM ABOU AOUN MAJDALANI
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CXr539EJNqsJh5uzj7Ad21eiZ1uMJafgWa6EyoinwUjuDrL").unwrap(), // GOKHAN UGURLU
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5He8AdBZ3NRnrofaPJ1fNcmJcsHMxi5aCSJvv3k6CL7Uh89H").unwrap(), // REKHA V
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Co3wwpjJwDueAniMhCvEKXY53c412rRTpXqt3paciKKmFAJ").unwrap(), // BARBARA MUCHA
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5EHkibFtQLJBtMknMt6GK1ivQ57LPyQABoSg9TLJAS23jv6E").unwrap(), // KAMIL JERZY GOLEBIOWSKI
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CvyJDjYNm7SF5hzvsBkcepPc1xRkPYiM4LpA96PVTXh8Tbs").unwrap(), // NGOC HUNG HOANG
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5E4MkdbaXoo8y6HyzMQMUESjDzurEVYEcDQRxQvokPV96jMf").unwrap(), // BEATRICE MIREILLE SONIA PIEROTTI
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5F1Pb8dZxmrKh8ZDvmFcdV365ue7e36ZqLMbizmwwtJso4bw").unwrap(), // BENJAMIN ALEXANDER CORNEY
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5F9pnipps8oZK685cXmB8Dpv176M4CgGyE2xQVqEz9FyoFkS").unwrap(), // NAWRS KASHOUT
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5G6t6a78XmdBnbxqEyo5xhRYJUSz9QkNisUPgznSSMYbLiQV").unwrap(), // ALEKSANDR BULANOV
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DebwzrFf3JknUj1uof38sR2kbqzwtvNY7R9dxC4bqBukr3W").unwrap(), // LALE COBAN
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FUaQ146MTPUxzsabJF66N6GZ13uLrJsrbaqKG6fEx36c1Rj").unwrap(), // TOMASZ ZDZIS_AW RADOMSKI
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5E9Yu84K3kiZGPBuj1ATL84rYQUwn95ckJvJCLq1BVi1DFQL").unwrap(), // PHILIPP HOLDMANN
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Ca6LWiu8Jq3vN7xsaXy2tuhWzDgDtmEpHkBJ3SLTeDHQXsN").unwrap(), // SANJEEV MENON
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Hn7EaKxJu4tR9keTNL4gZDsUfQoG5r4qtMif58gcA4fSsET").unwrap(), // MACIEJ JASKULA
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CJsAiwx4TQqZx746L4MQp2ojzjrBQTHGmNqHE4pzPxe2chT").unwrap(), // DMITRII MOROZOV
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CJPEDH99tmA11aR1pPzunvChWd9L4S6GPnRGHcNsYPHn2oy").unwrap(), // ISAAC ISRAEL BAUTISTA RANGEL
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CUMMm9jBB3JsLs4r7YJfxgbi5Znf8DPZbhEErxn2pjTu1SM").unwrap(), // LUKASZ ZINCZUK
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DHoAuLBEqEpSV7YEW5porEpSMDnpJoYCupLyQa8gWkzy4hZ").unwrap(), // GURUPAD DUGANAVAR
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DLcLEKd7dzRegwBMRncJnMhJJCQEboFmAhU3QwSHiCgxvYR").unwrap(), // S_AWOMIR JAN GR__LIK
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CQLMSa3LSGW3EeWGV4GxEYRKB6a4zuxsqFw1kUDhvezAnkW").unwrap(), // MICHA_ WOJCIECH MIZERA
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FEk6bFte4y7guQeHDuusoeehNrwUkBDJ4RFLQ6bzWrqwi7y").unwrap(), // MARCIN KAMI_SKI
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5F1kmu133XZktCLBbbEBMLxtCevTECtcX2vGXFQpL1NsQSvL").unwrap(), // ELIGIUSZ PRZEMYS_AW SOBEL
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CJGBYo6nHU9veGafBWYW9RTZRcU7KHnyz6uTuQXG8e3BGF2").unwrap(), // ALEXANDRE XAVIER DION
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HbJHGU25io7JCKoni9jjrxG2jnWdiknfxU63mAJMEZkvmYz").unwrap(), // RHEINHARDT INGVAR GERULA
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Gx2PeqSyiWXgDoa2ntrxv9G14aJTBfmGEXZMzYU772GNF3M").unwrap(), // ___
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5D2TFuGtzJ2JFBPHFgijNebgomrx81G9e7RAWCDH1adRLJV3").unwrap(), // ALAA EDDIN BAERA
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Eeun9BDEHQXcZB4YmihUoZybcaqdkxkNPJcY9dpTSVxpXTz").unwrap(), // KRZYSZTOF PIOTR KUBOSZEK
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Dq6Wpsf1vKDsH4q3jVngmfHeMtCHgDRsWcQnUUk1xNDBUG3").unwrap(), // FAIRUZA VALIEVA
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CkbRgvtiCC8sNt1z6mTUCqvh3e5LmJD5jfEPX6Jn6epL6Hz").unwrap(), // ROMAN DUDCHENKO
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GRM9uao4EeNyE9pSMN4soDF7omgaVqJutAjAgokTvNyxct5").unwrap(), // MUHAMMAD KABIR MUSA
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CaVkagPsW3whEkn6GczdNCSZxSRF3aLUnNX7ki3797mBDGs").unwrap(), // MATVEI SHAIKEVICH
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5F45KEELCtM3EAkxz2Qog3u9A3QBKFxGXxAMWkZNVXSVm9Zm").unwrap(), // QUENTIN JOSEPH JEAN CLAUDE FANOUILLERE
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5H8rdq8poLvjPTbPWPus5z5dMN8D4QaPRweoa191nYuJk5jR").unwrap(), // KAYA ONUR G�LTEN
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FhZgrDQxPxKVw1FN5BgeoP5QRKTw5rBEvExEeoamC89SaKa").unwrap(), // ALATTIN SANCAKLI
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HmMEvxoq35M97owjwqFKjxpHUsSnLHvey3ePyTMxKyjccr8").unwrap(), // AHMAD GHAITH ALSHARIF
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HTdj7twNHdwtWY9zcyAnFEtiAVSjx7DpwoeSZbyKzj6M97s").unwrap(), // LUKASZ BOROWSKI
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GTk2jNe3dJNN4Uj3qwzRM1KNPSm2GD7eUht5h7QAy5wmd1q").unwrap(), // VAN DUONG NGUYEN
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Fs2UQNp3a2C6gcU4zgHjWUBitN4eKkwsdbG7umqwCLDmawb").unwrap(), // SULEYMAN CEVIK
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GLCPXD95akd9zvVx8vo9wD7qvwofaHWsSEQPQxzq8XpfUEY").unwrap(), // MUSTAFA YESILYURT
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5EcnvRnYWfGoa1NoZ465i42Qk2Sa8YTuZmfHJbQsAjKnSujN").unwrap(), // HUMAID KHALFAN HUMAID ALSHAIN ALAMERI
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DRTXnFHDAxztCdEo6qB7EN1V6fK8jxCrdhizypM5AA7zXoP").unwrap(), // HO�NG _�NH NG_C
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DhUMwaWjfiioE6WDXpcWdES6nmGdoMRZ7AzskR144a6CF56").unwrap(), // PAVEL OLEGOVICH ROMANOV
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CM8gWNaPuAk5bk2R8mSGUesYvw39HsAFgdViC4f9eLCPaze").unwrap(), // K�VIN NAVARRO
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HMmZWR5mrUGLAZHEcBhp34RVPVJ6DaWGjKq1pNcZPMvuCGo").unwrap(), // PRABHAKARAN A
            150000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Ek6ypxFwx5QgJ2MvuztfhFJhjhD41oXggRQPT7H9WHHjaaS").unwrap(), // BAPTISTE S�BASTIEN BUREAU
            375000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DNms3kq3pThyQjhsGrdUBBAQLh6JdfGzXvucDTiBKFNwXnb").unwrap(), // RAFA_ KOMINCZAK
            375000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DJQqn7Q4LXofswfhtpqZNws44U6JymhzKWGgUs3AKpWfurw").unwrap(), // TOMASZ KRZYSZTOF MENTEL
            375000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Gzkrn8wa6u6M8C7qoYKMuCwdNYqusRn5tDg2FoCQQCYaJR8").unwrap(), // NHU CUONG LE
            375000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DNocQ1r64vk8xXhx3qC4VUcDo2yARMPfEoGGhRjEFBjkTGR").unwrap(), // ESER UYANIK
            375000 * DEIP
        ),
        (
            AccountId::from_ss58check("5EFMNTTzujmzwVfwA6W5zYfGcRCNkmsHc1BjD2cUSfTa751U").unwrap(), // MHD MOHANNAD GHANNAM
            375000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HMsSWJgXw486DKzKJfmuD5K4BdUeS9ixaVgmDKjJJBdYUPR").unwrap(), // ABDULAZIZ HUSSAIN M MOAFA
            375000 * DEIP
        ),
        (
            AccountId::from_ss58check("5G3emosXFDpUbRV4dzF67Zd99U2hGxkB3mmegJ6g45h9bZGb").unwrap(), // CHARLES CHO
            375000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GLFLTjFDoHPFFCRQ8YFvaBAZzCtBZBCv9K1maJxqD6QQV4v").unwrap(), // TOMASZ KRZYSZTOF BARAN
            375000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HGt2Q6REHK89YKTozBLDDqki2bFXP7L8qbU2iM7xBqwsczo").unwrap(), // JEROME JOSEPH JACQUES
            375000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GjfJHTSY1JTgGqSidn4LnFpwxcsuZo3UJAqCEXvuM7cFTbn").unwrap(), // KEVIN LAURENT HOARAU
            375000 * DEIP
        ),
        (
            AccountId::from_ss58check("5EgwYPk3bWYTzwqAC6Qig2bYsMXVAkVb6SWC1JpZpFL2qs9w").unwrap(), // H� MINH KHOA
            375000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GYcE56KkPfvFkrrChxQ1bX78hgqAhwJkE1ffaxVXmXRteS3").unwrap(), // ORIFJON TUROPOV
            375000 * DEIP
        ),
        (
            AccountId::from_ss58check("5G4Zxitrc91nWnYzZNyeHWyJsMeuWHu2uJjMVqEg4dPcTD1j").unwrap(), // GRZEGORZ LIZEWSKI
            375000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FBpniJMt1MGg8PaP8rDZRU3DbHbM6E5sLkVokuK3xrbh32i").unwrap(), // MACIEJ RUDZINSKI
            375000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HYdetyjfQBUi8u1tp6j6xGXeLuvoNGWFrK86d8MPFbfhE6Z").unwrap(), // ADEMOLA MUTAWA ADELEKE
            375000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CJhB914fmc36aXgxMBdjG86zbrgQ6JFUddYQ16R12hzcrkf").unwrap(), // NICOLAS PELARD
            375000 * DEIP
        ),
        (
            AccountId::from_ss58check("5C52JPV1reDw49FEy5WcEzzbkqpaQbDiTzSYbxHBi6YZGy6L").unwrap(), // JULIEN YVES SOTTAS
            375000 * DEIP
        ),
        (
            AccountId::from_ss58check("5EEywi73XUYYPGfdK9urHmE9kzFWpkDZkLUrL2BGe1BRr98y").unwrap(), // WOJCIECH SZA_AJ
            375000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GZDHnkgr482kAaG8ZJjfa7rDqGs5eVcghydnNyPAwfN3NGY").unwrap(), // MI SHUG MERET
            750000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HKoudvvAmyr7M67iDswU3eni2riMuwE5xX6yShBY1q6U67s").unwrap(), // CHAI HANG CHOOI
            750000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CA59EKuBWXW8tc4G1ZKGurczn7WWakme9CGnA61dK17mYeG").unwrap(), // TR_N TH_Y H__NG
            750000 * DEIP
        ),
        (
            AccountId::from_ss58check("5EX269zEYT6GUAdyuVYadJjCrHZGbYHH9sZPMFr6CN7bVUs9").unwrap(), // KADIR ASLAN
            750000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GHRXyJrvJeUXExS6oNvdZdqy9Hn5MdtqCtVM9Ni3nUjv41B").unwrap(), // MYKOLA VIITIV
            750000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GCJSXLB2kzFF1XXRVQ1exg6nwkMBYY7pCLGs42YdNR61AP7").unwrap(), // SEDAT GUELBAKAN
            750000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GLQCYsrQfR1GtXwtpGyRQ1hsyhN6yB9YM5S5AyKvm4ExRvX").unwrap(), // SOFIIA ZINKEVYCH
            750000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CRtAAFZ7iWTn3iPyPN5aRsGVof72z4njP41QXzSA17ANjaE").unwrap(), // ABDULLAH NAC_ KULA_
            750000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FPDCX61CsbLQn8yiDm7n4zZE3gqJ28DUtDnzhTzNhJ5qgfA").unwrap(), // MICHAL LUKASZ POWA_KA
            750000 * DEIP
        ),
        (
            AccountId::from_ss58check("5G4122JFTzaL9C3sKR5ZRW8M4bgbVzXmZYPxndPyUS7tr7Zc").unwrap(), // KEMAL G�KDOGAN
            750000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GKNrruGWJ7WQkuZPBNVH3RCGRHs8hsFDcGwTHNacwRv232u").unwrap(), // OLEKSANDR BIZINSKYI
            1875000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GWoHGn5SE6xyywKdavCKERdkcBEDzvcqHC5JuLkWW2nMJBP").unwrap(), // DMYTRO PUSTOVALOV
            1875000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Ew9VUt2U7xu3XswQU7DbfMefyAP4P3PKKceJf5SvAdtX1QF").unwrap(), // BERNARD E BURST III
            3750000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CvdKkEGRsMDjkyiyrBhiUdf2zmCzXzbQeVRQsWV2sqihzWj").unwrap(), // ANTON NADTOCHIY
            3750000 * DEIP
        ),
        (
            AccountId::from_ss58check("5G3p1RvBR8ZHjpJGcSq4ncfkbi7bnaHujUr4MZnMfeP96hhh").unwrap(), // SEBASTIAAN Y L VAN ERNE
            3750000 * DEIP
        ),
        (
            AccountId::from_ss58check("5F4cuDdz7kQkcTYVWdQ3dWijij8uEySAZYKdGXHyqtHjxsZe").unwrap(), // PH_M THANH CH�U
            3750000 * DEIP
        ),
        (
            AccountId::from_ss58check("5F6kwQXjHiui7ZwsA9AFg7JTtiMtZDmC5bNGdcFUM88TANF2").unwrap(), // OLEKSIY STESHENKO
            750000 * DEIP
        ),
        (
            AccountId::from_ss58check("5HTP7FuWLN9ewvksUwen9F8s3qU9vJb2Sh4QLKJ6R8u7t25U").unwrap(), // PRE SALE 1 UNALLOCATED BALANCE
            13650000 * DEIP
        ),
        /* PRE SALE 2 ============================================================================ */
        (
            AccountId::from_ss58check("5Ejq7fBCnK7w8CEN94wMWtYpmr5FrMaZK8PWhjy8i7SQcC65").unwrap(), // DZMITRYI SHCHATNIKOVICH
            15000000 * DEIP
        ),
        (
            AccountId::from_ss58check("5D9vjB39SpUVtA6rkrtro9MESr4yzr6JxTB1tNVd49t2mv5n").unwrap(), // VALENTIN SOKOL
            900000 * DEIP
        ),
        (
            AccountId::from_ss58check("5EHxPn4GjSBAum9JF7rSrcpHkDWErpHf6sBexhyBGqzgfwj9").unwrap(), // GORKEM BEREKET 
            3000000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FpxzrSpUMp7pngGxzFtnnyKapb9AjypGJ9Kxbnb6THwF34L").unwrap(), // NINOSLAV SRETENOVIC
            6000000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DMD9DWmKZohxuqAFsAuy3poqinybnAp6qttwwr3MFyPrfxi").unwrap(), // PIOTR PAPROCKI
            1500000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GsYznNyotNJn2uyBYevPGBXPsTWC6NJ8WUhKwYmPLk8DL8P").unwrap(), // VITALI BUBEN
            600000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DaM5Bzcj75skNtPCuZPcgAhL5vgYJQWNKtG1EGMwYsBQ3x9").unwrap(), // LI MING-YEH
            600000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CoMRyUo48KmdrQZSb96UBRX6vBgFRdAxMGQxiLMWXGn4xoU").unwrap(), // SZYMON MIOSKOWSKI
            600000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Gh5sgV5GL8UvapJQ5D39nRN8UkRCxwRN7H9LjgnoQeAGAES").unwrap(), // ILYA FIADOTAU
            600000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GECsYhbYjDifRWnBbhrg5MgT5eJ5jj9EJTycAFiWqBBe5t8").unwrap(), // IHAR KRASNIK
            600000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Fh36EmGgZDQKdrj3q123bNL8sKMZY3f6wEbGNJotyHQ2JkC").unwrap(), // WIOLETA KOPYCKA
            600000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GxbE2r89cdV3U37GwjEvNSf5EakXumbSe2nym9ZQ5YAJpUy").unwrap(), // ROMAIN (JEAN-MICHEL) AVRIL
            600000 * DEIP
        ),
        (
            AccountId::from_ss58check("5EoFsYhMgkLZnouhhUZTX5rtLF4onSYBV9Bjqn4HfaKCa5ZP").unwrap(), // TOMASZ KRZYSZTOF BARAN
            1500000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Dz8GMVA3L2tDJYVtjGLHKK1EzTZwzX16YYjELMe7B7iVGZT").unwrap(), // DAMIAN CICHOSZ 
            600000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DXbsLbvQ5BAdTEG75iypqmkSrujG4wTZp2xdCMZHLdzhJ5E").unwrap(), // ANDREI ORSICH
            3000000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CMHN7W6bDQ7JBnoCpe8cnRxH6SVpoUfxmAj35GKXjHaVJHE").unwrap(), // LAMANOSAU VIACHASLAU
            3000000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FeJLNDcpKat31UsCouLPhAjVsG5MxmuTduxH8GtNnU1pmcf").unwrap(), // PHAM QUOC DUC 
            1500000 * DEIP
        ),
        (
            AccountId::from_ss58check("5D7AhhKY17fQkpmYoVjZ7rJpmfQWBAnBRUK6UY9UybnK9Qv9").unwrap(), // ALEXANDER SCOTT COLE
            600000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CFYSNuo4Xf7Qzhoi9kvUm8BbkFDHWvS5JiSSqzbYWXHZQPb").unwrap(), // KEMAL G�KDOGAN
            3000000 * DEIP
        ),
        (
            AccountId::from_ss58check("5Cd9CMi9nyfXGYRzUxE2uzMxURUXC4E8o8yRU6TbQLyiDmpM").unwrap(), // BIZINSKYI OLEKSANDR
            1500000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DSc3NK3D2N8fZ3Mu7bz2kWC5MiWiw5q23E2uKijNv8oX37Q").unwrap(), // THIJS SCHRIJVER
            1500000 * DEIP
        ),
        (
            AccountId::from_ss58check("5FgikPX1o9jFfmpkgLVe7H6Mg3DZVvMGgk5Eo51S4moXWqFA").unwrap(), // BERKAY C'ATALYUREK
            600000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DUXdT1M99tMNuR1E83kSMk22SWUhufmh1jCegdeiT5WFgHv").unwrap(), // VERNIK OLEKSDANDR 
            600000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CexETqnfLugG4HAnaK1e32UYEqfAobC7zcmwuvq2FuquwAi").unwrap(), // STANISLAV ANDRIUSHCHENKO
            1500000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DhBryXMZgmKG9nXmqYWsU2NnFZFerGpt2is1k5LmHUNqx2J").unwrap(), // STRYUK VADYM
            600000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DoL5yzLuiir4vHk5gfN4qQkbeLtMJ77SymBcBaYDu7UB9vG").unwrap(), // PRE SALE 2 UNALLOCATED BALANCE
            7600000 * DEIP
        ),
        /* PRIVATE =============================================================================== */
        (
            AccountId::from_ss58check("5G8mQWUT29huVVx6owuAoWBuTLdL5uboiD1qMoysxn7u5xo1").unwrap(), // ALEKSANDER STEFANOWICZ
            10000000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GNGowiCgZSxbXvuK11kraPgUKRrN2dEfuseoze2UmpcckEf").unwrap(), // ASHTON ADDISON (CRYPTOCOINSHOW)
            1000000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DqWNLhwZw2ibBoYvBA5TRwYcsaZhrsVPWCjk6z5XgqwYBbh").unwrap(), // FAIRUM VENTURES
            14500000 * DEIP
        ),
        (
            AccountId::from_ss58check("5GjNxHr9hwkgyLtWgVtxjnYMsd6EyquFJLkECDDmTFgs2VUQ").unwrap(), // GAINS
            20000000 * DEIP
        ),
        (
            AccountId::from_ss58check("5DDSHCVEzvVX29nFq9PhXeJDSKxTXoaPBox3qRvqUHTWxp3n").unwrap(), // TOKENSUITE
            10000000 * DEIP
        ),
        (
            AccountId::from_ss58check("5C7vLmAZues23ZTCtVfvGtZSRygPDWndYwBhFhW9yHbRHEKV").unwrap(), // PRIVATE UNALLOCATED BALANCE
            67687500 * DEIP
        ),
        /* STRATEGIC ============================================================================= */
        (
            AccountId::from_ss58check("5GubXsYvvjsyf4Pi3jE7w5B7W9c7gZQWXMJpNrvHKyaX8Vmj").unwrap(), // BLOCKWALL CAPITAL
            3541666 * DEIP
        ),
        (
            AccountId::from_ss58check("5G9YgYppcvsfCNWEMgKmFf2PBsFgezpHt9Yb4FMVyozaZp3y").unwrap(), // FAIRUM VENTURES
            12041666 * DEIP
        ),
        (
            AccountId::from_ss58check("5DkCHxyuxENSVLvd4AqrTcYyqDJJJC2JjJpNXoqs6HEeWDTA").unwrap(), // KEF VENTURES
            7083333 * DEIP
        ),
        (
            AccountId::from_ss58check("5Chf4ksgxVoiSovkSBWYytxCG4GTTiE5PQ2SeZ8De9c9SZ3p").unwrap(), // SIMON SCHWERIN
            3541666 * DEIP
        ),
        (
            AccountId::from_ss58check("5EvCg4bNMHEcsKVLW5tXBpUed2tvhWCqQP89o9qXEVks996C").unwrap(), // TOKENSUITE
            14166666 * DEIP
        ),
        (
            AccountId::from_ss58check("5HWF2Xz2gr4abfbtcEZPPRaETB7Hhp7L53bcW4ZLnoyfEZz7").unwrap(), // RAY CHOHAN (PATSNAP)
            7083333 * DEIP
        ),
        (
            AccountId::from_ss58check("5ETvWZWHbviZTKGEGyHxk4BsVSrej4mYGJSccVSe36j8d6mX").unwrap(), // STRATEGIC UNALLOCATED BALANCE
            94416663 * DEIP
        ),
        /* SEED ================================================================================== */
        (
            AccountId::from_ss58check("5Exf3FTXKUPJXDWPvHQC48un7ZkZ4EFbZYPz3GavrUeBJHXm").unwrap(), // AU21
            11250000 * DEIP
        ),
        (
            AccountId::from_ss58check("5CSKm2aRkdtYEfuUDuHDYXhCed4eVffMoZoMLzHVw2Py6BTy").unwrap(), // BLOCKWALL CAPITAL
            5625000 * DEIP
        ),
        (
            AccountId::from_ss58check("5En3sVgNTmDTXB3JaYNq6kQ6sjncvvGZQaR6YcmJExb7LpsS").unwrap(), // GLIB DUDKA
            4500000 * DEIP
        ),
        (
            AccountId::from_ss58check("5ENXPZH6buaW3tiTx7kE6aHmyEZiQ19BwV4vqQGETC8qhhAU").unwrap(), // SEED UNALLOCATED BALANCE
            120000000 * DEIP
        ),
        /* UNLOCKED AT TGE BALANCE =============================================================== */
        (
            AccountId::from_ss58check("5EHZV8Lbm92vJ5aHotkEHFgsdfbmmzEpFTq2dxKpbjCG1WCW").unwrap(), // UNLOCKED AT TGE BALANCE
            60200000 * DEIP
        ),
        /* VENTURE CAPITAL FUND ================================================================== */
        (
            AccountId::from_ss58check("5HpfxMvEz737C2VqVT8SLaVviSbpkGRQMeqEzPh99dd5Yzfq").unwrap(), // VENTURE CAPITAL FUND
            320000000 * DEIP
        ),
        /* EDUCATION PROGRAM FUND ================================================================ */
        (
            AccountId::from_ss58check("5FRjkK2swczXPPUeSxBTVaUkXtywzzGX3spwopQa6dLssJZG").unwrap(), // EDUCATION PROGRAM FUND
            200000000 * DEIP
        ),
        /* ECOSYSTEM FUND ======================================================================== */
        (
            AccountId::from_ss58check("5Cyvp4wHSaHej7ScxP3PKSRt6sNkBv9D7ZC5sU9k6YoMr5eh").unwrap(), // ECOSYSTEM FUND
            200000000 * DEIP
        ),
        /* DEVELOPER SUPPORT PROGRAM FUND ======================================================== */
        (
            AccountId::from_ss58check("5GWBCvKuBP4tPEmS9ADsED9xNAa4BGvWQRCxuYC33Lmj98Uz").unwrap(), // DEVELOPER SUPPORT PROGRAM FUND
            400000000 * DEIP
        ),
        /* PORTAL BUILDERS PROGRAM FUND ========================================================== */
        (
            AccountId::from_ss58check("5H8tevgDw4FTBAgfmbxAbUxVj7BYXbgChfjsd8vXtqXtQqRB").unwrap(), // PORTAL BUILDERS PROGRAM FUND
            200000000 * DEIP
        ),
        /* REGULATION AND COMPILANCE FUND ======================================================== */
        (
            AccountId::from_ss58check("5G9iyTW95RsbmETF6WZeWdCSy7dsNCzdVMQbCTEbbwR3p22L").unwrap(), // REGULATION AND COMPILANCE FUND
            200000000 * DEIP
        ),
        /* CIL COUNCIL FUND ====================================================================== */
        (
            AccountId::from_ss58check("5CFvee5NMchUoMwgiUk3tocWZUCfnkHNbP1H4iGccA61iGsS").unwrap(), // CIL COUNCIL FUND
            40000000 * DEIP
        ),
        /* AWARDS FOR CREATORS FUND ============================================================== */
        (
            AccountId::from_ss58check("5FCrpDtqm6Co719Lj53Bh6Lh5uFBfTSQnQmYbSuKBozseGrT").unwrap(), // AWARDS FOR CREATORS FUND
            80000000 * DEIP
        ),
        /* RESERVE FUND AMOUNT =================================================================== */
        (
            AccountId::from_ss58check("5CP7ESrfSG2gYGdfbLPex6nFj5Eotz3gzaW69F1cCAXLrqAH").unwrap(), // RESERVE FUND AMOUNT
            314000000 * DEIP
        ),
        /* TEAM OPTIONS FUND ===================================================================== */
        (
            AccountId::from_ss58check("5EL8Dn9RZowRUPDGizdfZYNvPCouFxmBtr9c4wQX7EPBSjBN").unwrap(), // TEAM OPTIONS FUND
            46033333 * DEIP
        ),
        /* TEAM RESERVE FUND ==================================================================== */
        (
            AccountId::from_ss58check("5HTqyqwz2JfSCh4EzdEDXorpL6PRHtzJscxyMQ8kejEHFySs").unwrap(), // TEAM RESERVE FUND
            17716667 * DEIP
        ),
        /* BOCA CHICA IDO ======================================================================= */
        (
            AccountId::from_ss58check("5Hp8ebMEaj3rHroZggGM5Cmrw4H1diCN2Kb5BdeL4gw39hXg").unwrap(), // BOCA CHICA IDO
            8000000 * DEIP
        ),
        /* OCTOPUS FOUNDATION NODE 1 ============================================================ */
        (
            AccountId::from_ss58check("5CJrDFTVTYVgU66qb33CqBnk5nUYU7Uo81idUyUyRPnq1qCH").unwrap(), // OCTOPUS FOUNDATION NODE 1
            510 * DEIP
        ),
        /* OCTOPUS FOUNDATION NODE 2 ============================================================ */
        (
            AccountId::from_ss58check("5DvQX4kFnch428Bb5oZnGt2duYbo7gFtNs7SHn6AgGDYdWKH").unwrap(), // OCTOPUS FOUNDATION NODE 2
            10 * DEIP
        ),
        /* OCTOPUS FOUNDATION NODE 3 ============================================================ */
        (
            AccountId::from_ss58check("5DyzstWafuTaSUHBM8Qpxr8GofkbUFe9MLdHZs96PfzbaZQM").unwrap(), // OCTOPUS FOUNDATION NODE 3
            10 * DEIP
        ),
        /* OCTOPUS FOUNDATION NODE 4 ============================================================ */
        (
            AccountId::from_ss58check("5HbS3KQ8PdR3uQbYrXzwF6wgDL3XSiHdfoKa1GtrdQRpKGeW").unwrap(), // OCTOPUS FOUNDATION NODE 4
            10 * DEIP
        ),
        /* SUDO ================================================================================= */
        (
            AccountId::from_ss58check("5F9XVEoQDCYmTH4k5qczas4DiZUfp1RGbYKHyBYFgA9Zj1qn").unwrap(), // SUDO BALANCE
            11967 * DEIP
        ),
    ]
}


pub fn get_vesting_plans() -> Vec<(AccountId, u64, u64, u64, u64, u128, u128, bool)> {
    vec![
        
        /* TEAM ================================================================================== */
        (
            AccountId::from_ss58check("5Db6cnUaq5h9CLUeYxcbfqGhVaV5hFaXf71WFHRUwKcQUBdn").unwrap(), // ALEX SHKOR
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            150725000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HoyzrbEtJzoLa4SuPrvTBcDphKFuKhdKey1E15f4FnyCqi7").unwrap(), // ALEXEY KULIK
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            127100000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HC9am1sP8zjC69Pjcx1bqJHbwBPPtRRZ77tuN7V9HWGHF4e").unwrap(), // YAHOR TSARYK
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            96800000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DZaWv9HQZCRiASBcB6363nj6xYYwTpKCHV96NTbxt1HNZrU").unwrap(), // DIMITRI SIDOROVITCH
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            75375000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Hmt1AxMWF91dAKVcNWhMfiSyLQ7Ki1hWPbW8SspfMtMY8aE").unwrap(), // NIKOLAY SYUSKO
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            50000000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HpPDX8PjNgY8iTxpsk4L5N1pqTyVd8sHvdE8yZs8T4x5vyb").unwrap(), // ADAM YATES
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            1000000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EPLFFo6WbZeGDEASPJGEbttX9mvUtLmeDAPk5HL7tC62Rn1").unwrap(), // YURI BOKACH
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            3000000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CXQhTzBXrbwrNWNM4WVFhd6jaV8Z8VvgrykdzsND1PRiWjP").unwrap(), // DANA MALAYEVA
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            1250000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FFTeqMRXkwYP7mqMyyCgaznzGp1pUF2r7ts6vqZXk7Eywm9").unwrap(), // EUHENY BONDAROVICH
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            2500000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5C8awaFMLtcByzqKb9Vow7HjogFCNeD8V7MyFd1iFt9byNyZ").unwrap(), // VADIM SIAMRO
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            250000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CJvjfoaFbU7H6bg3YVeVQUPzpvu1DL3XvxBUd4bdyXW3poW").unwrap(), // ANN REVYAKOVA
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            250000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Dc7H96uJfqobyq6jmcFNbhHGyMRy6zV24qR4hd7js7nqnP8").unwrap(), // ANASTSIA LOPAREVA
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            250000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HBFw8FGnWGR2xaRVcF3HeyxzA32kyJFnmcrhJ4PaxnvjN1k").unwrap(), // KIRILL DONCHENKO
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            750000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5E81jQhukbEQejzMiFjieCcU9LQJnNCYM3wKqkzAeKtpSuej").unwrap(), // JULIA SHINKEVICH
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            3000000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GGnoW31ByXs2JsPNXnox3v3D1ohuQyNFth4PX9cVHPnQJvr").unwrap(), // YAHOR VIARBITSKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            500000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CY2FdXfoa6d488avA6Pse4q7LLG3Ve3cSgWNziYJHgywBxZ").unwrap(), // DMITRY LYMAREV
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            250000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5C7qQK6vVcGDdddhtcrgaefLoCr4eGxsXn2AhFc8KPzWg4xc").unwrap(), // DARYA MARKEVICH
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            250000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CcQeujdaVf3q4Hisc1WixeTVVJePucg1yGeiytZJtosMJ7v").unwrap(), // YAROSLAV MITROFANOF
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            3000000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EjiFe2G9m6GX8dqMWwG2ag7pcga5aUYfUUp1dJ6JJRTxUwo").unwrap(), // YAHOR ANIKEI
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            1250000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HC9VNrbXoATKmQMSgGmCu1dQ6wSENMh1bzA6Lbpmv26hhCi").unwrap(), // JULIA NEKHAI
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            750000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5G6rNhewyF4AuzFfmo71VMsYnFqwWqneUFqxvDsgiot8k4yj").unwrap(), // ALEXANDRA LUZAN & ALEXANDRA SERYKH
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            2000000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Gx37xJrXEG6U9agztqkEeCP814v1kzF1AfWjoiaiw6Pucpk").unwrap(), // SHISHKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            1250000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GEsc8PsQpxW4k4anHShJnu1DMJE7faj3rpf7ubNRYa9VFom").unwrap(), // SVYATOSLAV IGNATSEVICH
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            750000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FC9GuwzwhNAvMGHXS4i5JLFJGBVTZAWskCSgtfP3geXoM9s").unwrap(), // PETER FARBEY
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            750000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GjQJFSBxkLhGB4mUhPCVFrqw2XxEVfqRVTKHRPu91JNHvk6").unwrap(), // VITALIY SHALAK
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            1500000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HWTkfyQ5aREGYjAomdHn41yfnfjzvZ3mnq8HwaTATtE3drM").unwrap(), // STAS DASHCHINSKIY
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            1000000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EeJUD76GrG1H9gDWYqt54XP3r7yfSXiXUeLSUfhM6FeKJh9").unwrap(), // YULIYA CHIKILINOVA
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            250000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FjJC6w1tFb8dqTB3Ke9uWZa8Kdv3nEEacCbxQxBZm7V6bEX").unwrap(), // ALENA KRASINSKIENE
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            250000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5H8e3DPYYUg3eij6xLeePYxwxtbztCVkcDemWSHiLKNYbTPj").unwrap(), // TATYANA BARDASHEVICH
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            1500000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5H1YZ7jhJK4CgzPoCmMfp5SYFDa74GeVgN8q5uEPxHmB2WBn").unwrap(), // MIROSLAV MILOVANOVIC
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            500000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CK7Nzhf4a1HWqjAuFiyoq2RGzTtrH75mPyeBuf8m9bnsRMd").unwrap(), // ANNA YADLOVSKAYA
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            1500000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HbeSwjQs1xWbQ6M35VdoMbK1NPAwyS8thk2cTNr2khzeFqj").unwrap(), // NIKITA AKSYUTIN
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            1000000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Cm571HB9WQ1MA4cVmwGDZvZARdjyLAZTgLs7vJmXgxUP4y5").unwrap(), // NICK HAVRYLIAK
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            2000000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5G3eg2UqYcA141fbed1V1XZfuSxktY4v3da93i31JjvHHtim").unwrap(), // ANASTASIYA BOGOMOLOVA
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            750000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CiRKcwip5QVLT6Ec9HCXfgJbyMd24YyGWqyCZLyRrqGM8EU").unwrap(), // JULIETTA VOSKANYAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            250000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DXGDisfj9jgp62wL6VjDs4RTmUVzsBJeGeva1UHk5pcrQk5").unwrap(), // ALEXANDR SHMIDT 
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            250000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5H73xJdkPepNHyrRDSKbw8AtauZn3wt9nCP8k3i1xq1GoNfH").unwrap(), // ELENA VLASOVA
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            500000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5F1RoDYEgAsCALL9ZXX9B5ctXTSi2Fm4ryzdFEUQnjvATh9Q").unwrap(), // PAVEL LAKUSHIN
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            126230400000, // 48 months
            7889400000, // 3 months
            0 * DEIP,
            2000000 * DEIP,
            true,
        ),
        /* ANGELS AND ADVISORS =================================================================== */
        (
            AccountId::from_ss58check("5FUBk8tjkXUauTwKDvhjj3ujBGNswQH5evY3WkCFd1TxrUct").unwrap(), // IGOR CHEBATARENOK
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            94694400000, // 36 months
            7889400000, // 3 months
            0 * DEIP,
            18750000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GRdTZyJYjUM53ZC1FkrzTaNqSmw1N8WShTxDdKaWCbUe4su").unwrap(), // DAVID BOWILL
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            94694400000, // 36 months
            7889400000, // 3 months
            0 * DEIP,
            12500000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5F2BhPMUZjU39PCGr4CRrgXTp2bxenvXYdUMNeHTrx5EWEfL").unwrap(), // LEONID LOZNER
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            94694400000, // 36 months
            7889400000, // 3 months
            0 * DEIP,
            34625000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Ebi4kE8kBwuuYvJH9GQmfmbF6v2trviVQS86Hv4PEpoDiML").unwrap(), // SERGE DZERANOV
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            94694400000, // 36 months
            7889400000, // 3 months
            0 * DEIP,
            7500000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5H13CpAwvMUTeNcTxfGmi6YdoSDo6kAHJfde3H4YoGyq5WeL").unwrap(), // ELLIOTT TEISSONNIERE
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            94694400000, // 36 months
            7889400000, // 3 months
            0 * DEIP,
            16000000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Gq4YB8A4keRLFu84cKQNizWo93RLCyt3TZqENMeFxxXpmTJ").unwrap(), // PARALECT
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            94694400000, // 36 months
            7889400000, // 3 months
            0 * DEIP,
            20000000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GHNyHmqtBUoDDxrZiYn5btVx2axrt7Yxaf9LxkjWJiXm2Ev").unwrap(), // GOTBIT
            1651276800000, // 2022-04-30T00:00:00.000Z
            15811200000, // 6 months
            94694400000, // 36 months
            7889400000, // 3 months
            0 * DEIP,
            2000000 * DEIP,
            true,
        ),
        /* PRE SALE 1 ============================================================================ */
        (
            AccountId::from_ss58check("5GEsCZbCvPYWpTkHrJTRYZFQZSTrBSat8bPLWA21prM1Kkfm").unwrap(), // ERINC ATILLA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DHZfJSmAA88jg7o882GNn7PdcQ97BxEQzz5orDY9uvqFGKF").unwrap(), // MUHAMMAD SHAFQUAT HOS_AIN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DDSbcYtRQjDtrFRkrnbGv5apMw8jCweArAjRtvdnNi9jGnY").unwrap(), // BUSRA KABAOGLU
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Cfqza9NQX4hjXqoYeo5HtWGSF6mQ9U43X9iCP1GEPCuPzwX").unwrap(), // MIKITA HRYVITSKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5H6AZtYUqLcYX7YHD5Wq2CPEvGktXS8ZNh8Z9pHJPD8CAAKJ").unwrap(), // KIRT SCHELLHAAS
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DjwpJNWAfmKByGaKZRXAWQ6Lu6TrDEmeVvMSrbyhTb91CAr").unwrap(), // RYAN JACOB BEECHER
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FTPxZcTuz5t96kD1jgfwDoP8Ya7XTbve6CMWncfPotn1fHR").unwrap(), // SYAIFUL ULUM
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5D1zyTaQmMPR9bL8zXN9Ws25ESStgHKaSwPT8Yc8hsJ12qLN").unwrap(), // WILLIAM ERIC VAMANADEVA JACKSON
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CCsqkL72QRb1qEpuLiATq4cQeKaRmMyp69Su2tuXukrvGyh").unwrap(), // HARIS HONDZO
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5C5KUz3uYhn8kz7toUg5D3R9P9CkFahdV6Cs2GXBCXLzBPHy").unwrap(), // YAOTSE A EDAH
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FW2Udq1VmExRw4MhRiEA3uUhCTxv6Q8ZZZbvGXYqSQRWHJk").unwrap(), // PATRICK MUNYANEZA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5E1ko47DKbfkn7wKsRpBWJ4qUkUZ4SbmnN7PaoKyBKM4RnU2").unwrap(), // BAPTISTE RAPHAEL LOUIS DUMORTIER
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CcPbph6M1sALoGj5NmH9u2SQa9JeGr9anzECv9qT8Dc12Ak").unwrap(), // ALEXANDRE BERNARD DONY
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FCebSFxH7Be21xEp2FTchMTekHBhZfpTVsdGLiqAEeMEq8Y").unwrap(), // Y�CEL T�RK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5ChZRhziKx8iHWHqPWixbxSiyvEDieeMMSCXZzuszTVht96N").unwrap(), // XAVIER MAGRI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DfZS7hwHRr1Dv5Moyf7KurJFKcnq9GqBUtHP2gsnEnbJkiN").unwrap(), // VINCENT ZUFFEREY
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5He6kffATaaeZNhUYefX461zP2PGoAMUB9me6AWeHwsEhWyY").unwrap(), // RADOSLAW ANTONI MOTEK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EvBdDfJP1ZM2FrrVMkoNsN82GdsHGEHVwZKrtPPreBWF5Lb").unwrap(), // DANIEL CHAVARRI ANDREU
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DRjne1xdsGCNK2N4EVTBiaBKH9PDDmX8EP19bVscYohPwzY").unwrap(), // NURIDDIN JAMOLIDDINOVICH GAIBOV
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DqaWGntH95mrKDrf2r4CWFshg2hS6MP1icPCmRuDAmi9QXL").unwrap(), // AHMED ALI M IDRIS
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FHwVWT9eJGknuwYEk7VDcn8N7oWTKBSPvrraQhXdWnC5amF").unwrap(), // SEBASTIAN KULCZEWSKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EXE67p4eLmZ75esAuyhPExBMEJKT6ZLRHnbUzGxxecZsxKQ").unwrap(), // FILIP PIOTR KOSZUTA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DNq57BKzighP1ihFfBjPhMZPF8S71HoSWuPtdoVQUenUbwg").unwrap(), // DOMINIK WINCENTY FR_CZEK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Gpye9H772hyRYoRVywV9eHJDpDhCJgwyaxJaBGL8PJugu4P").unwrap(), // DAMIAN KUCHARCZYK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FudVGoyhwqBYMQpuZK7Jgg19n9KhjLZhhMesHcKyzX8hWVn").unwrap(), // ROMAIN AUR�LIEN L�ONCE MILLON
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Ccvzpdvh3Wdh4Btxa9L95ADfWz3W2MZnRDPE6Qcg9nTnu4q").unwrap(), // ALEKSANDR ALEKSANDROVICH SHUTILKOV
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FcCPCz2tByawPgbB3irkwXKxCrEyfR9eb91TZMd8gPHfcfB").unwrap(), // MAGAR KHEM B THAPA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CPt2UtBquYxwa7HaU5eHvQa1wSZLte7V3psthHqiNeddvsQ").unwrap(), // HAKAN DEVIREN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DJXK4UP7suN2nQpU6fHNrsgygnPjxSg35EtDnyqRkFEHfSr").unwrap(), // NICOLAS R�MI ROGER LASALLE
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DhpnzKCG1A3UHXtw4qFJn3TNMLZdrLZ558XGmiYLnr9uMD4").unwrap(), // MARGAUX ROSE MARIE MARIE MADELE BORGEY
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5F9VhfFoGCQaGoKmPF8wKxowVWBPTsZHPySKfWoNH62yDd2B").unwrap(), // DMITRII NIKOLAEVICH LEDENEV
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HMeurL1piUBN5CjX88yYckcLtBSuWDZGjh5bzqSeo8u72nS").unwrap(), // DERYA VURALGIL
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Gk3UwL8hiR3zk4CHXh9WTKArr85UaNEypjo9epCjWQXnmeH").unwrap(), // JEMIL BIHA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Hivc2wnjFbprGWX38Cr33dC4kTb4M6g9j9axsnW23DEgntw").unwrap(), // ASHLEY LUKE ROZARIO
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DPCjbRXWPK4127AquKm2DEHMGeXVT9TqqCiVSGBieZuuCCp").unwrap(), // WALID SAYADA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HbH9dZT9nsBaEqwwNTnW5BjfNHMwY3GgGJzN5qK94NbtkzE").unwrap(), // SZYMON JOBKIEWICZ
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5G4UL8LP5bfp9VSXrWYD2JPta1tcNGpRmD2LkTPJ8Q33rrAU").unwrap(), // JALAL
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DLg7qwTjqaTRQ59EqZV6hKVVaMBqsxk8NH4Z6ev8p7DtTcF").unwrap(), // NILS CLAUDE JEAN FRANCOIS POUET BOCARD
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CFTFgDxcoCf8eefpYwuJhtswt688quPZzAhb71EzopfF8df").unwrap(), // RESTY CHORYANDA PUTRI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FH3k5DBadPwS8XZmRPBPQXTbAxRAAHxUcRAZmM2KoHvd4L1").unwrap(), // _BRAHIM KORAY _ENG�L
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GKQ1dsf1KUm8QvPqFvwsHZX3LYCsbTEenVJDeZg92kNqMkX").unwrap(), // RAPHA�L MATHIAS PIERRE GAVA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5G1hfJDpNpTAWV8j467qKr54yJnRBXST581k7gsaHCh3iFDP").unwrap(), // ALBERT ILYASOVICH KARIMOV
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GeRzXJwcjyEe8uPPzBWmDaHiMadcwjPHJi6rPmkVR8yhMtT").unwrap(), // FARID OLA AKIN IBRAHIM
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GEQ2c9CmuamDxxgmiNwfQAD6h2VnKTGtNPA4qv2J29qRsKM").unwrap(), // DUYGU BURAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5ELiJmmsV6BcN9MUgzBL2GZQYqTnkkZ8Xa4zFCznFivHdHHJ").unwrap(), // IVAN ANATOLYEVICH LAPTEV
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Dsd5VVNfY5jNnZke9FCed87LYAnsCXRC8fiZ34x8guyUVv2").unwrap(), // AHMET CAGINE
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DkBf5CqqYYofPjR3578hsJNqRNKzNVZs2YSjXaUp3wbChRp").unwrap(), // _UKASZ KAROL ROGOZI_SKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HT2rjX6rYAwpH6vEQx8hNGkYft5b8pE1qKkAQG49mBmD5fS").unwrap(), // PIOTR JAN PI_TKA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5G4N1E1CVPnfSfTTLwJomHQguVkaLygkv8omuXzuRREuym6n").unwrap(), // BAHATTIN AYHAN YILMAZ
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5G1dtLuJgFnvGJrUCe3izYUGvjtEvSUe3fsMuysqa2C8S2tq").unwrap(), // PATRYK SIEK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5D5P4uCAoeTLGvipLzF4LGXT5qabtNdwfL2v1rp5SjrjcCeU").unwrap(), // KRYSTIAN KARLI_SKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Gv91dEkixMuZrFY8urnd7mbfjHHza1SzJ8NGeLTbh4j5wCJ").unwrap(), // EMIN EDIZ OEZ
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EUo17DKM6h2H8JoWoAs4kRqsb9JH2X1PHFPrU6GNUMMsBVu").unwrap(), // WOJCIECH DEPUT
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FgxZH2LuQcjiCWU74XveLnRnxABfU52797fsVNvUTQacKjW").unwrap(), // YUSUF POLAT
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GjbPbgZTKyLvqp8ZdDBpsveMY5nU7ngMgWZf1TdhEy4Uhp1").unwrap(), // ARKADIUSZ KRZYSZTOF KSI__KI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FFVfmN8hiKbaEpu3mzyu6TskfRQvYte2nersKbPB4HWS1se").unwrap(), // OLEKSANDR ZOLOTAROV
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CkxPn5wfJtqWLFsTzmTahUD4ygeaJMtpUiRhKadjN9FmPVa").unwrap(), // FRANCESCO BERARDI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GZQPjQjnwwhnRAGjeTj2hwP3RQEoGrpmtTyshwyopSZTLfb").unwrap(), // TARIK BUGRA UYSAL
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FHtQXTUzkpXYVDrNUX6Sbm3Fy57HU4LDLz8sBjgQwm5hbDv").unwrap(), // RUESTEM YI_IT
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DkdcpXFZG97hyTjdr8mbq8DNJbPLk1VVyTR9h1jYQVx12iy").unwrap(), // TR_N-QU_C-VI_T
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5D51Eu39GMApmGFaHGXLA6a8MFXkrTegaELy38rtgdUEq5XL").unwrap(), // ILDEFONS FERRANDIS CASTILLEJO
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DFSP7D28gCk9m6wjz8q9vwEk8mXsx2E1UNcPqyYQbv8pFhK").unwrap(), // IHOR VOLOBUIEV
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5G7Lvu2qoesT5gskfS548reXXRtAmWnUqCAGBAZQNWUHLfV5").unwrap(), // LANCE ATKINS ANDERSON
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HE6pvU24j1VWFgvsE5xHUqCkN4JrmCM4bsM36v9gvDBBnwN").unwrap(), // MERT �AKIR
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DFjQAs5vwt1dEazTbar3k5KG2TijBKfBP7uynvmL5ZJT6sB").unwrap(), // RHOMAD ANTIONIO JORDAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FbHrKHPUwMrgNxZnMEPBxf1N4vPduWr6CjjGUEgR5g8HryZ").unwrap(), // LANCE A ANDERSON JR.
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5G6JLkKd1icetzbzoKC1CLd8GJVbwRyo93J7LEZNnhJCV2JP").unwrap(), // IGOR VALER'EVICH ZAETS
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FA82P8eQJMLuYPhDkiactaNtyiC4BYEuHnwA5LEY4NeHDhe").unwrap(), // PAWE_ ADRIAN MA_ECKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EcEmmuFibsEZfY238F2wgyTz32X1bV6HHvxzhL3jSiziFXp").unwrap(), // BRANDI LEE PORCHE
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DoAjJ4Wpf9nmeFgGo5fMm9kNQha436vJgRAmjBhignvndEY").unwrap(), // FADI GASMI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CGGWwgB42qJPhJD5pCwKuKwVXvnyfEpLRAaWreQRGmSjihr").unwrap(), // LUKASZ TADEUSZ JAKIEL
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FUSXEc2x5uj7X45TTnhAZ3PDprciNLiyToHiW2PZKrkMNrd").unwrap(), // SLAWOMIR MARIUSZ LESZCZY_SKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CB1R4sBRgAPU2fEt9R1JyqfcKB7dee4MUUXYLVbHF8uUJ9X").unwrap(), // PRIYA RAVIKUMAR
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EqrWxrCq2pj6gqzLkPeRUUuJkJDeyVNkXSKgnfnAqurCrBS").unwrap(), // ADAM WOJCIECH PI_TKA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5G1D7FjJoCJBZ1mYxUAbPz8yMCUa3nrcC9FPXkoqZSNat7gi").unwrap(), // MARCIN RAFA_ BIENIASZEWSKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CDWpi55MD1b8ZWAu1AMnqaoVzKT3ATJiZXsPnyaroqys6FB").unwrap(), // DAVID ADOLPHE RIMBAUT
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Gy7BKPaZdXCoNPPTbAaZ7hGxRhvWxyFfo37RXPHTekCeLS6").unwrap(), // CIHAT CELIK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CFk6txbsc9ENtpuqaCcybkWUxuV9pg1qyrKkiZ2nw3nz2na").unwrap(), // MOHD SYAZWAN BIN JUMAAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Gq934hmtcwJDXbp83aKkRYMGvQt8FRtsAuzZaph3rfYAhrw").unwrap(), // ALBERT OKO_SKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5F1vYv1fzakeKw3y3F1BT24zVWqYyngG8qqh8gXjFcBvDhm1").unwrap(), // JAKUB BRUNON SOBIESZEK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5E7a16dmk6yxUa32yefyJXVv6kWfee3FJeSxEDE84EZeJQVB").unwrap(), // TAYGUN YAVUZ
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FR1dQJKMEyVWiwJLnq1N3a9cfWcj8C9jRnUfykKtKNHJZXp").unwrap(), // GURKAN YILMAZ
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CfsErYV7jxshMysgF6A8sFQQ5xTkW81VTx96zH8F1amanuq").unwrap(), // BARIS ALTUN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GZSa6ckrP1epyP1arv24Zf1fA3b9T7tMUkj675xcKAEuAVY").unwrap(), // ERTUGRUL SEZER
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GKe9KugAmMcVUos68JG7Wcj5fhsH65BBLsLhPkdXFGtemSi").unwrap(), // HAKKI KORAY SULUNBE
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CK59QH7W3RGtrgpQmcSwfEfnhhrmWZSwoFow2RkGDM1Up8N").unwrap(), // BAYRAM ALPER YAPICI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5E4WSQxJctZRuss27XP4s8veCVCtMzN7qFBjQtLPZsGfDBqA").unwrap(), // MEHMET EMRE DEMIRYILMAZ
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EA8HZ8eCN8Gu3K3KGP14ZtJpbvx85ZzoRmCVByF9xU9zR8m").unwrap(), // BRUNO MICHAEL SYLVAIN HULIN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FPH7x7DPQiyvjAXRABvciPeYMVCTrAw2gKykXzF6dqZGSmQ").unwrap(), // ANTON SEMENOV
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GCVKWA4iNsJp5eAbe22KthcsQ3QfsjMeQuyM6sUCWppbN9C").unwrap(), // SOF'YA ANDREYEVNA DORONINA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5G9r8ybBJysCt7XzwyH4StG2hADnZurcapsCqCVciiHBJ6E7").unwrap(), // OLEKSII GOLUB
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CDWtH3VKa73pcPicwYwcUeCyyuPZosyEiaPS1n9gpxApDSd").unwrap(), // PAUL ANTHONY ROGER FERNAND BODIN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DyHyJqtTVmPAN9mNkzfeVaQT1pDnYTZVMFRJ2s5o7cwbzZe").unwrap(), // WUSHOUER XIAOKELATI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EHijRE1oU3kmdAhudEf5fzf4zhkdNVnwfGWiARyBcY4ZsyK").unwrap(), // MARCIN MARIUSZ LESNA_
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5F4UrfjG1hho9fWUnQWCH8GxUyYXoBJn42nv6A5yUnZA6647").unwrap(), // MARCIN CYBICHOWSKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DPbmEstsfnN1gYDb8ThwQqAZWhgFrYqAustj7tMJ9C47H2L").unwrap(), // EVGENII DENISOV
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FPGhKucgRbLS6nV4bnP85sa56AUfcYi9LaD4W3bvVNpd6Ts").unwrap(), // JEAZRIEL DAVIS
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FZVgDaL98RKtpqQcfSfZoBjbA3tiaUQoqDfQvNYtFySNeJH").unwrap(), // MICHEL WU
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Ebrru89ZaCuzfxR6hU1hXoTxd5CPYWqdCUzVgqbEen1WxPP").unwrap(), // ROBERT _LIWI_SKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DtZKzQiQjcKX7YYyuj4jvBrL1oQBQNfFVYw44Y4JENTJQPX").unwrap(), // FLORIAN LOUIS ALEXANDRE DUMAS
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FxZRZivo1a6xnRvsLt1BX1ZY7WKuHbMPhRS9nMmuJSYn9L5").unwrap(), // VLADIMIR ALEKSANDROVICH TERENTYAEV
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5E48pbGgtccLYDG3qdCLViBwCpjdnGrNgkxwsrX5vxtyxhst").unwrap(), // ANDRZEJ PY_
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Higt9RGuMpLmPGPdqbN3WPX13ktep2zjnTSUQWME5NyYXwL").unwrap(), // TOMASZ SOBARNIA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Ef4y5z69kqyCsBFDLLfum66AxHqPau3eqEqXYmu1ZQvg7Ea").unwrap(), // MAKSYMILIAN ROBERT JARO_
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5G7PDu2dJds796o6YYP7iaHdb4SSo4TFuyBDdAnhKUdqPWMv").unwrap(), // SAMET CAN SOYLEMEZ
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5ENfyfBnuRedkoC5y2RbN1PPt6DT36dgzkq3doCHMTTXyNAa").unwrap(), // KRZYSZTOF ANDRZEJ WI_NIEWSKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FWHW4JVyocygybNTK4zFW3tqaeM665M4YzZLXx6xzMcDvsA").unwrap(), // MARIUSZ MACIEJ KASPRZYK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5ELSTVVms4AbBLmcKhPzsr7W8seFz8syVtxx5TuHLPNcNYkA").unwrap(), // LESZEK PIECUCH
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GNTpYPo48vw2J3vNdg5X8d2Z8ru4K15DpnYLhWkosUgHFvb").unwrap(), // KAROL RADOS_AW WAWSZCZAK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CidYAncvMYAuTWVnQvUqun9nqyRBJBKx17AkMxbxWJJ8Pei").unwrap(), // PAVEL SERGEEVI3 KOROLEV
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FHYAHqqKALf77VD3u1Qcf5sggZhg1fLpH7iUUi9h5UUxPyv").unwrap(), // MEHMET ATILGAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CifMwqxP9VbkisrE4ydLZQhY1n1dBfb8eznczpi8CsmL7pe").unwrap(), // GRZEGORZ ADAM KACZKOWSKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FpobETv8QZKjLJKKqCbRsUcAdXym65KESsopV1VeQBdqcog").unwrap(), // THEOPHILE JOUANNO
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5E4vByiKUxgkLH52psitfAhZQxgDdZM9BC88HZ65rz8wZrZJ").unwrap(), // ROBERT JAROSLAW BLACHEWICZ
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Caps6mVq2VXhRCb16Jzn3QmhVehZ5JbHUWWGrjUa7r8SPdk").unwrap(), // MEHMET AKG�N
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Ct2tgkW1dp43bz3y5NTueMwcEqs5a7hVLMtVvxX5nBExdwJ").unwrap(), // OZLEM ATILGAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GioisoF8wA1Gnjf1LBTVQaZ5JpJ8bvGw75M9a1jRrFK5DSX").unwrap(), // HENRIK HOLDMANN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GQxj5yHQ1qkKdMshJ423T8KJujrFMXBC1YTb6vD5cZMHsnA").unwrap(), // WILLIAM RICHARD WEBSTER
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DSCYHHMxKtCKGcHAPZjN8TGrdiz9xrh1L66psMkki7NAogv").unwrap(), // EVGENIY DENISOV
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CrYhYDYYtNeQZyP5at2v8vL2dJ4X9R3kSdT1LzoBnHxjxZT").unwrap(), // HO�NG V_N D_NG
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GNsNVuvW8oExaVKHv9qSTYEAcqfAZukXETeJc7TqkrsU5ZK").unwrap(), // ROMAIN OLIVIER GERARD GAGU
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GMsifhQ67jGTiGwtvL74jEEsTiY8VzouNKzHaVXftpjZX2m").unwrap(), // WALDEMAR PIOTR GRACZYK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EbdGHSdfbdJJsxohWDk9bGxKbsttp4fr2dofevfZu9W4x6K").unwrap(), // MEHMET AKIN AYHAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GTrVcFpJTuWeyLh8H2eQCW9ogPSavUSsSMusMiZFPPz9y6w").unwrap(), // MUZAFFER CAN YIGIT
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5ComRrjZcPnBuRbNJPB1A3LVSYoB6bqeW8XjzVZpWgdZMTA8").unwrap(), // MARTYNA WLAZ_O
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5H3tTfTJpKqNTJhiXjZKtwAa17EYjeTW6h2whzvocDeALw4J").unwrap(), // �MIT ALDEM_R
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EYwPNwfXaf4juWw2Nre8iQNNDeYQEhBuvvC5sFcHViDbuKv").unwrap(), // ANDRZEJ SZABLEWSKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5F72r6GzvddtYg9NEhUjY7Q1vnz71UUfHUVBHnQXZPd2MspH").unwrap(), // MARIUSZ KONRAD KLATKA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CVC1azsa5wwTyWqjdy4f5YEX45NhYW7dkUYBqciQKbkdevp").unwrap(), // TERIIVAEA STEVE FAETA PIERRE HANDERSON
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EwBSkbtu4MatmxPQPkXtf2YQYVmSZ6UAHTJ5WztMTt88U68").unwrap(), // QUENTIN CL�MENT ALAIN PIOGER
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GnDDL8Wx8GfbixRobMFwbkkmAc9bHkyoKGYrzak7vtd8vX2").unwrap(), // JACQUES CLAUDE ROBERT WENDLING
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DPuMNh2rnf6Lu6wkCpuu938R6wSYz7u3NXAHjih9uABfuKv").unwrap(), // AMEER HAMZA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GKbdZyYYXE1afJEEFhJdGjRxXJgxyehhzKEKhufAT4jj4Pz").unwrap(), // ALIX ANGEL OLIVIER SARTORE
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Fh566w4yt87jtu5RQc1Ndc4uhJsacRUWRC3JkmFGSv8CNCr").unwrap(), // ANTOINE PAUL MARIE CHARVERIAT
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HotcMk6sLcgSBhvT9Xis1hHLsBgHBodPfD9UqYB55Bm2kHs").unwrap(), // BERTRAND MICHEL PAUL LEMAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GbA4M1wsKNvuxhVH6gEwSb9X4t37XzCC1wYSypcAMu8eRFN").unwrap(), // HALIL DASKESEN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DPDK1kss1NroqNB5FMcBGyULQvstwPz94q1eeubh2vMwTL5").unwrap(), // FRANCOIS NICOLAS VANHOUTTE
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CA2f2F9wyXjvTbaypa2QeLpYKGhghid9JTafbPLJK1ozPxV").unwrap(), // RENAUD GALAAD PERRIER
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GRE9ZjqMj7RZnJVKrrTt7em3eRtMtZMbFHWPmQFazvnk5rj").unwrap(), // MICHAIL AL BERJAOUI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GgXXgLCmAdyWMehHgTNj2AHtuQhFMfRFSGE24RHDcqgHzYG").unwrap(), // SUEKRUE YILDIRIM
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DLaaySmgFw7B9dp7174Y9bvf76Qe9kRP1uf6tfpLm1WgbX2").unwrap(), // YAROSLAV ALEKSANDROVICH GRIDNEV
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5H64HE5CoyDmtpt658RsrREJL3ovBonpb4JR4mierfymiBMc").unwrap(), // TARKAN OZDEMIR
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GULF6rJaibL2xqN87xLnPMtPPFNVdUZaMNx58uWoFQ8r6br").unwrap(), // KURSED KHAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HY3zft4dQSgGhtwsubPbwSDyXgBeNx6b1fM28QfpVhB2i9D").unwrap(), // YAVUZ AVC�
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EJC4N2mCm6t8rM5Qvhi6xVvR59bMf6iaauwhEYEnJu5R1Tp").unwrap(), // MICHAEL FRIEDBERG
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GWqnB7oVXVwFL7Xu2cXVkXhfY4LhpTQTDbYZTY19Mn2PKQE").unwrap(), // JEAN-HUBERT JACOUES GEORG�S GENEVAY
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FWwovjmjyRtmCFPrQJx5pGGNNanFUCrz5uKPcAGYPjD6ZSV").unwrap(), // GAETAN ANDRE ALAIN CHAULOUX
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HmJzti1FXbfurC8ASLtJQNhhJhyWH7ibjgmJPW2YxhSX43S").unwrap(), // VO DANG HUY MAI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HRJdRcge8SDny2Zod54P8abtcvCzXDyijhVNkvp24S5AXo1").unwrap(), // ILHAN GOERGEN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HdWmM3E6B6mhqy8JGE2kdQ3LkE9UX5e31sP8mAkTg5cQme8").unwrap(), // DAVIDE STIMOLI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HTGBD87z9iwkZ4dcySRHNuty9QMXsWbS5VdVUkEZM9ofVEE").unwrap(), // BENJAMIN JEAN CLAUDE ROCER PICARD
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5whQV6WUtavNKNgHDeCbbRoxoHMTkHQbqCAWpAEoJeEYW4PU").unwrap(), // JEREMY ASTA VOLA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5ECk1sb1cHy8AjYrEG2sR76e5YiFUiBKy9qnLgVZdufNr8cX").unwrap(), // PAWE_ ARTUR JANOSIK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DAtnPuWS9sagNeNjFxExS4tzKdMx4g2Aa1arZJs6w8jwptu").unwrap(), // AHMED DJELLALI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5E7NVxP8jQcGvZh3DsvYs1atCaGDrjBJUsUz6rN4Gzu4Z5XT").unwrap(), // ADRIAN LOIC ALEXANDRE CALTAGIRONE
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HCDAhDhdz8b5hCVZSkhrFhiwaidK965eLqG8UMCdFsZU7yd").unwrap(), // _LKER KARASU
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Ft7StW2W8JUmqs689dc9B15MwAvPhrkHVrcaqJGWbrvibu7").unwrap(), // RENAUD PATRICK JOSEPH LEVEQUE
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EecAEaFmh3GVpe2yvJbeqX9pN3noRH3zsSFSnmB3CqyZxMe").unwrap(), // HERVE GRYCZKA DANTHONY
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HSyAGdNa54q5mWGPB6itYbJ9uyEtWDxjhqo6wgrdE6Amsyc").unwrap(), // GUILLAUME GR�GORY KOCHKANIAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GYtaj4acQ4T1AKNPkc1nfi3Bb4HaJpUTLqrhTdWjpG9sM3M").unwrap(), // ANTONIN GILLES MARLIER
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Fxzg3y2S4cE87WyZn6VpHjEfcv3swDAaDMyMAD5eZi4BivP").unwrap(), // MICHA�LS SAVIOZ
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DhXackC2obWqeNfd8Gnzs6ZCpX4STMuLSyCLik9c8tn4Sec").unwrap(), // JOHN STEVE TOURET
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Hov9Dp11Ub9TiziwV9HjSmpyxTfFkhmdVPCRLz7vUYMAxNP").unwrap(), // GLORIUS MARTINUS SINAGA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5E1j5rocTTzLjw7dYVhRsgL4x7QoS8WG4k1mEfbwUun394wr").unwrap(), // ABDULBAK_ ERTA_
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            37500 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5G1bY9HW7gafVafKhVDNYrtfzYB9qTJm6Y6kJ6wj26jjMGyX").unwrap(), // RABAH HAMISSI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HKTMzbWrzfCYU3B6rUeQ4Kp3RcyjbgtXrXVA7npQkq4RHiA").unwrap(), // MOHAMMAD BASHIR KATTAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CXSGV4Ymfo1CyESzsYDQE2uY3vBwbqFToqM3vC9mQ9A9pFh").unwrap(), // DOMINIKA JANINA KUBOSZEK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FetVhu8xRYaHRxMh7n6UzWxYzjzhgihwoqotHbanJKY3gyD").unwrap(), // JOHN P SILVA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Coq6v2FRoMycVbGFXnwhrJAhdLPVTEWAC1V2g4D4G69AuXV").unwrap(), // HILARY DUGAS
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CFkDL51P3MLQV4Go66nJ24UAjZKguKgZorejsn8o7NXiZ9o").unwrap(), // SERGE JACQUES DURR
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DnkzidtPueVa3G3uhF2AaJEVZAoDE5r6FYFNYspPxwFVoCX").unwrap(), // B�I NG_C TH_Y VY
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Fsiq8JxVvgLkLLCLa6qXmRyckeVM8VaHDhaPg3BnFqGhpDX").unwrap(), // ISMAIL BIRICIK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FvEEg1UNuAWNVwYNNKfRvMxjGAP2khzgbUgLNbHjadafRcR").unwrap(), // BARTOSZ RADOS_AW KUSSOWSKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FYp4JDjAipuygfAP2iJdAzWvLL4cQkzMQmiDSH67Bg1uF39").unwrap(), // BURAK GOCMEN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FdtudHXTwd7CgTV8MLvqkXtK5xYF7Yht7d4Y6pTvPEw5NvS").unwrap(), // JULIAN MADDOCKS
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EZcJue6Rj4t4P9sPE36HMSTspgr3jvdXyt4SECh25Rim6yD").unwrap(), // IAN GRENVILLE JONES
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HnPZGebXciqUQioe1x7MMmKW9w7XMoqPAp4zH8nMe5FsR2i").unwrap(), // ESAIAH MAKANA GIDEON
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Gj5d8gqYi7y7MbDbd3RBp2JATZt4bfMbQwiZBbCCaKbz26j").unwrap(), // JAROS_AW J�ZEF BRZYSKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GbDK1asBajZ24J2z49zbvW7NT4m87sP1zjxmcv3RTNYnYup").unwrap(), // JEAN BAPTISTE DAVY
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GCUMmR9cpHD4uYfRc3nDSv4346yKHhPs75Tey7o5na2HWDT").unwrap(), // LIAM PATRICK NADEN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HbJv4X8Y8ahS5zz4DhTA3sWkB3BZoUkkMTbD5Ar1M9vV919").unwrap(), // N/A STANILAUS CHOO WEI EN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GmXNpKo5GfE9kHg7Hsx6ALsLEdHBZ9H87HswUMMDbGvUkVp").unwrap(), // SANGITIANA FARARANO RAKOTOZAFY HARISON
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CqTaFsVWNWpw28JKtmbXiEBhYcgGaQZetLvVKMdYQCXW21j").unwrap(), // DARIUSZ PIOTR MIKO_AJCZYK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5C7cZNnFEtUogFbJiAHan3NEmBm97bWeoXd2uPS7Q1Cj5Vaf").unwrap(), // IGOR OLEGOVICH IUD�N
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GeRi2bcbmTtH8CCGFggc14dW4cUCe2knzKkNNW292SN9i6J").unwrap(), // DAWID BOGON
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Gk5CoBvGYr8X5MfRtTKybvDxqHgGTbHzsW83o53SkdTSpPD").unwrap(), // N/A NG BOON HOW
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DZvatEE12h5cKSCpR74E71YR4Bt2778HBuhFKEf9zuU4hv7").unwrap(), // _UKASZ NOWAK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GGqs1p77sagQzmCkkCPorESZjcwwi7s6YhQzQwfPcpGCjkF").unwrap(), // THI KIEU NGUYEN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GzpZSQnoBEC2p2btH7JVTSBudEf9jGoKR2snTik7hmnnbXz").unwrap(), // LAURENT THOUILLEZ
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Gs2YUSyVBBKDNQ4f5c5xntMBszeDczf9FiwTFqA6raCWxPz").unwrap(), // ELENA ZORINA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CVCRPP5bDgN8Yx5MhCWRk88fi1TcLxGK3pk3H751m9e6LSv").unwrap(), // ERIC HERV� ARCZYNSKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GLVFopWE8nsFNqmaTnVG38qVz5ckujC2dLPHqRyHcLm6vUv").unwrap(), // PIOTR SZYMON FERENS
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EqUhi3gUAsVq7M6Aw87hpRoG5RqA8BURyXYvdNS2HM6d7Kr").unwrap(), // MARK JOSEPH MAYAO ORAJAY
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GEEgdk9Dc3b8higzm4sKGizBhccyeo56HDzrMLsGryPmi1E").unwrap(), // VINCENT MICHEL SYLVAIN BUISSON
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Dz4xfTyhfQSLuo3jUuenYbYjNQ654W8xoxttmvsTQAFSNCR").unwrap(), // ADILIE SEYT
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5E2iyD6QoPtbEi3WzqLkqRiwm69PGcxQukSYGJB7Ppzzs1mo").unwrap(), // PREMOMTZ BAPTISTE PASCAL BERTHELON
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GuRxnCQLgGFYEd3ntBFNJ9b5FXkb8m3Tx6ditjMN1uVxmJo").unwrap(), // DAMIAN PAWE_ SOBIK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HTRrpYbecYNze8rkj7DdVtU73Nfny3wU5zsEoBTvzUXc8K7").unwrap(), // OMER ASLANHAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FHh2WkAvHtoRSncZ3fL3aYh3ZUHVrXnS1aWQchyfGG1quuA").unwrap(), // KONSTANTIN VALER'EVICH LAPIN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FdxFD3xLvWH4N22eEnKoSXUncvrxxTzr1kY5ikvCc5ng1fc").unwrap(), // AQDAR MINEGUSMANOVI3 AHMETOV
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EAMu9fp4t1WSX8v1SCFnqH7FMxQ58cGN7HVhqVRp4VfbjFm").unwrap(), // KENTA NISHIYAMA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HMeLFNfmTw54aqJcBxbCBfAtbjrpu1qweR1XCCwVoAmTGdy").unwrap(), // MAIKEL JOHANNES AAN DE STEGGE
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DMSKMEh12ht7s2rFE1my7E2RytGtxteuNXeALHcCdQX6DNw").unwrap(), // GOKHAN ATALAY
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5C4y2LNJ56yEbvuv1ZRfpHrfVjnE6J5yEKjzd7j7YDfdGyST").unwrap(), // DANIEL CHARLES RANSOM
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DaRGevDLQud3ibceEtryCU5WCeiAGniZhuExu4pfAtnpo8C").unwrap(), // THOMAS BAUER
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GVteyLCA8xdW3HcvjJ28GVNLaEJeMKxdYprNYBMJxtwz4fu").unwrap(), // EMIN BADEM
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GESrLcrHjoPhHgh3cCBAsLVeVLFb2C28cP8w2c6MphPeXqK").unwrap(), // MIKOLAJ ANDRZEJ RYBAK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5E47tcu2YQmvkXeFn7JCNJErYLjS9hwXh1wBeUJuvtXi8PVm").unwrap(), // SANTOSH KHAREL
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5G4DQvdUkE44MihjAs4BYn8qugjTrHUN5BFne6SadHUDEwab").unwrap(), // KEVIN WONG JUN JIE
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5G1DNz4d59vXXb5KXFJWVthdiJH2KqQLnHiLbdotqcZv5wYa").unwrap(), // KARANDEEP SINGH
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FkMwUcPSWsuzFu89a7bD5BL9NzUZCfmZu5KQrx8g3vNhbjx").unwrap(), // ABDUL MALEK MALALLA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HpSm9Tr5tAdHouQdeJUVTSP3dBwnXWsPJyhC2hSThZg4D9p").unwrap(), // HOSSAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HbjQALeKB34c8AQf5skL9xkV6wnB6X293TtsRhDDzNNkP8Y").unwrap(), // MATEUSZ PIOTR KLI_
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EyqKHCHKSyJQQ7nzdKnu1JNUt1r8c9fDP4BkHKR1ucbzwJo").unwrap(), // VAN TUAN VU
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EfFRWfTWTCeMXkRADvJ7BmwBzRnyDGvpLKb9ycfajuLS9tu").unwrap(), // MAGESHWARAN RAJENDRAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GQbNUTYdqThDURZt9cAuYvgUh8ekq8VVhoGwrF34hSa8k6m").unwrap(), // DAMIEN LOUIS VAN BRAGT
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GnGrfSQ5g83mhWKboH17jUZ7WwJxeBbxkkC5eB1Ur5EyWLd").unwrap(), // N/A FONG KEAN LIM
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Hp4zcVnCxjCUotQwymZFG1prbn8KwPE1dtMaHJ4CGYK9GTN").unwrap(), // PRZEMYS_AW ANDRZEJ SZURYN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HJzvkrcAvj7PgqyoMMr1KByCXL47mfbVPx2aWn3RnHAge7f").unwrap(), // _SMAIL BARI_ SALMAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GCU7AuxsGbKAiYzfeqFSmYMqVezPQRrV8khacg7TNKU6fVc").unwrap(), // LOIK OTTO MAURICE HOMMET
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EvzgXUfUU74zhUJy1vXTQeV1nsBfVMcaYbGUsTV1vxYgdBC").unwrap(), // ALEJANDRO GUI BASCUNANA SORDO
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5E7cHFXWqPTif4x2fNj2Cn3hR1DzcogyTB46ba9UWW4jikt3").unwrap(), // LISA MIREILLE CHARLETTE COGNET
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FCYQwkz6gLpm2zn6cecQrex4d8wtH8tTh5GeGHjSUrYRcpw").unwrap(), // BARAN ALTAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HCFBPBhiyCi4gH1j7tWQjQ5cR46fWvya7f5nT2g4L1d1EjP").unwrap(), // NASSIM MOHAMED KHITER
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HQ9TEqdC2PstXxX4UL5kLSUftEw7C9YZhHGNhqZs8mpnSJv").unwrap(), // VALENTIN PIERRE CHAPONNAY
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HmMrpbqkdqEsaSuW8PfVqPi37Jk7pGSLzHKpJTSy7xFsCG6").unwrap(), // JAN BAPTISTE VALERY DE BRUYCKER
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GuytJKogtzZnehCrw2grrJDXSCM8oHqC7sQE9kqsejxvMkq").unwrap(), // PHAN TR_N TH_Y HO�NG
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EWfj38RieR6c438imVhUyrqN1e3xupqjmMco4hqYepUvRJM").unwrap(), // TOMASZ MICHAL SOLAREWICZ
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CwcJG3AU8YhoZsdH78j2B8MCDhqxj4hkPstuB6b3VyWkWsj").unwrap(), // JEREMY SCOTT SNIDER
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CFpD4Bzr4C6ne9TGTXrtWFyYV8C7i3jgku8vQdfpH2Cv2DX").unwrap(), // MICHA_ TOMASZ SZAFRON
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HmB6hQw96E3a4pEkvnTTDts9NfPYS2Ef3h5x95LXLrf648u").unwrap(), // AHMED MOHAMED ELSHERIF
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EvDqZaMu7ay3zB7xqHWihj6GgTD6Q42hoVfGeddAcJwEB2C").unwrap(), // JACEK MACIEJEWSKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Fgud3XB4cmLXzfBrRpb8rWx5Zy5ekrsk54KgrjboKRB5FgD").unwrap(), // BAHADIR KURNAZ
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FWP4y7g5LSXvVQT6esuAT5uVTTLqVHUduwY3VmmvVkRhuAD").unwrap(), // CRISTEL FONTAINE
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FqUpd7Cbej5kK9CynMn1s3qyKtkDw927HdxxS3bs68v94HU").unwrap(), // MARCIN EDMUND IMA_SKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DDVNLHmhTgAJQEgN97kQ7RHdRPhya8S2SFa8PoPwoom3fJt").unwrap(), // CIHAN KARATAS
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Hpzw48GDUtc2X8jfNEaVQ7nA1hNvn7qHcS11btxW9QQJMLb").unwrap(), // HUYNH THI MY HANH HO
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CASDXMydcH7kGrCb2cVwwPiN7RviXTTdxnnVDYL29U1wbMw").unwrap(), // JACEK KRZYSZTOF JANIK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HBHaYWWgaTr19a6k9Go188JYzVn5KZACN2VAGdjkjaASJLc").unwrap(), // MICHA_ MAREK BORCZYK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CA1UYPyPGS3kcBS6vJ44BRzXJYmN7AGD2ULNucmRkzgVeU8").unwrap(), // SOM BAHADUR TAMANG
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FtHtRtL7WbTMKJWufBXsVm9YWKX7Cs5B3q5uWqQ2GpWLZQW").unwrap(), // CATALINA ANDREEA ROMILA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DUrurNNX2CkD6stiQCvMEFu1Fo4tzyrefK8iVCKgSxq64dg").unwrap(), // KRZYSZTOF _WI_TKOWSKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CVTW4StguUEHWmreq4Gi4j8TYco9cu81UvzXBvXT99P3sws").unwrap(), // KONRAD KULIG
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CJo8o56fqH16EK9cYG2jZtEjQubNDRE2XTHVLMVcC3zYeoT").unwrap(), // LUKASZ PAC
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5ECh6snLLitBHEFjtBpCYfwUVC3rT9rEi9jzusCY1kBhzK8A").unwrap(), // ALEKSANDER FRANCISZEK SWINCZYK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HB3NoD5a38or7i7jFpJnNB9sjqHdDxp61qGdHjPHQt3zPcx").unwrap(), // ERIC TAN JIE JIAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HnNA6rGjrTL72Bia6MozSA6u7FF4Eo2VKt7AdUGHVMLss96").unwrap(), // R�GIS BOLIS
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Gn5ZGaZnExKSBgz3N2u697pt46mnXk81L8bQo3Rm3kw5u7v").unwrap(), // SAMY RIZCALLAH
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DSep7DZop3iZ1BH9S6nR8gqt6PaebKoJDv4kTNvMbYn2aod").unwrap(), // SERGEY GUNKO
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GEEUip4kYDtQwYFfaZuSVBFZjopND24SS2ja652j6GrCSVS").unwrap(), // THUSHARA SAMPATH ADIRIYAN MESTRIGE
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FjdQuqJaYwSCfz7PUYp1hCcrJT6K9xjvtECi3V1YSRcaWqM").unwrap(), // __ TU_N B�NH
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HicdXFYJ1iSgE9V8qvvwFKKiwr9aNpEoyihqMndDn8nEXWV").unwrap(), // AHMET DEMIR
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Gk6ApfG2d33cT1UCvb625qfkpG76wFoSNCJH8wn9Ss5LzAT").unwrap(), // AYELEN FABIANA SALAZAR
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FBwGUuN4V8w4BQFtBUZg56TN9jtKdNH7xS5kkDcrbVamwGz").unwrap(), // KONRAD SEBASTIAN CICHOSZ
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Cr9k1YzQ52atu5z6BFbnotTTHT8KeXUqDA4hhwdHWTib84v").unwrap(), // GOUALI MARTIAL ERIC-GUSTAVE DIGBEU
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5H8yMjWuzEGEmhNyFbg5JhmbwtU4pY6XDn9oNwr9dnfo4BzH").unwrap(), // SALAHEDDINE BEN BOUAZZA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HENNyFZuhGjTKfo7foz58Zs4BKcbGAaMqSKCCV5oUA2ZVAf").unwrap(), // ROBERT JERZY MERGNER
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5H48J6WELGUXsQhjcrfsrNBkNdyoQ5vBKG7uYtuqFrj9zWGP").unwrap(), // TR_N NG_C L_NH
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DHbzqsv4w3CQ2yMpshJHzgMo4pohoFZ8yuTbnpEVu5Syw82").unwrap(), // PHAN DINH QUAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5H1cmQwd1jRKYHspCiMRNx9pVqaJCEa4qTdw4dxVWZx8EHQv").unwrap(), // DMYTRO KUZMIN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CfPHH5GtGkL5D9WZB7RTgCrhxfstpxKwmuGxoZc8tUxKQhn").unwrap(), // WIJETHUNGA ARACHCHIGE MITHUN HASARU
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FLFZeJec63MY9npda9AwKVJNfRJoc2kJ4kC9ngXaUCc4rWt").unwrap(), // KAMIL D K�_ECZKO
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GHLMzsx6mWYwbWcBSFqnQQVP7DccbQjgMNxJtceQ4piSRio").unwrap(), // OJO-KOLAWOLE OLUMIDE ALFRED
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HbBy9xHxY844Dm9whzhzn1nSF92E5knz6jYCsUipR4SfKX9").unwrap(), // ARKADIUSZ KO_EK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DcsDuTHLfFmGWtmZW9EEvWWvZEW3C6zQbgNUiADTJiuMfgS").unwrap(), // TAN THANH VAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GUDK338Kkwj5tqzsgYq8nCqFupR8PYPrcDKTFpReBJnd4uu").unwrap(), // NDWABA ELIZABETH OSOBU
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Hp5WQx2RmJwk1W14xUbjYfppdvCiFE1RzPjuRfX1GykMa7T").unwrap(), // KAROL MAREK KULASIEWICZ
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GgvWf633Eokey5wSaJ4LMUxdDFgxLWhCcxyEbCWe4q7hhRq").unwrap(), // THOR H�KON WANG
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5F2RRb5XNyLkndVhDBgG178PPprr7tLTumGhCmMCX7FY6BLd").unwrap(), // IOANNIS TROULIS
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CfLNHsqMydp3MQ5LC6bCnTQu4nTD2nqgNWNb5NkurVUxjch").unwrap(), // ARRIS BOYACIYAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5E9pNsPCCjr8MvY1o3Katzx3v1NgsaXVP14vpvkp64z34Km5").unwrap(), // OLUSHINA A BASHIRU
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5C8ZRhWGiAQEjnPkm27zKY6KMD8r4BZaWEiEwMQTkVgj18vV").unwrap(), // MURAT F__NEC_
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DLFCSjPe89FLn75G28vxphTWEfwyEZvEJQXDAW3yZ5EjTkB").unwrap(), // LINAR LATYPOV
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Gc9SnsHkyqMX37oHWwgS9hBtuuSLujhsth6dXhdZkQgnD9u").unwrap(), // EMAM JAFFER SADIQ MOHIDEEN ABDUL KADER
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Dc489UukEDKmRLwNsUvuWbRyjT7ErQ6gvZtDX9wRfX5esQc").unwrap(), // JULIEN BAUER
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CorQdsXpjvQj6KfMa1BMya3n9WVYgxpeBiCEoETrToLfQFn").unwrap(), // ANDREI YEMIALYANCHYK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CPUwRy36yHJU3hMr3xbbKcRh9QNKg6mLhhJiGajhuBQE8vH").unwrap(), // EROL SENOL
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EReZcJHjuQ2J8SJBpbUeBBWpuUhkjKnZmaCVqh2RvgCxvJY").unwrap(), // L� V_N PHUONG
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Gps3QiEBkr51e4YRJeE3FxZ2LbxpQLuA1x35zPQPb86Ubzi").unwrap(), // ROBERT K_KOL
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5C7zcdThasnLciuxH7BwWk7BGRqsMrbEg6hEd5Dbdg3SNBGR").unwrap(), // EMIN GUR
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Gsiyd4qCxv9q8w1F3VKZS8VizLKRUhqV9JJtEqz6DjgH7RV").unwrap(), // FELICE EMANUELE VALENTE
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DebCxcwKUKNUSooMvHbKVkCMqG89uEFGcMcRhA78o4x4Hz4").unwrap(), // CONSTANTIN DANIEL ARPASANU
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GEtQCQkA8ZRpSS9uA2TtoFEsxSoy3hnzpJFZPvjp2qcDDJp").unwrap(), // VLADLEN CHERNOGOR
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CDmvGjUMbUYowos5HmmGuHAaHbisgw7We3vc6a1fpEc3wHP").unwrap(), // GILLES PHILIPPE GHISLAIN COGNET
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DqVZo7msUazKrdwWhaxQoTEz35EqTn7G8d32WAZvzmUuAmr").unwrap(), // ERIK FRANCOIS EJNER SCHJOTH
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5D2vsuXN2B41LyvVLjDAxczRj7UDKxg8vZ85awK4JXpDWHxm").unwrap(), // DANIEL PENA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5E7WRmoVhFMNiMsrfMDi9twcrzeqsh1StpaJ5Qmnpq1DAriz").unwrap(), // ROBERT ADAM PRASKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5ERCXojWmTGrSqSgHML4dZkHV1mbB8gybmSe2uWMb3BnvNbg").unwrap(), // JER�ME BERNARD JEANSON
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            75000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GR3hLLqPm7t3e9acKPcUgFivA5tG4wb7HUkZqxze4DJsS9M").unwrap(), // CHANGMIN AN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EFD9ppXaxmXiQUqwvQ5yetCU7W9AA6YMdMC4cENCsM1w4cL").unwrap(), // BENIAMIN _UKASZ FOJCIK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FxwgVjHiK5fxp5CBeHSTVLP6H2qSKPLvvCdfMh7gcJdYspk").unwrap(), // WASSIM ABOU AOUN MAJDALANI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CXr539EJNqsJh5uzj7Ad21eiZ1uMJafgWa6EyoinwUjuDrL").unwrap(), // GOKHAN UGURLU
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5He8AdBZ3NRnrofaPJ1fNcmJcsHMxi5aCSJvv3k6CL7Uh89H").unwrap(), // REKHA V
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Co3wwpjJwDueAniMhCvEKXY53c412rRTpXqt3paciKKmFAJ").unwrap(), // BARBARA MUCHA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EHkibFtQLJBtMknMt6GK1ivQ57LPyQABoSg9TLJAS23jv6E").unwrap(), // KAMIL JERZY GOLEBIOWSKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CvyJDjYNm7SF5hzvsBkcepPc1xRkPYiM4LpA96PVTXh8Tbs").unwrap(), // NGOC HUNG HOANG
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5E4MkdbaXoo8y6HyzMQMUESjDzurEVYEcDQRxQvokPV96jMf").unwrap(), // BEATRICE MIREILLE SONIA PIEROTTI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5F1Pb8dZxmrKh8ZDvmFcdV365ue7e36ZqLMbizmwwtJso4bw").unwrap(), // BENJAMIN ALEXANDER CORNEY
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5F9pnipps8oZK685cXmB8Dpv176M4CgGyE2xQVqEz9FyoFkS").unwrap(), // NAWRS KASHOUT
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5G6t6a78XmdBnbxqEyo5xhRYJUSz9QkNisUPgznSSMYbLiQV").unwrap(), // ALEKSANDR BULANOV
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DebwzrFf3JknUj1uof38sR2kbqzwtvNY7R9dxC4bqBukr3W").unwrap(), // LALE COBAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FUaQ146MTPUxzsabJF66N6GZ13uLrJsrbaqKG6fEx36c1Rj").unwrap(), // TOMASZ ZDZIS_AW RADOMSKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5E9Yu84K3kiZGPBuj1ATL84rYQUwn95ckJvJCLq1BVi1DFQL").unwrap(), // PHILIPP HOLDMANN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Ca6LWiu8Jq3vN7xsaXy2tuhWzDgDtmEpHkBJ3SLTeDHQXsN").unwrap(), // SANJEEV MENON
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Hn7EaKxJu4tR9keTNL4gZDsUfQoG5r4qtMif58gcA4fSsET").unwrap(), // MACIEJ JASKULA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CJsAiwx4TQqZx746L4MQp2ojzjrBQTHGmNqHE4pzPxe2chT").unwrap(), // DMITRII MOROZOV
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CJPEDH99tmA11aR1pPzunvChWd9L4S6GPnRGHcNsYPHn2oy").unwrap(), // ISAAC ISRAEL BAUTISTA RANGEL
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CUMMm9jBB3JsLs4r7YJfxgbi5Znf8DPZbhEErxn2pjTu1SM").unwrap(), // LUKASZ ZINCZUK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DHoAuLBEqEpSV7YEW5porEpSMDnpJoYCupLyQa8gWkzy4hZ").unwrap(), // GURUPAD DUGANAVAR
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DLcLEKd7dzRegwBMRncJnMhJJCQEboFmAhU3QwSHiCgxvYR").unwrap(), // S_AWOMIR JAN GR__LIK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CQLMSa3LSGW3EeWGV4GxEYRKB6a4zuxsqFw1kUDhvezAnkW").unwrap(), // MICHA_ WOJCIECH MIZERA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FEk6bFte4y7guQeHDuusoeehNrwUkBDJ4RFLQ6bzWrqwi7y").unwrap(), // MARCIN KAMI_SKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5F1kmu133XZktCLBbbEBMLxtCevTECtcX2vGXFQpL1NsQSvL").unwrap(), // ELIGIUSZ PRZEMYS_AW SOBEL
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CJGBYo6nHU9veGafBWYW9RTZRcU7KHnyz6uTuQXG8e3BGF2").unwrap(), // ALEXANDRE XAVIER DION
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HbJHGU25io7JCKoni9jjrxG2jnWdiknfxU63mAJMEZkvmYz").unwrap(), // RHEINHARDT INGVAR GERULA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Gx2PeqSyiWXgDoa2ntrxv9G14aJTBfmGEXZMzYU772GNF3M").unwrap(), // ___
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5D2TFuGtzJ2JFBPHFgijNebgomrx81G9e7RAWCDH1adRLJV3").unwrap(), // ALAA EDDIN BAERA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Eeun9BDEHQXcZB4YmihUoZybcaqdkxkNPJcY9dpTSVxpXTz").unwrap(), // KRZYSZTOF PIOTR KUBOSZEK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Dq6Wpsf1vKDsH4q3jVngmfHeMtCHgDRsWcQnUUk1xNDBUG3").unwrap(), // FAIRUZA VALIEVA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CkbRgvtiCC8sNt1z6mTUCqvh3e5LmJD5jfEPX6Jn6epL6Hz").unwrap(), // ROMAN DUDCHENKO
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GRM9uao4EeNyE9pSMN4soDF7omgaVqJutAjAgokTvNyxct5").unwrap(), // MUHAMMAD KABIR MUSA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CaVkagPsW3whEkn6GczdNCSZxSRF3aLUnNX7ki3797mBDGs").unwrap(), // MATVEI SHAIKEVICH
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5F45KEELCtM3EAkxz2Qog3u9A3QBKFxGXxAMWkZNVXSVm9Zm").unwrap(), // QUENTIN JOSEPH JEAN CLAUDE FANOUILLERE
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5H8rdq8poLvjPTbPWPus5z5dMN8D4QaPRweoa191nYuJk5jR").unwrap(), // KAYA ONUR G�LTEN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FhZgrDQxPxKVw1FN5BgeoP5QRKTw5rBEvExEeoamC89SaKa").unwrap(), // ALATTIN SANCAKLI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HmMEvxoq35M97owjwqFKjxpHUsSnLHvey3ePyTMxKyjccr8").unwrap(), // AHMAD GHAITH ALSHARIF
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HTdj7twNHdwtWY9zcyAnFEtiAVSjx7DpwoeSZbyKzj6M97s").unwrap(), // LUKASZ BOROWSKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GTk2jNe3dJNN4Uj3qwzRM1KNPSm2GD7eUht5h7QAy5wmd1q").unwrap(), // VAN DUONG NGUYEN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Fs2UQNp3a2C6gcU4zgHjWUBitN4eKkwsdbG7umqwCLDmawb").unwrap(), // SULEYMAN CEVIK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GLCPXD95akd9zvVx8vo9wD7qvwofaHWsSEQPQxzq8XpfUEY").unwrap(), // MUSTAFA YESILYURT
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EcnvRnYWfGoa1NoZ465i42Qk2Sa8YTuZmfHJbQsAjKnSujN").unwrap(), // HUMAID KHALFAN HUMAID ALSHAIN ALAMERI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DRTXnFHDAxztCdEo6qB7EN1V6fK8jxCrdhizypM5AA7zXoP").unwrap(), // HO�NG _�NH NG_C
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DhUMwaWjfiioE6WDXpcWdES6nmGdoMRZ7AzskR144a6CF56").unwrap(), // PAVEL OLEGOVICH ROMANOV
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CM8gWNaPuAk5bk2R8mSGUesYvw39HsAFgdViC4f9eLCPaze").unwrap(), // K�VIN NAVARRO
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HMmZWR5mrUGLAZHEcBhp34RVPVJ6DaWGjKq1pNcZPMvuCGo").unwrap(), // PRABHAKARAN A
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            150000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Ek6ypxFwx5QgJ2MvuztfhFJhjhD41oXggRQPT7H9WHHjaaS").unwrap(), // BAPTISTE S�BASTIEN BUREAU
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            375000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DNms3kq3pThyQjhsGrdUBBAQLh6JdfGzXvucDTiBKFNwXnb").unwrap(), // RAFA_ KOMINCZAK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            375000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DJQqn7Q4LXofswfhtpqZNws44U6JymhzKWGgUs3AKpWfurw").unwrap(), // TOMASZ KRZYSZTOF MENTEL
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            375000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Gzkrn8wa6u6M8C7qoYKMuCwdNYqusRn5tDg2FoCQQCYaJR8").unwrap(), // NHU CUONG LE
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            375000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DNocQ1r64vk8xXhx3qC4VUcDo2yARMPfEoGGhRjEFBjkTGR").unwrap(), // ESER UYANIK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            375000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EFMNTTzujmzwVfwA6W5zYfGcRCNkmsHc1BjD2cUSfTa751U").unwrap(), // MHD MOHANNAD GHANNAM
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            375000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HMsSWJgXw486DKzKJfmuD5K4BdUeS9ixaVgmDKjJJBdYUPR").unwrap(), // ABDULAZIZ HUSSAIN M MOAFA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            375000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5G3emosXFDpUbRV4dzF67Zd99U2hGxkB3mmegJ6g45h9bZGb").unwrap(), // CHARLES CHO
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            375000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GLFLTjFDoHPFFCRQ8YFvaBAZzCtBZBCv9K1maJxqD6QQV4v").unwrap(), // TOMASZ KRZYSZTOF BARAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            375000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HGt2Q6REHK89YKTozBLDDqki2bFXP7L8qbU2iM7xBqwsczo").unwrap(), // JEROME JOSEPH JACQUES
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            375000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GjfJHTSY1JTgGqSidn4LnFpwxcsuZo3UJAqCEXvuM7cFTbn").unwrap(), // KEVIN LAURENT HOARAU
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            375000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EgwYPk3bWYTzwqAC6Qig2bYsMXVAkVb6SWC1JpZpFL2qs9w").unwrap(), // H� MINH KHOA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            375000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GYcE56KkPfvFkrrChxQ1bX78hgqAhwJkE1ffaxVXmXRteS3").unwrap(), // ORIFJON TUROPOV
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            375000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5G4Zxitrc91nWnYzZNyeHWyJsMeuWHu2uJjMVqEg4dPcTD1j").unwrap(), // GRZEGORZ LIZEWSKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            375000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FBpniJMt1MGg8PaP8rDZRU3DbHbM6E5sLkVokuK3xrbh32i").unwrap(), // MACIEJ RUDZINSKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            375000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HYdetyjfQBUi8u1tp6j6xGXeLuvoNGWFrK86d8MPFbfhE6Z").unwrap(), // ADEMOLA MUTAWA ADELEKE
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            375000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CJhB914fmc36aXgxMBdjG86zbrgQ6JFUddYQ16R12hzcrkf").unwrap(), // NICOLAS PELARD
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            375000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5C52JPV1reDw49FEy5WcEzzbkqpaQbDiTzSYbxHBi6YZGy6L").unwrap(), // JULIEN YVES SOTTAS
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            375000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EEywi73XUYYPGfdK9urHmE9kzFWpkDZkLUrL2BGe1BRr98y").unwrap(), // WOJCIECH SZA_AJ
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            375000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GZDHnkgr482kAaG8ZJjfa7rDqGs5eVcghydnNyPAwfN3NGY").unwrap(), // MI SHUG MERET
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            750000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HKoudvvAmyr7M67iDswU3eni2riMuwE5xX6yShBY1q6U67s").unwrap(), // CHAI HANG CHOOI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            750000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CA59EKuBWXW8tc4G1ZKGurczn7WWakme9CGnA61dK17mYeG").unwrap(), // TR_N TH_Y H__NG
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            750000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EX269zEYT6GUAdyuVYadJjCrHZGbYHH9sZPMFr6CN7bVUs9").unwrap(), // KADIR ASLAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            750000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GHRXyJrvJeUXExS6oNvdZdqy9Hn5MdtqCtVM9Ni3nUjv41B").unwrap(), // MYKOLA VIITIV
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            750000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GCJSXLB2kzFF1XXRVQ1exg6nwkMBYY7pCLGs42YdNR61AP7").unwrap(), // SEDAT GUELBAKAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            750000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GLQCYsrQfR1GtXwtpGyRQ1hsyhN6yB9YM5S5AyKvm4ExRvX").unwrap(), // SOFIIA ZINKEVYCH
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            750000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CRtAAFZ7iWTn3iPyPN5aRsGVof72z4njP41QXzSA17ANjaE").unwrap(), // ABDULLAH NAC_ KULA_
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            750000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FPDCX61CsbLQn8yiDm7n4zZE3gqJ28DUtDnzhTzNhJ5qgfA").unwrap(), // MICHAL LUKASZ POWA_KA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            750000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5G4122JFTzaL9C3sKR5ZRW8M4bgbVzXmZYPxndPyUS7tr7Zc").unwrap(), // KEMAL G�KDOGAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            750000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GKNrruGWJ7WQkuZPBNVH3RCGRHs8hsFDcGwTHNacwRv232u").unwrap(), // OLEKSANDR BIZINSKYI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            1875000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GWoHGn5SE6xyywKdavCKERdkcBEDzvcqHC5JuLkWW2nMJBP").unwrap(), // DMYTRO PUSTOVALOV
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            1875000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Ew9VUt2U7xu3XswQU7DbfMefyAP4P3PKKceJf5SvAdtX1QF").unwrap(), // BERNARD E BURST III
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            3750000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CvdKkEGRsMDjkyiyrBhiUdf2zmCzXzbQeVRQsWV2sqihzWj").unwrap(), // ANTON NADTOCHIY
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            3750000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5G3p1RvBR8ZHjpJGcSq4ncfkbi7bnaHujUr4MZnMfeP96hhh").unwrap(), // SEBASTIAAN Y L VAN ERNE
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            3750000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5F4cuDdz7kQkcTYVWdQ3dWijij8uEySAZYKdGXHyqtHjxsZe").unwrap(), // PH_M THANH CH�U
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            3750000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5F6kwQXjHiui7ZwsA9AFg7JTtiMtZDmC5bNGdcFUM88TANF2").unwrap(), // OLEKSIY STESHENKO
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            750000 * DEIP,
            true,
        ),
        /* PRE SALE 2 ============================================================================ */
        (
            AccountId::from_ss58check("5Ejq7fBCnK7w8CEN94wMWtYpmr5FrMaZK8PWhjy8i7SQcC65").unwrap(), // DZMITRYI SHCHATNIKOVICH
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            15000000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5D9vjB39SpUVtA6rkrtro9MESr4yzr6JxTB1tNVd49t2mv5n").unwrap(), // VALENTIN SOKOL
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            900000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EHxPn4GjSBAum9JF7rSrcpHkDWErpHf6sBexhyBGqzgfwj9").unwrap(), // GORKEM BEREKET 
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            3000000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FpxzrSpUMp7pngGxzFtnnyKapb9AjypGJ9Kxbnb6THwF34L").unwrap(), // NINOSLAV SRETENOVIC
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            6000000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DMD9DWmKZohxuqAFsAuy3poqinybnAp6qttwwr3MFyPrfxi").unwrap(), // PIOTR PAPROCKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            1500000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GsYznNyotNJn2uyBYevPGBXPsTWC6NJ8WUhKwYmPLk8DL8P").unwrap(), // VITALI BUBEN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            600000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DaM5Bzcj75skNtPCuZPcgAhL5vgYJQWNKtG1EGMwYsBQ3x9").unwrap(), // LI MING-YEH
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            600000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CoMRyUo48KmdrQZSb96UBRX6vBgFRdAxMGQxiLMWXGn4xoU").unwrap(), // SZYMON MIOSKOWSKI
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            600000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Gh5sgV5GL8UvapJQ5D39nRN8UkRCxwRN7H9LjgnoQeAGAES").unwrap(), // ILYA FIADOTAU
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            600000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GECsYhbYjDifRWnBbhrg5MgT5eJ5jj9EJTycAFiWqBBe5t8").unwrap(), // IHAR KRASNIK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            600000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Fh36EmGgZDQKdrj3q123bNL8sKMZY3f6wEbGNJotyHQ2JkC").unwrap(), // WIOLETA KOPYCKA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            600000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GxbE2r89cdV3U37GwjEvNSf5EakXumbSe2nym9ZQ5YAJpUy").unwrap(), // ROMAIN (JEAN-MICHEL) AVRIL
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            600000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EoFsYhMgkLZnouhhUZTX5rtLF4onSYBV9Bjqn4HfaKCa5ZP").unwrap(), // TOMASZ KRZYSZTOF BARAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            1500000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Dz8GMVA3L2tDJYVtjGLHKK1EzTZwzX16YYjELMe7B7iVGZT").unwrap(), // DAMIAN CICHOSZ 
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            600000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DXbsLbvQ5BAdTEG75iypqmkSrujG4wTZp2xdCMZHLdzhJ5E").unwrap(), // ANDREI ORSICH
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            3000000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CMHN7W6bDQ7JBnoCpe8cnRxH6SVpoUfxmAj35GKXjHaVJHE").unwrap(), // LAMANOSAU VIACHASLAU
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            3000000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FeJLNDcpKat31UsCouLPhAjVsG5MxmuTduxH8GtNnU1pmcf").unwrap(), // PHAM QUOC DUC 
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            1500000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5D7AhhKY17fQkpmYoVjZ7rJpmfQWBAnBRUK6UY9UybnK9Qv9").unwrap(), // ALEXANDER SCOTT COLE
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            600000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CFYSNuo4Xf7Qzhoi9kvUm8BbkFDHWvS5JiSSqzbYWXHZQPb").unwrap(), // KEMAL G�KDOGAN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            3000000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Cd9CMi9nyfXGYRzUxE2uzMxURUXC4E8o8yRU6TbQLyiDmpM").unwrap(), // BIZINSKYI OLEKSANDR
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            1500000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DSc3NK3D2N8fZ3Mu7bz2kWC5MiWiw5q23E2uKijNv8oX37Q").unwrap(), // THIJS SCHRIJVER
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            1500000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5FgikPX1o9jFfmpkgLVe7H6Mg3DZVvMGgk5Eo51S4moXWqFA").unwrap(), // BERKAY C'ATALYUREK
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            600000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DUXdT1M99tMNuR1E83kSMk22SWUhufmh1jCegdeiT5WFgHv").unwrap(), // VERNIK OLEKSDANDR 
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            600000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CexETqnfLugG4HAnaK1e32UYEqfAobC7zcmwuvq2FuquwAi").unwrap(), // STANISLAV ANDRIUSHCHENKO
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            1500000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DhBryXMZgmKG9nXmqYWsU2NnFZFerGpt2is1k5LmHUNqx2J").unwrap(), // STRYUK VADYM
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            600000 * DEIP,
            true,
        ),
        /* PRIVATE =============================================================================== */
        (
            AccountId::from_ss58check("5G8mQWUT29huVVx6owuAoWBuTLdL5uboiD1qMoysxn7u5xo1").unwrap(), // ALEKSANDER STEFANOWICZ
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            10000000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GNGowiCgZSxbXvuK11kraPgUKRrN2dEfuseoze2UmpcckEf").unwrap(), // ASHTON ADDISON (CRYPTOCOINSHOW)
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            1000000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DqWNLhwZw2ibBoYvBA5TRwYcsaZhrsVPWCjk6z5XgqwYBbh").unwrap(), // FAIRUM VENTURES
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            14500000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5GjNxHr9hwkgyLtWgVtxjnYMsd6EyquFJLkECDDmTFgs2VUQ").unwrap(), // GAINS
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            20000000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DDSHCVEzvVX29nFq9PhXeJDSKxTXoaPBox3qRvqUHTWxp3n").unwrap(), // TOKENSUITE
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            10000000 * DEIP,
            true,
        ),
        /* STRATEGIC ============================================================================= */
        (
            AccountId::from_ss58check("5GubXsYvvjsyf4Pi3jE7w5B7W9c7gZQWXMJpNrvHKyaX8Vmj").unwrap(), // BLOCKWALL CAPITAL
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            3541666 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5G9YgYppcvsfCNWEMgKmFf2PBsFgezpHt9Yb4FMVyozaZp3y").unwrap(), // FAIRUM VENTURES
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            12041666 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5DkCHxyuxENSVLvd4AqrTcYyqDJJJC2JjJpNXoqs6HEeWDTA").unwrap(), // KEF VENTURES
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            7083333 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5Chf4ksgxVoiSovkSBWYytxCG4GTTiE5PQ2SeZ8De9c9SZ3p").unwrap(), // SIMON SCHWERIN
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            3541666 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5EvCg4bNMHEcsKVLW5tXBpUed2tvhWCqQP89o9qXEVks996C").unwrap(), // TOKENSUITE
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            14166666 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5HWF2Xz2gr4abfbtcEZPPRaETB7Hhp7L53bcW4ZLnoyfEZz7").unwrap(), // RAY CHOHAN (PATSNAP)
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            7083333 * DEIP,
            true,
        ),
        /* SEED ================================================================================== */
        (
            AccountId::from_ss58check("5Exf3FTXKUPJXDWPvHQC48un7ZkZ4EFbZYPz3GavrUeBJHXm").unwrap(), // AU21
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            11250000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5CSKm2aRkdtYEfuUDuHDYXhCed4eVffMoZoMLzHVw2Py6BTy").unwrap(), // BLOCKWALL CAPITAL
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            5625000 * DEIP,
            true,
        ),
        (
            AccountId::from_ss58check("5En3sVgNTmDTXB3JaYNq6kQ6sjncvvGZQaR6YcmJExb7LpsS").unwrap(), // GLIB DUDKA
            1651276800000, // 2022-04-30T00:00:00.000Z
            0, // 0 months
            31536000000, // 12 months
            7889400000, // 3 months
            0 * DEIP,
            4500000 * DEIP,
            true,
        ),
    ]
}