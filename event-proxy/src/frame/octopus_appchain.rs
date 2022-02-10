use frame_support::Parameter;
use sp_runtime::traits::Member;

use serde::Serialize;
use subxt::Config;

pub trait OctopusAppchain: Config {
    type Balance: Parameter + Member + Serialize;
    type AssetBalance: Parameter + Member + Serialize;
    type AssetId: Parameter + Member + Serialize;
}
