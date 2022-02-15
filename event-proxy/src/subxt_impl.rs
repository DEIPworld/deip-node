use frame_system::Config as FrameConfig;
use node_template_runtime::{Runtime, Signature, UncheckedExtrinsic};
use sp_runtime::traits::StaticLookup;
use subxt::Config;

use crate::RuntimeT;

impl Config for RuntimeT {
    type Index = <Runtime as FrameConfig>::Index;

    type BlockNumber = <Runtime as FrameConfig>::BlockNumber;

    type Hash = <Runtime as FrameConfig>::Hash;

    type Hashing = <Runtime as FrameConfig>::Hashing;

    type AccountId = <Runtime as FrameConfig>::AccountId;

    type Address = <<Runtime as FrameConfig>::Lookup as StaticLookup>::Source;

    type Header = <Runtime as FrameConfig>::Header;

    type Signature = Signature;

    type Extrinsic = UncheckedExtrinsic;
}
