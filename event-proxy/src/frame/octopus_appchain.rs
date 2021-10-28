
use substrate_subxt::system::System;
use substrate_subxt::{module};

use frame_support::{Parameter};
use sp_runtime::traits::Member;

use serde::{Serialize};

#[module]
pub trait OctopusAppchain: System {
    type Balance: Parameter + Member + Serialize;
    type AssetBalance: Parameter + Member + Serialize;
    type AssetId: Parameter + Member + Serialize;
}
