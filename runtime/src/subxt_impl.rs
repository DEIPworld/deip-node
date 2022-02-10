use sp_runtime::traits::StaticLookup;
use subxt::Config;

use crate::Runtime;

impl Config for Runtime {
    type Index = <Self as frame_system::Config>::Index;

    type BlockNumber = <Self as frame_system::Config>::BlockNumber;

    type Hash = <Self as frame_system::Config>::Hash;

    type Hashing = <Self as frame_system::Config>::Hashing;

    type AccountId = <Self as frame_system::Config>::AccountId;

    type Address = <<Self as frame_system::Config>::Lookup as StaticLookup>::Source;

    type Header = <Self as frame_system::Config>::Header;

    type Signature = super::Signature;

    type Extrinsic = super::UncheckedExtrinsic;
}
