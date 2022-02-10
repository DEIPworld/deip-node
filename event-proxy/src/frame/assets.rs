use codec::Decode;
use frame_support::Parameter;
use frame_system::Config;
use sp_runtime::traits::Member;

use serde::{
    ser::{SerializeMap, Serializer},
    Serialize,
};

pub trait Assets: Config {
    type AssetId: Parameter + Member + Serialize;
    type Balance: Parameter + Member + Serialize;
}

const ASSET_ID: &str = "asset_id";
const OWNER: &str = "owner";
const FROM: &str = "from";
const TO: &str = "to";
const AMOUNT: &str = "amount";
const WHO: &str = "who";

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct CreatedEvent<T: Assets>(T::AssetId, T::AccountId, T::AccountId);
impl<T: Assets> Serialize for CreatedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(3))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry("creator", &self.1)?;
        s.serialize_entry(OWNER, &self.2)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct IssuedEvent<T: Assets>(T::AssetId, T::AccountId, T::Balance);
impl<T: Assets> Serialize for IssuedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(3))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry(OWNER, &self.1)?;
        s.serialize_entry("total_supply", &self.2)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct TransferredEvent<T: Assets>(T::AssetId, T::AccountId, T::AccountId, T::Balance);
impl<T: Assets> Serialize for TransferredEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(4))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry(FROM, &self.1)?;
        s.serialize_entry(TO, &self.2)?;
        s.serialize_entry(AMOUNT, &self.3)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct BurnedEvent<T: Assets>(T::AssetId, T::AccountId, T::Balance);
impl<T: Assets> Serialize for BurnedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(3))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry(OWNER, &self.1)?;
        s.serialize_entry("balance", &self.2)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct TeamChangedEvent<T: Assets>(T::AssetId, T::AccountId, T::AccountId, T::AccountId);
impl<T: Assets> Serialize for TeamChangedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(4))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry("issuer", &self.1)?;
        s.serialize_entry("admin", &self.2)?;
        s.serialize_entry("freezer", &self.3)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct OwnerChangedEvent<T: Assets>(T::AssetId, T::AccountId);
impl<T: Assets> Serialize for OwnerChangedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(2))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry(OWNER, &self.1)?;
        s.end()
    }
}

#[cfg(not(feature = "octopus"))]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ForceTransferredEvent<T: Assets>(T::AssetId, T::AccountId, T::AccountId, T::Balance);
#[cfg(not(feature = "octopus"))]
impl<T: Assets> Serialize for ForceTransferredEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(4))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry(FROM, &self.1)?;
        s.serialize_entry(TO, &self.2)?;
        s.serialize_entry(AMOUNT, &self.3)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct FrozenEvent<T: Assets>(T::AssetId, T::AccountId);
impl<T: Assets> Serialize for FrozenEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(2))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry(WHO, &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct ThawedEvent<T: Assets>(T::AssetId, T::AccountId);
impl<T: Assets> Serialize for ThawedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(2))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry(WHO, &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct AssetFrozenEvent<T: Assets>(T::AssetId);
impl<T: Assets> Serialize for AssetFrozenEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(1))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct AssetThawedEvent<T: Assets>(T::AssetId);
impl<T: Assets> Serialize for AssetThawedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(1))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct DestroyedEvent<T: Assets>(T::AssetId);
impl<T: Assets> Serialize for DestroyedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(1))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct ForceCreatedEvent<T: Assets>(T::AssetId, T::AccountId);
impl<T: Assets> Serialize for ForceCreatedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(2))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry(OWNER, &self.1)?;
        s.end()
    }
}
#[cfg(not(feature = "octopus"))]
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct MaxZombiesChangedEvent<T: Assets>(T::AssetId, u32);
#[cfg(not(feature = "octopus"))]
impl<T: Assets> Serialize for MaxZombiesChangedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(2))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry("max_zombies", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct MetadataSetEvent<T: Assets>(T::AssetId, Vec<u8>, Vec<u8>, u8);
impl<T: Assets> Serialize for MetadataSetEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(4))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry("name", &self.1)?;
        s.serialize_entry("symbol", &self.2)?;
        s.serialize_entry("decimals", &self.3)?;
        s.end()
    }
}

#[cfg(feature = "octopus")]
#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct MetadataClearedEvent<T: Assets>(T::AssetId);
#[cfg(feature = "octopus")]
impl<T: Assets> Serialize for MetadataClearedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(1))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.end()
    }
}

#[cfg(feature = "octopus")]
#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct ApprovedTransferEvent<T: Assets>(T::AssetId, T::AccountId, T::AccountId, T::Balance);
#[cfg(feature = "octopus")]
impl<T: Assets> Serialize for ApprovedTransferEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(4))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry("source", &self.1)?;
        s.serialize_entry("delegate", &self.2)?;
        s.serialize_entry(AMOUNT, &self.3)?;
        s.end()
    }
}

#[cfg(feature = "octopus")]
#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct ApprovalCancelledEvent<T: Assets>(T::AssetId, T::AccountId, T::AccountId);
#[cfg(feature = "octopus")]
impl<T: Assets> Serialize for ApprovalCancelledEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(3))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry(OWNER, &self.1)?;
        s.serialize_entry("delegate", &self.2)?;
        s.end()
    }
}

#[cfg(feature = "octopus")]
#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct TransferredApprovedEvent<T: Assets>(
    T::AssetId,
    T::AccountId,
    T::AccountId,
    T::AccountId,
    T::Balance,
);
#[cfg(feature = "octopus")]
impl<T: Assets> Serialize for TransferredApprovedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(4))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry(OWNER, &self.1)?;
        s.serialize_entry("delegate", &self.2)?;
        s.serialize_entry("destination", &self.3)?;
        s.end()
    }
}

#[cfg(feature = "octopus")]
#[derive(Clone, Debug, Eq, PartialEq, Decode)]
pub struct AssetStatusChangedEvent<T: Assets>(T::AssetId);
#[cfg(feature = "octopus")]
impl<T: Assets> Serialize for AssetStatusChangedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(1))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.end()
    }
}
