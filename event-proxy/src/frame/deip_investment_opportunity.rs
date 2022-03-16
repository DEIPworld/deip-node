use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};

use crate::appchain_deip::deip_investment_opportunity::events::{
    SimpleCrowdfundingActivated, SimpleCrowdfundingCreated, SimpleCrowdfundingExpired,
    SimpleCrowdfundingFinished, Invested,
};

const ACCOUNT_ID_KEY: &str = "account_id";

const INVESTMENT_ID_KEY: &str = "investment_id";

impl Serialize for SimpleCrowdfundingCreated {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("SimpleCrowdfundingCreatedEvent", 1)?;
        s.serialize_field(INVESTMENT_ID_KEY, &self.0)?;
        s.end()
    }
}

impl Serialize for SimpleCrowdfundingActivated {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("SimpleCrowdfundingActivatedEvent", 1)?;
        s.serialize_field(INVESTMENT_ID_KEY, &self.0)?;
        s.end()
    }
}

impl Serialize for SimpleCrowdfundingFinished {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("SimpleCrowdfundingFinishedEvent", 1)?;
        s.serialize_field(INVESTMENT_ID_KEY, &self.0)?;
        s.end()
    }
}

impl Serialize for SimpleCrowdfundingExpired {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("SimpleCrowdfundingExpiredEvent", 1)?;
        s.serialize_field(INVESTMENT_ID_KEY, &self.0)?;
        s.end()
    }
}

impl Serialize for Invested {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("InvestedEvent", 2)?;
        s.serialize_field(INVESTMENT_ID_KEY, &self.0)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.1)?;
        s.end()
    }
}
