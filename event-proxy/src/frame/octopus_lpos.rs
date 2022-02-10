use frame_support::Parameter;
use sp_runtime::traits::Member;

use serde::Serialize;
use subxt::Config;

pub trait OctopusLpos: Config {
    type EraIndex: Parameter + Member + Serialize;
}
