use std::fmt::Debug;

// use substrate_subxt::{RawEvent, Event, system::System};
use codec::Decode;
use frame_support::pallet_prelude::Member;
use serde::{ser::Serializer, Deserialize, Serialize};
use subxt::{
    sp_runtime::{
        generic::Block,
        traits::{Block as _Block, Header as _Header},
    },
    Config, Event,
};

use subxt::RawEvent;

use crate::{
    appchain_deip::{
        deip::events as deip_events, deip_dao::events as dao_events,
        deip_investment_opportunity::events as deip_investment_opportunity_events,
        deip_proposal::events as proposal_events, assets::events as assets_events,
    },
    frame::deip_proposal::{self, DeipProposal},
};

mod mapping;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct BlockMetadata<T: Config> {
    pub number: T::BlockNumber,
    pub hash: T::Hash,
    pub parent_hash: T::Hash,
}

impl<T> BlockMetadata<T>
where
    T: Config,
    T::Extrinsic: Member,
{
    pub fn new(block: &Block<T::Header, T::Extrinsic>) -> Self {
        Self {
            number: block.header().number().to_owned(),
            hash: block.header().hash(),
            parent_hash: block.header().parent_hash().to_owned(),
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum TypedEvent<D, I>
where
    Self: From<D> + From<I>,
{
    Domain(D),
    Infrastructure(I),
}

pub type SpecializedEvent<T> = TypedEvent<DomainEvent<T>, InfrastructureEvent<T>>;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum InfrastructureEventData<BlockCreated> {
    BlockCreated(BlockCreated),
}
#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum InfrastructureEventMeta {
    BlockCreated { domain_events: u32 },
}

pub type InfrastructureEvent<T> =
    BaseEvent<InfrastructureEventData<BlockMetadata<T>>, InfrastructureEventMeta>;

impl<T> InfrastructureEvent<T>
where
    T: Config,
    T::Extrinsic: Send + Sync,
{
    pub fn block_created(block: &Block<T::Header, T::Extrinsic>, domain_events: u32) -> Self {
        Self {
            name: "block_created".to_string(),
            data: InfrastructureEventData::BlockCreated(BlockMetadata::new(block)),
            meta: InfrastructureEventMeta::BlockCreated { domain_events },
        }
    }
}

#[derive(Serialize, Debug)]
pub struct BaseEvent<Data, Meta> {
    name: String,
    data: Data,
    meta: Meta,
}

pub type PortalId = sp_core::H160;
pub type ExtrinsicIndex = u32;

#[derive(Serialize, Debug)]
pub struct DomainEventMeta<Block> {
    index: ExtrinsicIndex,
    block: Block,
    portal_id: PortalId,
}

pub type DomainEvent<T> = BaseEvent<DomainEventData<T>, DomainEventMeta<BlockMetadata<T>>>;

impl<T: Config + DeipProposal> From<DomainEvent<T>> for SpecializedEvent<T> {
    fn from(source: DomainEvent<T>) -> Self {
        Self::Domain(source)
    }
}

impl<T: Config + DeipProposal> From<InfrastructureEvent<T>> for SpecializedEvent<T> {
    fn from(source: InfrastructureEvent<T>) -> Self {
        Self::Infrastructure(source)
    }
}

impl<T> Serialize for DomainEventData<T>
where
    T: DeipProposal,
    T::AccountId: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        pub use DomainEventData::*;
        match self {
            // =============== DeipProposal:
            ProposalProposed(e) => e.serialize(serializer),
            ProposalApproved(e) => e.serialize(serializer),
            ProposalRevokedApproval(e) => e.serialize(serializer),
            ProposalResolved(e) => e.serialize(serializer),
            ProposalExpired(e) => e.serialize(serializer),
            // =============== Deip:
            ProjectCreated(e) => e.serialize(serializer),
            ProjectRemoved(e) => e.serialize(serializer),
            ProjectUpdated(e) => e.serialize(serializer),
            ProjectContentCreated(e) => e.serialize(serializer),
            NdaCreated(e) => e.serialize(serializer),
            NdaAccessRequestCreated(e) => e.serialize(serializer),
            NdaAccessRequestFulfilled(e) => e.serialize(serializer),
            NdaAccessRequestRejected(e) => e.serialize(serializer),
            DomainAdded(e) => e.serialize(serializer),
            ReviewCreated(e) => e.serialize(serializer),
            ReviewUpvoted(e) => e.serialize(serializer),
            ContractAgreementCreated(e) => e.serialize(serializer),
            ContractAgreementAccepted(e) => e.serialize(serializer),
            ContractAgreementFinalized(e) => e.serialize(serializer),
            ContractAgreementRejected(e) => e.serialize(serializer),
            // =============== DeipInvestmentOpportunity:
            SimpleCrowdfundingCreated(e) => e.serialize(serializer),
            SimpleCrowdfundingActivated(e) => e.serialize(serializer),
            SimpleCrowdfundingFinished(e) => e.serialize(serializer),
            SimpleCrowdfundingExpired(e) => e.serialize(serializer),
            Invested(e) => e.serialize(serializer),
            // =============== DeipDao:
            DaoCreate(e) => e.serialize(serializer),
            DaoAlterAuthority(e) => e.serialize(serializer),
            DaoMetadataUpdated(e) => e.serialize(serializer),
            // =============== Assets:
            AssetClassCreated(e) => e.serialize(serializer),
            AssetIssued(e) => e.serialize(serializer),
            AssetTransferred(e) => e.serialize(serializer),
            AssetBurned(e) => e.serialize(serializer),
            AssetTeamChanged(e) => e.serialize(serializer),
            AssetOwnerChanged(e) => e.serialize(serializer),
            AssetAccountFrozen(e) => e.serialize(serializer),
            AssetAccountThawed(e) => e.serialize(serializer),
            AssetFrozen(e) => e.serialize(serializer),
            AssetThawed(e) => e.serialize(serializer),
            AssetClassDestroyed(e) => e.serialize(serializer),
            AssetClassForceCreated(e) => e.serialize(serializer),
            AssetMetadataSet(e) => e.serialize(serializer),
            #[cfg(feature = "octopus")]
            AssetMetadataCleared(e) => e.serialize(serializer),
            #[cfg(feature = "octopus")]
            AssetApprovedTransfer(e) => e.serialize(serializer),
            #[cfg(feature = "octopus")]
            AssetApprovalCancelled(e) => e.serialize(serializer),
            #[cfg(feature = "octopus")]
            AssetTransferredApproved(e) => e.serialize(serializer),
            #[cfg(feature = "octopus")]
            AssetStatusChanged(e) => e.serialize(serializer),
        }
    }
}

