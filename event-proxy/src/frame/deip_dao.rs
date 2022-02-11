use frame_support::Parameter;
use frame_system::Config;
use sp_runtime::traits::Member;

use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};

use crate::appchain_deip::deip_dao::events::{DaoAlterAuthority, DaoCreate, DaoMetadataUpdated};

pub trait DeipDao: Config {
    type Dao: Parameter + Member + Serialize;
}

impl Serialize for DaoCreate {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("DaoCreateEvent", 1)?;
        s.serialize_field("dao", &self.0)?;
        s.end()
    }
}

impl Serialize for DaoAlterAuthority {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("DaoAlterAuthorityEvent", 1)?;
        s.serialize_field("dao", &self.0)?;
        s.end()
    }
}

impl Serialize for DaoMetadataUpdated {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("DaoMetadataUpdatedEvent", 1)?;
        s.serialize_field("dao", &self.0)?;
        s.end()
    }
}
