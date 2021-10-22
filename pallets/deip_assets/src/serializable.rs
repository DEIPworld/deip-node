use crate::*;
use crate::pallet::{AssetsBalanceOf, AssetsAssetIdOf};

use sp_std::marker::PhantomData;
use sp_runtime::traits::UniqueSaturatedInto;
use codec::{Encode, Decode};

#[cfg(feature = "std")]
use serde::{
    de::{Error, Visitor, Unexpected, Deserializer},
    Deserialize, Serialize,
};

pub struct AssetId<T: Config>(pub AssetsAssetIdOf<T>);

impl<T: Config> Default for AssetId<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[cfg(feature = "std")]
impl<T: Config> Serialize for AssetId<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let v = self.0.encode();
        serializer.serialize_bytes(&v)
    }
}

#[cfg(feature = "std")]
struct AssetIdVisitor<T: Config>(PhantomData<T>);

#[cfg(feature = "std")]
impl<'de, T: Config> Visitor<'de> for AssetIdVisitor<T> {
    type Value = AssetId<T>;

    fn expecting(&self, formatter: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
        formatter.write_str("bytes")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let mut mut_v = v;
        if let Ok(value) = AssetsAssetIdOf::<T>::decode(&mut mut_v) {
            return Ok(AssetId::<T>(value));
        }

        Err(Error::invalid_value(Unexpected::Bytes(v), &self))
    }
}

#[cfg(feature = "std")]
impl<'de, T: Config> Deserialize<'de> for AssetId<T> {
    fn deserialize<D>(deserializer: D) -> Result<AssetId<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = AssetIdVisitor(PhantomData);
        deserializer.deserialize_bytes(visitor)
    }
}

pub struct AssetBalance<T: Config>(pub AssetsBalanceOf<T>);

#[cfg(feature = "std")]
impl<T: Config> Serialize for AssetBalance<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let v: u128 = self.0.unique_saturated_into();
        serializer.serialize_u128(v)
    }
}

#[cfg(feature = "std")]
struct AssetBalanceVisitor<T: Config>(PhantomData<T>);

#[cfg(feature = "std")]
impl<'de, T: Config> Visitor<'de> for AssetBalanceVisitor<T> {
    type Value = AssetBalance<T>;

    fn expecting(&self, formatter: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
        formatter.write_str("an unsigned between 0 and u128::MAX")
    }

    fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match <AssetsBalanceOf<T> as sp_std::convert::TryFrom<u128>>::try_from(value) {
            Ok(value) => Ok(AssetBalance::<T>(value)),
            _ => Err(E::custom(format!("AssetBalance out of range: {}", value))),
        }
    }
}

#[cfg(feature = "std")]
impl<'de, T: Config> Deserialize<'de> for AssetBalance<T> {
    fn deserialize<D>(deserializer: D) -> Result<AssetBalance<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = AssetBalanceVisitor(PhantomData);
        deserializer.deserialize_u128(visitor)
    }
}