#[derive(Debug)]
pub enum LegacyEvent<Current, Legacy> {
    Current(Current),
    #[allow(dead_code)]
    Legacy(Legacy)
}
impl<C: Serialize, L: Serialize> Serialize for LegacyEvent<C, L> {
    fn serialize<S: Serializer>(&self, serializer: S)
        -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    {
        match self {
            Self::Current(current) => current.serialize(serializer),
            Self::Legacy(legacy) => legacy.serialize(serializer),
        }
    }
}

#[derive(Debug)]
pub enum DomainEventData<T: DeipProposal> {
    // DeipProposal:
    ProposalProposed(deip_proposal::ProposedEvent<T>),
    ProposalApproved(proposal_events::Approved),
    ProposalRevokedApproval(proposal_events::RevokedApproval),
    ProposalResolved(proposal_events::Resolved),
    ProposalExpired(proposal_events::Expired),
    // Deip:
    ProjectCreated(deip_events::ProjectCreated),
    ProjectRemoved(deip_events::ProjectRemoved),
    ProjectUpdated(deip_events::ProjectUpdated),
    ProjectContentCreated(deip_events::ProjectContnetCreated),
    NdaCreated(deip_events::NdaCreated),
    NdaAccessRequestCreated(deip_events::NdaAccessRequestCreated),
    NdaAccessRequestFulfilled(deip_events::NdaAccessRequestFulfilled),
    NdaAccessRequestRejected(deip_events::NdaAccessRequestRejected),
    DomainAdded(deip_events::DomainAdded),
    ReviewCreated(deip_events::ReviewCreated),
    ReviewUpvoted(deip_events::ReviewUpvoted),
    ContractAgreementCreated(deip_events::ContractAgreementCreated),
    ContractAgreementAccepted(deip_events::ContractAgreementAccepted),
    ContractAgreementFinalized(deip_events::ContractAgreementFinalized),
    ContractAgreementRejected(deip_events::ContractAgreementRejected),
    // DeipInvestmentOpportunity:
    SimpleCrowdfundingCreated(LegacyEvent<
        deip_investment_opportunity_events::SimpleCrowdfundingCreated,
        (),
    >),
    SimpleCrowdfundingActivated(LegacyEvent<
        deip_investment_opportunity_events::SimpleCrowdfundingActivated,
        (),
    >),
    SimpleCrowdfundingFinished(LegacyEvent<
        deip_investment_opportunity_events::SimpleCrowdfundingFinished,
        (),
    >),
    SimpleCrowdfundingExpired(LegacyEvent<
        deip_investment_opportunity_events::SimpleCrowdfundingExpired,
        (),
    >),
    Invested(LegacyEvent<
        deip_investment_opportunity_events::Invested,
        (),
    >),
    // DeipDao:
    DaoCreate(dao_events::DaoCreate),
    DaoAlterAuthority(dao_events::DaoAlterAuthority),
    DaoMetadataUpdated(dao_events::DaoMetadataUpdated),
    // Assets:
    AssetClassCreated(assets_events::Created),
    AssetIssued(assets_events::Issued),
    AssetTransferred(assets_events::Transferred),
    AssetBurned(assets_events::Burned),
    AssetTeamChanged(assets_events::TeamChanged),
    AssetOwnerChanged(assets_events::OwnerChanged),
    AssetAccountFrozen(assets_events::Frozen),
    AssetAccountThawed(assets_events::Thawed),
    AssetFrozen(assets_events::AssetFrozen),
    AssetThawed(assets_events::AssetThawed),
    AssetClassDestroyed(assets_events::Destroyed),
    AssetClassForceCreated(assets_events::ForceCreated),
    AssetMetadataSet(assets_events::MetadataSet),
    #[cfg(feature = "octopus")]
    AssetMetadataCleared(assets_events::MetadataCleared),
    #[cfg(feature = "octopus")]
    AssetApprovedTransfer(assets_events::ApprovedTransfer),
    #[cfg(feature = "octopus")]
    AssetApprovalCancelled(assets_events::ApprovalCancelled),
    #[cfg(feature = "octopus")]
    AssetTransferredApproved(assets_events::TransferredApproved),
    #[cfg(feature = "octopus")]
    AssetStatusChanged(assets_events::AssetStatusChanged),
}

