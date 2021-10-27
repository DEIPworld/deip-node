#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};

#[cfg(feature = "std")]
use serde::{
	de::{self, Deserializer, Visitor},
	Deserialize, Serialize, Serializer,
};

use sp_runtime::{traits::AtLeast32BitUnsigned, RuntimeDebug};

/* This code exists just to workaround an issue in serde{, _json} with u128.
	Check the following links for details:
		- https://github.com/paritytech/substrate/issues/4641
		- https://github.com/paritytech/substrate/pull/4166 - this means that
			"arbitrary_precision" doesn't work and hence can't be used.
*/

#[derive(Encode, Decode, Default, Clone, RuntimeDebug, PartialEq, Eq)]
pub struct SerializableAtLeast32BitUnsigned<T: Clone + AtLeast32BitUnsigned>(pub T);

#[cfg(feature = "std")]
impl<T: Clone + AtLeast32BitUnsigned> Serialize for SerializableAtLeast32BitUnsigned<T> {
	fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
	where
		S: Serializer,
	{
		let v: u128 = self.0.clone().unique_saturated_into();
		serializer.serialize_str(&v.to_string())
	}
}

#[cfg(feature = "std")]
struct SerializableAtLeast32BitUnsignedVisitor<T>(sp_std::marker::PhantomData<T>);

#[cfg(feature = "std")]
impl<'de, T: Clone + AtLeast32BitUnsigned> Visitor<'de>
	for SerializableAtLeast32BitUnsignedVisitor<T>
{
	type Value = SerializableAtLeast32BitUnsigned<T>;

	fn expecting(&self, formatter: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		formatter.write_str("an unsigned between 0 and u128::MAX")
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: de::Error,
	{
		let number = u128::from_str_radix(v, 10).map_err(|_| {
			de::Error::invalid_type(
				serde::de::Unexpected::Other("u128::from_str_radix failed"),
				&"u128",
			)
		})?;

		T::try_from(number).map(|v| SerializableAtLeast32BitUnsigned(v)).map_err(|_| {
			de::Error::invalid_type(serde::de::Unexpected::Other("T::try_from failed"), &"u128")
		})
	}
}

#[cfg(feature = "std")]
impl<'de, T: Clone + AtLeast32BitUnsigned> Deserialize<'de>
	for SerializableAtLeast32BitUnsigned<T>
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let visitor = SerializableAtLeast32BitUnsignedVisitor(Default::default());
		deserializer.deserialize_str(visitor)
	}
}
