use sp_std::prelude::*;

use frame_support::pallet_prelude::*;

use sp_core::crypto::AccountId32;

#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode, Ord, PartialOrd)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum DeipAccountId<Native, Dao=pallet_deip_dao::dao::DaoId> {
    Native(Native),
    Dao(Dao),
}

impl Into<AccountId32> for DeipAccountId<AccountId32> {
    fn into(self) -> AccountId32 {
        match self {
            Self::Native(native) => { native },
            Self::Dao(dao) => {
                pallet_deip_dao::dao_key::<AccountId32>(&dao)
            },
        }
    }
}

impl From<AccountId32> for DeipAccountId<AccountId32> {
    fn from(native: AccountId32) -> Self {
        Self::Native(native)
    }
}

impl Default for DeipAccountId<AccountId32> {
    fn default() -> Self {
        Self::Native(Default::default())
    }
}