pub fn known_domain_events<T>(
    raw: &(ExtrinsicIndex, RawEvent),
    block: &Block<<T as Config>::Header, T::Extrinsic>,
    portal_id: &PortalId,
) -> Result<Option<SpecializedEvent<T>>, codec::Error>
where
    T: DeipProposal + Debug + Config,
    T::Extrinsic: Member + Send + Sync,
{
    let (index, raw) = raw;
    let meta =
        DomainEventMeta { index: *index, block: BlockMetadata::new(block), portal_id: *portal_id };

    info!("Event from: {} - {}", raw.pallet.as_str(), raw.variant.as_str());
    let event = match (raw.pallet.as_str(), raw.variant.as_str()) {
        // =========== DeipProposal:
        (proposal_events::Proposed::PALLET, proposal_events::Proposed::EVENT) => DomainEvent {
            name: "proposal_proposed".to_string(),
            data: decode_event_data(raw).map(DomainEventData::ProposalProposed)?,
            meta,
        },
        (proposal_events::Approved::PALLET, proposal_events::Approved::EVENT) => DomainEvent {
            name: "proposal_approved".to_string(),
            data: decode_event_data(raw).map(DomainEventData::ProposalApproved)?,
            meta,
        },
        (proposal_events::RevokedApproval::PALLET, proposal_events::RevokedApproval::EVENT) =>
            DomainEvent {
                name: "proposal_revokedApproval".to_string(),
                data: decode_event_data(raw).map(DomainEventData::ProposalRevokedApproval)?,
                meta,
            },
        (proposal_events::Resolved::PALLET, proposal_events::Resolved::EVENT) => DomainEvent {
            name: "proposal_resolved".to_string(),
            data: decode_event_data(raw).map(DomainEventData::ProposalResolved)?,
            meta,
        },
        (proposal_events::Expired::PALLET, proposal_events::Expired::EVENT) => DomainEvent {
            name: "proposal_expired".to_string(),
            data: decode_event_data(raw).map(DomainEventData::ProposalExpired)?,
            meta,
        },
        // =========== Deip:
        (deip_events::ProjectCreated::PALLET, deip_events::ProjectCreated::EVENT) => DomainEvent {
            name: "project_created".to_string(),
            data: decode_event_data(raw).map(DomainEventData::ProjectCreated)?,
            meta,
        },
        (deip_events::ProjectRemoved::PALLET, deip_events::ProjectRemoved::EVENT) => DomainEvent {
            name: "project_removed".to_string(),
            data: decode_event_data(raw).map(DomainEventData::ProjectRemoved)?,
            meta,
        },
        (deip_events::ProjectUpdated::PALLET, deip_events::ProjectUpdated::EVENT) => DomainEvent {
            name: "project_updated".to_string(),
            data: decode_event_data(raw).map(DomainEventData::ProjectUpdated)?,
            meta,
        },
        (deip_events::ProjectContnetCreated::PALLET, deip_events::ProjectContnetCreated::EVENT) =>
            DomainEvent {
                name: "project_contentCreated".to_string(),
                data: decode_event_data(raw).map(DomainEventData::ProjectContentCreated)?,
                meta,
            },
        (deip_events::NdaCreated::PALLET, deip_events::NdaCreated::EVENT) => DomainEvent {
            name: "project_ndaCreated".to_string(),
            data: decode_event_data(raw).map(DomainEventData::NdaCreated)?,
            meta,
        },
        (
            deip_events::NdaAccessRequestCreated::PALLET,
            deip_events::NdaAccessRequestCreated::EVENT,
        ) => DomainEvent {
            name: "project_ndaAccessRequestCreated".to_string(),
            data: decode_event_data(raw).map(DomainEventData::NdaAccessRequestCreated)?,
            meta,
        },
        (
            deip_events::NdaAccessRequestFulfilled::PALLET,
            deip_events::NdaAccessRequestFulfilled::EVENT,
        ) => DomainEvent {
            name: "project_ndaAccessRequestFulfilled".to_string(),
            data: decode_event_data(raw).map(DomainEventData::NdaAccessRequestFulfilled)?,
            meta,
        },
        (
            deip_events::NdaAccessRequestRejected::PALLET,
            deip_events::NdaAccessRequestRejected::EVENT,
        ) => DomainEvent {
            name: "project_ndaAccessRequestRejected".to_string(),
            data: decode_event_data(raw).map(DomainEventData::NdaAccessRequestRejected)?,
            meta,
        },
        (deip_events::DomainAdded::PALLET, deip_events::DomainAdded::EVENT) => DomainEvent {
            name: "project_domainAdded".to_string(),
            data: decode_event_data(raw).map(DomainEventData::DomainAdded)?,
            meta,
        },
        (deip_events::ReviewCreated::PALLET, deip_events::ReviewCreated::EVENT) => DomainEvent {
            name: "project_reviewCreated".to_string(),
            data: decode_event_data(raw).map(DomainEventData::ReviewCreated)?,
            meta,
        },
        (deip_events::ReviewUpvoted::PALLET, deip_events::ReviewUpvoted::EVENT) => DomainEvent {
            name: "project_reviewUpvoted".to_string(),
            data: decode_event_data(raw).map(DomainEventData::ReviewUpvoted)?,
            meta,
        },
        (
            deip_events::ContractAgreementCreated::PALLET,
            deip_events::ContractAgreementCreated::EVENT,
        ) => DomainEvent {
            name: "deip_contractAgreementCreated".to_string(),
            data: decode_event_data(raw).map(DomainEventData::ContractAgreementCreated)?,
            meta,
        },
        (
            deip_events::ContractAgreementAccepted::PALLET,
            deip_events::ContractAgreementAccepted::EVENT,
        ) => DomainEvent {
            name: "deip_contractAgreementAccepted".to_string(),
            data: decode_event_data(raw).map(DomainEventData::ContractAgreementAccepted)?,
            meta,
        },
        (
            deip_events::ContractAgreementFinalized::PALLET,
            deip_events::ContractAgreementFinalized::EVENT,
        ) => DomainEvent {
            name: "deip_contractAgreementFinalized".to_string(),
            data: decode_event_data(raw).map(DomainEventData::ContractAgreementFinalized)?,
            meta,
        },
        (
            deip_events::ContractAgreementRejected::PALLET,
            deip_events::ContractAgreementRejected::EVENT,
        ) => DomainEvent {
            name: "deip_contractAgreementRejected".to_string(),
            data: decode_event_data(raw).map(DomainEventData::ContractAgreementRejected)?,
            meta,
        },
        // =========== DeipInvestmentOpportunity:
        (
            deip_investment_opportunity_events::SimpleCrowdfundingCreated::PALLET,
            deip_investment_opportunity_events::SimpleCrowdfundingCreated::EVENT,
        ) => DomainEvent {
            name: "project_tokenSaleCreated".to_string(),
            data: DomainEventData::SimpleCrowdfundingCreated(
                decode_event_data(raw).map(LegacyEvent::Current)?
            ),
            meta,
        },
        (
            deip_investment_opportunity_events::SimpleCrowdfundingActivated::PALLET,
            deip_investment_opportunity_events::SimpleCrowdfundingActivated::EVENT,
        ) => DomainEvent {
            name: "project_tokenSaleActivated".to_string(),
            data: DomainEventData::SimpleCrowdfundingActivated(
                decode_event_data(raw).map(LegacyEvent::Current)?
            ),
            meta,
        },
        (
            deip_investment_opportunity_events::SimpleCrowdfundingFinished::PALLET,
            deip_investment_opportunity_events::SimpleCrowdfundingFinished::EVENT,
        ) => DomainEvent {
            name: "project_tokenSaleFinished".to_string(),
            data: DomainEventData::SimpleCrowdfundingFinished(
                decode_event_data(raw).map(LegacyEvent::Current)?
            ),
            meta,
        },
        (
            deip_investment_opportunity_events::SimpleCrowdfundingExpired::PALLET,
            deip_investment_opportunity_events::SimpleCrowdfundingExpired::EVENT,
        ) => DomainEvent {
            name: "project_tokenSaleExpired".to_string(),
            data: DomainEventData::SimpleCrowdfundingExpired(
                decode_event_data(raw).map(LegacyEvent::Current)?
            ),
            meta,
        },
        (
            deip_investment_opportunity_events::Invested::PALLET,
            deip_investment_opportunity_events::Invested::EVENT
        ) => DomainEvent {
            name: "project_tokenSaleContributed".to_string(),
            data: DomainEventData::Invested(decode_event_data(raw).map(LegacyEvent::Current)?),
            meta,
        },
        // =========== DeipDao:
        (dao_events::DaoCreate::PALLET, dao_events::DaoCreate::EVENT) => DomainEvent {
            name: "dao_create".to_string(),
            data: decode_event_data(raw).map(DomainEventData::DaoCreate)?,
            meta,
        },
        (dao_events::DaoAlterAuthority::PALLET, dao_events::DaoAlterAuthority::EVENT) =>
            DomainEvent {
                name: "dao_alterAuthority".to_string(),
                data: decode_event_data(raw).map(DomainEventData::DaoAlterAuthority)?,
                meta,
            },
        (dao_events::DaoMetadataUpdated::PALLET, dao_events::DaoMetadataUpdated::EVENT) =>
            DomainEvent {
                name: "dao_metadataUpdated".to_string(),
                data: decode_event_data(raw).map(DomainEventData::DaoMetadataUpdated)?,
                meta,
            },
        // =========== Assets:
        (assets_events::Created::PALLET, assets_events::Created::EVENT) => DomainEvent {
            name: "asset_class_created".to_string(),
            data: decode_event_data(raw).map(DomainEventData::AssetClassCreated)?,
            meta,
        },
        (assets_events::Issued::PALLET, assets_events::Issued::EVENT) => DomainEvent {
            name: "asset_issued".to_string(),
            data: decode_event_data(raw).map(DomainEventData::AssetIssued)?,
            meta,
        },
        (assets_events::Transferred::PALLET, assets_events::Transferred::EVENT) => DomainEvent {
            name: "asset_transferred".to_string(),
            data: decode_event_data(raw).map(DomainEventData::AssetTransferred)?,
            meta,
        },
        (assets_events::Burned::PALLET, assets_events::Burned::EVENT) => DomainEvent {
            name: "asset_burned".to_string(),
            data: decode_event_data(raw).map(DomainEventData::AssetBurned)?,
            meta,
        },
        (assets_events::TeamChanged::PALLET, assets_events::TeamChanged::EVENT) => DomainEvent {
            name: "asset_team_changed".to_string(),
            data: decode_event_data(raw).map(DomainEventData::AssetTeamChanged)?,
            meta,
        },
        (assets_events::OwnerChanged::PALLET, assets_events::OwnerChanged::EVENT) => DomainEvent {
            name: "asset_owner_changed".to_string(),
            data: decode_event_data(raw).map(DomainEventData::AssetOwnerChanged)?,
            meta,
        },
        (assets_events::Frozen::PALLET, assets_events::Frozen::EVENT) => DomainEvent {
            name: "asset_account_frozen".to_string(),
            data: decode_event_data(raw).map(DomainEventData::AssetAccountFrozen)?,
            meta,
        },
        (assets_events::Thawed::PALLET, assets_events::Thawed::EVENT) => DomainEvent {
            name: "asset_account_thawed".to_string(),
            data: decode_event_data(raw).map(DomainEventData::AssetAccountThawed)?,
            meta,
        },
        (assets_events::AssetFrozen::PALLET, assets_events::AssetFrozen::EVENT) => DomainEvent {
            name: "asset_frozen".to_string(),
            data: decode_event_data(raw).map(DomainEventData::AssetFrozen)?,
            meta,
        },
        (assets_events::AssetThawed::PALLET, assets_events::AssetThawed::EVENT) => DomainEvent {
            name: "asset_thawed".to_string(),
            data: decode_event_data(raw).map(DomainEventData::AssetThawed)?,
            meta,
        },
        (assets_events::Destroyed::PALLET, assets_events::Destroyed::EVENT) => DomainEvent {
            name: "asset_class_destroyed".to_string(),
            data: decode_event_data(raw).map(DomainEventData::AssetClassDestroyed)?,
            meta,
        },
        (assets_events::ForceCreated::PALLET, assets_events::ForceCreated::EVENT) => DomainEvent {
            name: "asset_class_force_created".to_string(),
            data: decode_event_data(raw).map(DomainEventData::AssetClassForceCreated)?,
            meta,
        },
        (assets_events::MetadataSet::PALLET, assets_events::MetadataSet::EVENT) => DomainEvent {
            name: "asset_metadata_set".to_string(),
            data: decode_event_data(raw).map(DomainEventData::AssetMetadataSet)?,
            meta,
        },
        #[cfg(feature = "octopus")]
        (assets_events::MetadataCleared::PALLET, assets_events::MetadataCleared::EVENT) =>
            DomainEvent {
                name: "asset_metadata_cleared".to_string(),
                data: decode_event_data(raw).map(DomainEventData::AssetMetadataCleared)?,
                meta,
            },
        #[cfg(feature = "octopus")]
        (assets_events::ApprovedTransfer::PALLET, assets_events::ApprovedTransfer::EVENT) =>
            DomainEvent {
                name: "asset_approved_transfer".to_string(),
                data: decode_event_data(raw).map(DomainEventData::AssetApprovedTransfer)?,
                meta,
            },
        #[cfg(feature = "octopus")]
        (assets_events::ApprovalCancelled::PALLET, assets_events::ApprovalCancelled::EVENT) =>
            DomainEvent {
                name: "asset_approval_cancelled".to_string(),
                data: decode_event_data(raw).map(DomainEventData::AssetApprovalCancelled)?,
                meta,
            },
        #[cfg(feature = "octopus")]
        (assets_events::TransferredApproved::PALLET, assets_events::TransferredApproved::EVENT) =>
            DomainEvent {
                name: "asset_transferred_approved".to_string(),
                data: decode_event_data(raw).map(DomainEventData::AssetTransferredApproved)?,
                meta,
            },
        #[cfg(feature = "octopus")]
        (assets_events::AssetStatusChanged::PALLET, assets_events::AssetStatusChanged::EVENT) =>
            DomainEvent {
                name: "asset_status_changed".to_string(),
                data: decode_event_data(raw).map(DomainEventData::AssetStatusChanged)?,
                meta,
            },
        _ => return Ok(None),
    };
    info!("Event decoded: {}", event.name);
    Ok(Some(event.into()))
}

fn decode_event_data<T: Decode>(e: &RawEvent) -> Result<T, codec::Error> {
    T::decode(&mut &e.data[..])
}
