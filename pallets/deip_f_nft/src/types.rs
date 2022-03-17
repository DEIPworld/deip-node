use codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Clone, Encode, Decode, TypeInfo)]
pub struct PayloadDetails<AccountId> {
    pub owner: AccountId,
}
