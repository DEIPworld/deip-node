
use substrate_subxt::system::System;
use substrate_subxt::{module, Event};

use sp_std::prelude::*;
use codec::{Decode};
use frame_support::{Parameter};
use sp_runtime::traits::Member;

use serde::{Serialize, ser::{Serializer, SerializeStruct}};

#[module]
pub trait DeipDao: System {
    type Dao: Parameter + Member + Serialize;
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct DaoCreateEvent<T: DeipDao>(T::Dao);
impl<T: DeipDao> Serialize for DaoCreateEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("DaoCreateEvent", 1)?;
        s.serialize_field("dao", &self.0)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct DaoAlterAuthorityEvent<T: DeipDao>(T::Dao);
impl<T: DeipDao> Serialize for DaoAlterAuthorityEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("DaoAlterAuthorityEvent", 1)?;
        s.serialize_field("dao", &self.0)?;
        s.end()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct DaoMetadataUpdatedEvent<T: DeipDao>(T::Dao);
impl<T: DeipDao> Serialize for DaoMetadataUpdatedEvent<T> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("DaoMetadataUpdatedEvent", 1)?;
        s.serialize_field("dao", &self.0)?;
        s.end()
    }
}
