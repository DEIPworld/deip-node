use frame_support::Parameter;
use frame_system::Config;
use sp_runtime::traits::Member;

use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};

use crate::appchain_deip::deip::events::{
    ContractAgreementAccepted, ContractAgreementCreated, ContractAgreementFinalized,
    ContractAgreementRejected, DomainAdded, Invested, NdaAccessRequestCreated,
    NdaAccessRequestFulfilled, NdaAccessRequestRejected, NdaCreated, ProjectContnetCreated,
    ProjectCreated, ProjectRemoved, ProjectUpdated, ReviewCreated, ReviewUpvoted,
    SimpleCrowdfundingActivated, SimpleCrowdfundingCreated, SimpleCrowdfundingExpired,
    SimpleCrowdfundingFinished,
};

pub trait Deip: Config {
    type DomainId: Parameter + Member + Serialize;
    type ProjectId: Parameter + Member + Serialize;
    type Project: Parameter + Member + Serialize;
    type ReviewId: Parameter + Member + Serialize;
    type Review: Parameter + Member + Serialize;
    type NdaId: Parameter + Member + Serialize;
    type NdaAccessRequestId: Parameter + Member + Serialize;
    type ProjectContentId: Parameter + Member + Serialize;
    type InvestmentId: Parameter + Member + Serialize;
    type FundingModel: Parameter + Member + Serialize;
    type ContractAgreementId: Parameter + Member + Serialize;
    type ContractAgreementTerms: Parameter + Member + Serialize;
}

const ACCOUNT_ID_KEY: &str = "account_id";

impl Serialize for ProjectCreated {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ProjectCreatedEvent", 2)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.0)?;
        s.serialize_field("project", &self.1)?;
        s.end()
    }
}

impl Serialize for ProjectRemoved {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ProjectRemovedEvent", 2)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.0)?;
        s.serialize_field("project", &self.1)?;
        s.end()
    }
}

impl Serialize for ProjectUpdated {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ProjectUpdatedEvent", 2)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.0)?;
        s.serialize_field("project_id", &self.1)?;
        s.end()
    }
}

impl Serialize for ProjectContnetCreated {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ProjectContentCreatedEvent", 2)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.0)?;
        s.serialize_field("content_id", &self.1)?;
        s.end()
    }
}

impl Serialize for NdaCreated {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("NdaCreatedEvent", 2)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.0)?;
        s.serialize_field("nda_id", &self.1)?;
        s.end()
    }
}

impl Serialize for NdaAccessRequestCreated {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("NdaAccessRequestCreatedEvent", 2)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.0)?;
        s.serialize_field("access_request_id", &self.1)?;
        s.end()
    }
}

impl Serialize for NdaAccessRequestFulfilled {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("NdaAccessRequestFulfilledEvent", 2)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.0)?;
        s.serialize_field("access_request_id", &self.1)?;
        s.end()
    }
}

impl Serialize for NdaAccessRequestRejected {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("NdaAccessRequestRejectedEvent", 2)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.0)?;
        s.serialize_field("access_request_id", &self.1)?;
        s.end()
    }
}

impl Serialize for DomainAdded {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("DomainAddedEvent", 2)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.0)?;
        s.serialize_field("domain_id", &self.1)?;
        s.end()
    }
}

impl Serialize for ReviewCreated {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ReviewCreatedEvent", 2)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.0)?;
        s.serialize_field("review", &self.1)?;
        s.end()
    }
}

impl Serialize for ReviewUpvoted {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ReviewUpvotedEvent", 2)?;
        s.serialize_field("review_id", &self.0)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.1)?;
        s.serialize_field("domain_id", &self.2)?;
        s.end()
    }
}

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

const CONTRACTAGREEMENT_ID_KEY: &str = "contractagreement_id";

impl Serialize for ContractAgreementCreated {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ContractAgreementCreatedEvent", 1)?;
        s.serialize_field(CONTRACTAGREEMENT_ID_KEY, &self.0)?;
        s.end()
    }
}

impl Serialize for ContractAgreementAccepted {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ContractAgreementAcceptedEvent", 2)?;
        s.serialize_field(CONTRACTAGREEMENT_ID_KEY, &self.0)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.1)?;
        s.end()
    }
}

impl Serialize for ContractAgreementFinalized {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ContractAgreementFinalizedEvent", 1)?;
        s.serialize_field(CONTRACTAGREEMENT_ID_KEY, &self.0)?;
        s.end()
    }
}

impl Serialize for ContractAgreementRejected {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ContractAgreementRejectedEvent", 2)?;
        s.serialize_field(CONTRACTAGREEMENT_ID_KEY, &self.0)?;
        s.serialize_field(ACCOUNT_ID_KEY, &self.1)?;
        s.end()
    }
}
