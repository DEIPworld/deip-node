
use substrate_subxt::system::System;
use substrate_subxt::{module, Event};

use sp_std::prelude::*;
use codec::{Decode};
use frame_support::{Parameter};
use sp_runtime::traits::Member;

use serde::{Serialize, ser::{Serializer, SerializeMap}};

#[module]
pub trait DeipAssets: System {
    type AssetId: Parameter + Member + Serialize;
    type Balance: Parameter + Member + Serialize;
}

const ASSET_ID: &str = "asset_id";
const OWNER: &str = "owner";
const FROM: &str = "from";
const TO: &str = "to";
const AMOUNT: &str = "amount";
const WHO: &str = "who";

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct CreatedEvent<T: DeipAssets>(T::AssetId, T::AccountId, T::AccountId);
impl<T: DeipAssets> Serialize for CreatedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_map(Some(3))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry("creator", &self.1)?;
        s.serialize_entry(OWNER, &self.2)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct IssuedEvent<T: DeipAssets>(T::AssetId, T::AccountId, T::Balance);
impl<T: DeipAssets> Serialize for IssuedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_map(Some(3))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry(OWNER, &self.1)?;
        s.serialize_entry("total_supply", &self.2)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct TransferredEvent<T: DeipAssets>(T::AssetId, T::AccountId, T::AccountId, T::Balance);
impl<T: DeipAssets> Serialize for TransferredEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_map(Some(4))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry(FROM, &self.1)?;
        s.serialize_entry(TO, &self.2)?;
        s.serialize_entry(AMOUNT, &self.3)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct BurnedEvent<T: DeipAssets>(T::AssetId, T::AccountId, T::Balance);
impl<T: DeipAssets> Serialize for BurnedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_map(Some(3))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry(OWNER, &self.1)?;
        s.serialize_entry("balance", &self.2)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct TeamChangedEvent<T: DeipAssets>(T::AssetId, T::AccountId, T::AccountId, T::AccountId);
impl<T: DeipAssets> Serialize for TeamChangedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_map(Some(4))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry("issuer", &self.1)?;
        s.serialize_entry("admin", &self.2)?;
        s.serialize_entry("freezer", &self.3)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct OwnerChangedEvent<T: DeipAssets>(T::AssetId, T::AccountId);
impl<T: DeipAssets> Serialize for OwnerChangedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_map(Some(2))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry(OWNER, &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ForceTransferredEvent<T: DeipAssets>(T::AssetId, T::AccountId, T::AccountId, T::Balance);
impl<T: DeipAssets> Serialize for ForceTransferredEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_map(Some(4))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry(FROM, &self.1)?;
        s.serialize_entry(TO, &self.2)?;
        s.serialize_entry(AMOUNT, &self.3)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct FrozenEvent<T: DeipAssets>(T::AssetId, T::AccountId);
impl<T: DeipAssets> Serialize for FrozenEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_map(Some(2))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry(WHO, &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ThawedEvent<T: DeipAssets>(T::AssetId, T::AccountId);
impl<T: DeipAssets> Serialize for ThawedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_map(Some(2))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry(WHO, &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct AssetFrozenEvent<T: DeipAssets>(T::AssetId);
impl<T: DeipAssets> Serialize for AssetFrozenEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_map(Some(1))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct AssetThawedEvent<T: DeipAssets>(T::AssetId);
impl<T: DeipAssets> Serialize for AssetThawedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_map(Some(1))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct DestroyedEvent<T: DeipAssets>(T::AssetId);
impl<T: DeipAssets> Serialize for DestroyedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_map(Some(1))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ForceCreatedEvent<T: DeipAssets>(T::AssetId, T::AccountId);
impl<T: DeipAssets> Serialize for ForceCreatedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_map(Some(2))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry(OWNER, &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct MaxZombiesChangedEvent<T: DeipAssets>(T::AssetId, u32);
impl<T: DeipAssets> Serialize for MaxZombiesChangedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_map(Some(2))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry("max_zombies", &self.1)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct MetadataSetEvent<T: DeipAssets>(T::AssetId, Vec<u8>, Vec<u8>, u8);
impl<T: DeipAssets> Serialize for MetadataSetEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_map(Some(4))?;
        s.serialize_entry(ASSET_ID, &self.0)?;
        s.serialize_entry("name", &self.1)?;
        s.serialize_entry("symbol", &self.1)?;
        s.serialize_entry("decimals", &self.1)?;
        s.end()
    }
}
