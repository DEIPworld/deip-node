use deip_serializable_u128::SerializableAtLeast32BitUnsigned;
use serde::Serialize;

use crate::CallObject;

#[derive(Serialize)]
pub(crate) struct AssetsCreateCallArgs<A: Serialize> {
    id: u32,
    admin: A,
    min_balance: SerializableAtLeast32BitUnsigned<u128>,
}

impl<A: Serialize> AssetsCreateCallArgs<A> {
    pub(crate) fn new(id: u32, admin: A, min_balance: u128) -> Self {
        Self { id, admin, min_balance: SerializableAtLeast32BitUnsigned(min_balance) }
    }

    pub(crate) fn into_call_object(
        self,
        module: &'static str,
        call: &'static str,
    ) -> CallObject<&'static str, &'static str, Self> {
        CallObject { module, call, args: self }
    }
}
