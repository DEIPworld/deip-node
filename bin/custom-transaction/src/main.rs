#![allow(unused_imports)]

use sp_runtime::{
    generic::{self, SignedPayload},
    traits::{IdentifyAccount, SignedExtension, Verify},
    MultiSigner,
};

use codec::Encode;
use frame_system::{CheckNonce, Config, Origin};

use node_template_runtime::{AccountId, Address, Call, Hash, Runtime, Signature};

use pallet_deip_dao::{
    dao::{DaoId, InputAuthority},
    Call as DeipDaoCall,
};

use sp_core::{
    crypto::{AccountId32, Pair, Ss58Codec},
    ed25519, sr25519,
};

use rustc_hex::{FromHex, ToHex};

use mock_check_genesis_ext::CheckGenesis;
use mock_check_mortality_ext::CheckMortality;

mod mock_check_genesis_ext;
mod mock_check_mortality_ext;

pub type SignedExtra = (
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    CheckGenesis<Runtime>,
    CheckMortality<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
    // pallet_deip_portal::CheckPortalExt<Runtime>,
);
type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;

fn genesis_hash() -> Hash {
    let genesis: Vec<u8> = std::env::var("GENESIS").unwrap().from_hex().unwrap();
    Hash::from_slice(genesis.as_slice())
}

pub const DEV_PHRASE: &str = "//Alice";

fn main() {
    let name = DaoId::from_slice("test_dao\0\0\0\0\0\0\0\0\0\0\0\0".as_bytes());

    let extra = (
        frame_system::CheckSpecVersion::new(),
        frame_system::CheckTxVersion::new(),
        CheckGenesis::new(),
        CheckMortality::from(sp_runtime::generic::Era::immortal()),
        CheckNonce::from(0),
        frame_system::CheckWeight::new(),
        pallet_transaction_payment::ChargeTransactionPayment::from(
            <Runtime as pallet_transaction_payment::Config>::TransactionByteFee::get(),
        ),
        // pallet_deip_portal::CheckPortalExt::from(name),
    );

    let pair = sr25519::Pair::from_string(DEV_PHRASE, None).unwrap();

    let account = MultiSigner::from(pair.public()).into_account();
    println!("{}", &account);

    let function = Call::DeipDao(DeipDaoCall::create {
        name,
        authority: InputAuthority { signatories: vec![account.clone()], threshold: 0 },
        metadata: None,
    });

    let signed = Address::from(account);

    let raw_payload = SignedPayload::new(function, extra).unwrap();
    let signature = raw_payload.using_encoded(|x| pair.sign(x));

    let (function, extra, _) = raw_payload.deconstruct();

    let ext = UncheckedExtrinsic::new_signed(function, signed, Signature::from(signature), extra);
    let bin = ext.encode();
    println!("{}", bin.to_hex::<String>());
}
