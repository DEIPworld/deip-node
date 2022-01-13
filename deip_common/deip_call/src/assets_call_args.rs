use deip_serializable_u128::SerializableAtLeast32BitUnsigned;
use serde::Serialize;

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
}

#[derive(Serialize)]
pub(crate) struct AssetsForceCreateCallArgs<A: Serialize> {
    id: u32,
    owner: A,
    is_sufficient: bool,
    min_balance: SerializableAtLeast32BitUnsigned<u128>,
}

impl<A: Serialize> AssetsForceCreateCallArgs<A> {
    pub(crate) fn new(id: u32, owner: A, is_sufficient: bool, min_balance: u128) -> Self {
        let min_balance = SerializableAtLeast32BitUnsigned(min_balance);
        Self { id, owner, is_sufficient, min_balance }
    }
}

#[derive(Serialize)]
pub(crate) struct AssetsDestroyCallArgs<A>
where
    A: Serialize,
{
    id: u32,
    witness: A,
}

#[derive(Serialize)]
pub(crate) struct AssetsMintCallArgs<A>
where
    A: Serialize,
{
    id: u32,
    beneficiary: A,
    amount: SerializableAtLeast32BitUnsigned<u128>,
}

impl<A> AssetsMintCallArgs<A>
where
    A: Serialize,
{
    pub(crate) fn new(id: u32, beneficiary: A, amount: u128) -> Self {
        let amount = SerializableAtLeast32BitUnsigned(amount);
        Self { id, beneficiary, amount }
    }
}
