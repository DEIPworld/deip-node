#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};

#[cfg(feature = "std")]
use serde::{
	de::{self, Deserializer, Visitor},
	ser::SerializeStruct,
	Deserialize, Serialize, Serializer,
};

use sp_runtime::traits::AtLeast32BitUnsigned;

#[cfg(feature = "std")]
use sp_std::convert::TryInto;

/* This code exists just to workaround an issue in serde{, _json} with u128.
	Check the following links for details:
		- https://github.com/paritytech/substrate/issues/4641
		- https://github.com/paritytech/substrate/pull/4166 - this means that
			"arbitrary_precision" doesn't work and hence can't be used.
*/

#[derive(Encode, Decode)]
pub struct SerializableAtLeast32BitUnsigned<T: Clone + AtLeast32BitUnsigned>(pub T);

const STRUCT_NAME: &str = "u128";
const FIRST_FIELD_NAME: &str = "first_be_u64";
const SECOND_FIELD_NAME: &str = "second_be_u64";

#[cfg(feature = "std")]
impl<T: Clone + AtLeast32BitUnsigned> Serialize for SerializableAtLeast32BitUnsigned<T> {
	fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
	where
		S: Serializer,
	{
		let v: u128 = self.0.clone().unique_saturated_into();
		let be_bytes = v.to_be_bytes();
		let mut s = serializer.serialize_struct(STRUCT_NAME, 2).unwrap();
		s.serialize_field(
			FIRST_FIELD_NAME,
			&u64::from_be_bytes(be_bytes[..8].try_into().unwrap()),
		)?;
		s.serialize_field(
			SECOND_FIELD_NAME,
			&u64::from_be_bytes(be_bytes[8..].try_into().unwrap()),
		)?;
		s.end()
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

	fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
	where
		A: de::MapAccess<'de>,
	{
		let first_entry: (&str, u64) =
			map.next_entry()?.ok_or(de::Error::missing_field(FIRST_FIELD_NAME))?;
		let second_entry: (&str, u64) =
			map.next_entry()?.ok_or(de::Error::missing_field(SECOND_FIELD_NAME))?;

		let (first, second) =
			if first_entry.0 == FIRST_FIELD_NAME && second_entry.0 == SECOND_FIELD_NAME {
				(first_entry.1, second_entry.1)
			} else if first_entry.0 == SECOND_FIELD_NAME && second_entry.0 == FIRST_FIELD_NAME {
				(second_entry.1, first_entry.1)
			} else {
				return Err(de::Error::custom(format!(
					"'{}' or '{}' is unknown",
					first_entry.0, second_entry.0,
				)));
			};

		let mut u128_be_bytes = [0u8; 16];
		u128_be_bytes
			.iter_mut()
			.zip(first.to_be_bytes().iter().chain(second.to_be_bytes().iter()))
			.for_each(|v| *v.0 = *v.1);

		T::try_from(u128::from_be_bytes(u128_be_bytes))
			.map(|v| SerializableAtLeast32BitUnsigned(v))
			.map_err(|_| {
				de::Error::invalid_type(
					serde::de::Unexpected::Other("u128::try_from failed"),
					&"u128",
				)
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
		deserializer.deserialize_struct(STRUCT_NAME, &[FIRST_FIELD_NAME, SECOND_FIELD_NAME], visitor)
	}
}
