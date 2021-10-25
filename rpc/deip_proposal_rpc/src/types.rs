use codec::Decode;
use common_rpc::{Error, GetError, KeyValueInfo};
use frame_support::Parameter;
use serde::{Deserialize, Serialize};
use sp_runtime::traits::Member;
use sp_std::collections::btree_map::BTreeMap;

use pallet_deip_proposal::proposal::{BatchItem, ProposalMemberDecision, ProposalState};

pub struct ProposalIdError;
impl GetError for ProposalIdError {
    fn get_error() -> Error {
        Error::ProposalIdDecodeFailed
    }
}

#[derive(Clone, Debug, Eq, PartialEq, codec::Encode, Serialize, Deserialize)]
#[serde(bound(serialize = "common_deip_call::WrappedCall<T>: Serialize"))]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
#[serde(transparent)]
pub struct Call<T: Parameter + Member>(pub common_deip_call::WrappedCall<T>);

impl<T: Parameter + Member> Decode for Call<T> {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        T::decode(input).map(|c| Self(common_deip_call::WrappedCall(c)))
    }
}

/// copied from DeipProposal pallet since the original is generic over T: Config
#[derive(Decode, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeipProposal<AccountId: Ord, Moment, CallT> {
    pub id: super::ProposalId,
    pub batch: Vec<BatchItem<AccountId, CallT>>,
    pub decisions: BTreeMap<AccountId, ProposalMemberDecision>,
    pub state: ProposalState,
    pub author: AccountId,
    pub created_at: Moment,
}

impl<AccountId: Ord, Moment, CallT> GetError for DeipProposal<AccountId, Moment, CallT> {
    fn get_error() -> Error {
        Error::ProposalDecodeFailed
    }
}

pub struct ProposalKeyValue<AccountId, Moment, Call> {
    pub id: super::ProposalId,
    _m: std::marker::PhantomData<(AccountId, Moment, Call)>,
}

impl<AccountId, Moment, Call> ProposalKeyValue<AccountId, Moment, Call> {
    pub fn new(id: super::ProposalId) -> Self {
        Self {
            id,
            _m: Default::default(),
        }
    }
}

impl<AccountId, Moment, Call> KeyValueInfo for ProposalKeyValue<AccountId, Moment, Call>
where
    AccountId: 'static + Decode + Send + Ord,
    Moment: 'static + Decode + Send,
    Call: 'static + Decode + Send,
{
    type Key = super::ProposalId;
    type KeyError = ProposalIdError;
    type Value = DeipProposal<AccountId, Moment, Call>;
    type ValueError = DeipProposal<AccountId, Moment, Call>;

    fn key(&self) -> &Self::Key {
        &self.id
    }
}
