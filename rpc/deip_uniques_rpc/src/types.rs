use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use sp_runtime::traits::AtLeast32BitUnsigned;

use deip_serializable_u128::SerializableAtLeast32BitUnsigned;

pub struct ClassIdError;

impl common_rpc::GetError for ClassIdError {
    fn get_error() -> common_rpc::Error {
        common_rpc::Error::ClassIdDecodeFailed
    }
}

// copied from pallet_uniques since struct members are not public
#[derive(Decode, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassDetails<AccountId, DepositBalance>
where
    DepositBalance: AtLeast32BitUnsigned + Clone,
{
    /// Can change `owner`, `issuer`, `freezer` and `admin` accounts.
    pub(super) owner: AccountId,
    /// Can mint tokens.
    pub(super) issuer: AccountId,
    /// Can thaw tokens, force transfers and burn tokens from any account.
    pub(super) admin: AccountId,
    /// Can freeze tokens.
    pub(super) freezer: AccountId,
    /// The total balance deposited for the all storage associated with this asset class.
    /// Used by `destroy`.
    pub(super) total_deposit: SerializableAtLeast32BitUnsigned<DepositBalance>,
    /// If `true`, then no deposit is needed to hold instances of this class.
    pub(super) free_holding: bool,
    /// The total number of outstanding instances of this asset class.
    pub(super) instances: u32,
    /// The total number of outstanding instance metadata of this asset class.
    pub(super) instance_metadatas: u32,
    /// The total number of attributes for this asset class.
    pub(super) attributes: u32,
    /// Whether the asset is frozen for non-admin transfers.
    pub(super) is_frozen: bool,
}

impl<AccountId, DepositBalance> common_rpc::GetError for ClassDetails<AccountId, DepositBalance>
where
    AccountId: Decode,
    DepositBalance: Clone + Decode + AtLeast32BitUnsigned,
{
    fn get_error() -> common_rpc::Error {
        common_rpc::Error::ClassDetailsDecodeFailed
    }
}

pub struct ClassKeyValue<ClassId, AccountId, DepositBalance> {
    pub id: ClassId,
    _m: std::marker::PhantomData<(AccountId, DepositBalance)>,
}

impl<ClassId, AccountId, DepositBalance> common_rpc::KeyValueInfo
    for ClassKeyValue<ClassId, AccountId, DepositBalance>
where
    ClassId: 'static + Encode + Decode + Send,
    AccountId: 'static + Decode + Send,
    DepositBalance: 'static + Send + Clone + Decode + AtLeast32BitUnsigned,
{
    type Key = ClassId;
    type KeyError = ClassIdError;
    type Value = ClassDetails<AccountId, DepositBalance>;
    type ValueError = Self::Value;

    fn key(&self) -> &Self::Key {
        &self.id
    }
}

#[derive(Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct ClassInstance<InstanceId: Decode, Extra: Decode> {
    instance: InstanceId,
    is_frozen: bool,
    sufficient: bool,
    extra: Extra,
}

impl<InstanceId: Decode, Extra: Decode> common_rpc::GetError for ClassInstance<InstanceId, Extra> {
    fn get_error() -> common_rpc::Error {
        common_rpc::Error::ClassInstanceDecodeFailed
    }
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct ClassInstanceWithIds<ClassId, InstanceId: Decode, AccountId, Extra: Decode> {
    pub class: ClassId,
    pub account: AccountId,
    #[serde(flatten)]
    pub balance: ClassInstance<InstanceId, Extra>,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct ClassInstanceWithOwner<InstanceId: Decode, AccountId, Extra: Decode> {
    pub account: AccountId,
    #[serde(flatten)]
    pub balance: ClassInstance<InstanceId, Extra>,
}
