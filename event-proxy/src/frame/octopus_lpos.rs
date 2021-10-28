
use substrate_subxt::system::System;
use substrate_subxt::{module};

use frame_support::{Parameter};
use sp_runtime::traits::Member;

use serde::{Serialize};

#[module]
pub trait OctopusLpos: System {
    type EraIndex: Parameter + Member + Serialize;
}
