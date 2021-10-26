use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use sp_runtime::traits::AtLeast32BitUnsigned;

use deip_serializable_u128::SerializableAtLeast32BitUnsigned;

pub struct AssetIdError;
impl common_rpc::GetError for AssetIdError {
	fn get_error() -> common_rpc::Error {
		common_rpc::Error::AssetIdDecodeFailed
	}
}

// copied from pallet_assets since struct members are not public
#[derive(Decode, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetDetails<Balance, AccountId, DepositBalance>
where
	AccountId: Decode,
	DepositBalance: Clone + Decode + AtLeast32BitUnsigned,
	Balance: Clone + Decode + AtLeast32BitUnsigned,
{
	owner: AccountId,
	issuer: AccountId,
	admin: AccountId,
	freezer: AccountId,
	supply: SerializableAtLeast32BitUnsigned<Balance>,
	deposit: SerializableAtLeast32BitUnsigned<DepositBalance>,
	min_balance: SerializableAtLeast32BitUnsigned<Balance>,
	is_sufficient: bool,
	accounts: u32,
	sufficients: u32,
	approvals: u32,
	is_frozen: bool,
}

impl<Balance, AccountId, DepositBalance> common_rpc::GetError
	for AssetDetails<Balance, AccountId, DepositBalance>
where
	AccountId: Decode,
	DepositBalance: Clone + Decode + AtLeast32BitUnsigned,
	Balance: Clone + Decode + AtLeast32BitUnsigned,
{
	fn get_error() -> common_rpc::Error {
		common_rpc::Error::AssetDetailsDecodeFailed
	}
}

pub struct AssetKeyValue<AssetId, Balance, AccountId, DepositBalance> {
	pub id: AssetId,
	_m: std::marker::PhantomData<(Balance, AccountId, DepositBalance)>,
}

impl<AssetId, Balance, AccountId, DepositBalance>
	AssetKeyValue<AssetId, Balance, AccountId, DepositBalance>
{
	pub fn new(id: AssetId) -> Self {
		Self { id, _m: Default::default() }
	}
}

impl<AssetId, Balance, AccountId, DepositBalance> common_rpc::KeyValueInfo
	for AssetKeyValue<AssetId, Balance, AccountId, DepositBalance>
where
	AssetId: 'static + Encode + Decode + Send,
	Balance: 'static + Send + Clone + Decode + AtLeast32BitUnsigned,
	AccountId: 'static + Decode + Send,
	DepositBalance: 'static + Send + Clone + Decode + AtLeast32BitUnsigned,
{
	type Key = AssetId;
	type KeyError = AssetIdError;
	type Value = AssetDetails<Balance, AccountId, DepositBalance>;
	type ValueError = Self::Value;

	fn key(&self) -> &Self::Key {
		&self.id
	}
}

#[derive(Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct AssetBalance<Balance: Clone + Decode + AtLeast32BitUnsigned, Extra: Decode> {
	balance: SerializableAtLeast32BitUnsigned<Balance>,
	is_frozen: bool,
	sufficient: bool,
	extra: Extra,
}

impl<Balance: Clone + Decode + AtLeast32BitUnsigned, Extra: Decode> common_rpc::GetError
	for AssetBalance<Balance, Extra>
{
	fn get_error() -> common_rpc::Error {
		common_rpc::Error::AssetBalanceDecodeFailed
	}
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct AssetBalanceWithIds<AssetId, Balance: Decode + Clone + AtLeast32BitUnsigned, AccountId, Extra: Decode> {
	pub asset: AssetId,
	pub account: AccountId,
	#[serde(flatten)]
	pub balance: AssetBalance<Balance, Extra>,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct AssetBalanceWithOwner<Balance: Decode + Clone + AtLeast32BitUnsigned, AccountId, Extra: Decode> {
	pub account: AccountId,
	#[serde(flatten)]
	pub balance: AssetBalance<Balance, Extra>,
}
