use codec::Decode;
use common_rpc::*;

use sp_runtime::traits::AtLeast32BitUnsigned;

// Domains

pub struct DomainIdError;
impl GetError for DomainIdError {
    fn get_error() -> Error {
        Error::DomainIdDecodeFailed
    }
}

pub struct DomainError;
impl GetError for DomainError {
    fn get_error() -> Error {
        Error::DomainDecodeFailed
    }
}

pub struct DomainKeyValue {
    pub id: super::DomainId,
}

impl DomainKeyValue {
    pub fn new(id: super::DomainId) -> Self {
        Self { id }
    }
}

impl KeyValueInfo for DomainKeyValue {
    type Key = super::DomainId;
    type KeyError = DomainIdError;
    type Value = super::Domain;
    type ValueError = DomainError;

    fn key(&self) -> &Self::Key {
        &self.id
    }
}

// Projects

pub struct ProjectIdError;
impl GetError for ProjectIdError {
    fn get_error() -> Error {
        Error::ProjectIdDecodeFailed
    }
}

pub struct ProjectError;
impl GetError for ProjectError {
    fn get_error() -> Error {
        Error::ProjectDecodeFailed
    }
}

pub struct ProjectKeyValue<Hash, AccountId> {
    pub id: super::ProjectId,
    _m: std::marker::PhantomData<(Hash, AccountId)>,
}

impl<Hash, AccountId> ProjectKeyValue<Hash, AccountId> {
    pub fn new(id: super::ProjectId) -> Self {
        Self { id, _m: Default::default() }
    }
}

impl<Hash: 'static + Decode + Send, AccountId: 'static + Decode + Send> KeyValueInfo
    for ProjectKeyValue<Hash, AccountId>
{
    type Key = super::ProjectId;
    type KeyError = ProjectIdError;
    type Value = super::Project<Hash, AccountId>;
    type ValueError = ProjectError;

    fn key(&self) -> &Self::Key {
        &self.id
    }
}

// Investment opportunities

pub struct InvestmentIdError;
impl GetError for InvestmentIdError {
    fn get_error() -> Error {
        Error::InvestmentIdDecodeFailed
    }
}

pub struct InvestmentOpportunityError;
impl GetError for InvestmentOpportunityError {
    fn get_error() -> Error {
        Error::InvestmentOpportunityDecodeFailed
    }
}

pub struct InvestmentOpportunityKeyValue<Moment, AssetId, AssetBalance, TransactionCtx> {
    pub id: super::InvestmentId,
    _m: std::marker::PhantomData<(Moment, AssetId, AssetBalance, TransactionCtx)>,
}

impl<Moment, AssetId, AssetBalance, TransactionCtx>
    InvestmentOpportunityKeyValue<Moment, AssetId, AssetBalance, TransactionCtx>
{
    #[allow(dead_code)]
    pub fn new(id: super::InvestmentId) -> Self {
        Self { id, _m: Default::default() }
    }
}

impl<Moment, AssetId, AssetBalance, TransactionCtx> KeyValueInfo
    for InvestmentOpportunityKeyValue<Moment, AssetId, AssetBalance, TransactionCtx>
where
    Moment: 'static + Decode + Send,
    AssetId: 'static + Decode + Send,
    AssetBalance: 'static + Decode + Send + Clone + AtLeast32BitUnsigned,
    TransactionCtx: 'static + Decode + Send,
{
    type Key = super::InvestmentId;
    type KeyError = InvestmentIdError;
    type Value = super::SimpleCrowdfunding<Moment, AssetId, AssetBalance, TransactionCtx>;
    type ValueError = InvestmentOpportunityError;

    fn key(&self) -> &Self::Key {
        &self.id
    }
}

// Contract agreements

pub struct AgreementIdError;
impl GetError for AgreementIdError {
    fn get_error() -> Error {
        Error::AgreementIdDecodeFailed
    }
}

pub struct AgreementError;
impl GetError for AgreementError {
    fn get_error() -> Error {
        Error::AgreementDecodeFailed
    }
}

pub struct AgreementKeyValue<AccountId, Hash, Moment> {
    pub id: super::ContractAgreementId,
    _m: std::marker::PhantomData<(AccountId, Hash, Moment)>,
}

impl<AccountId, Hash, Moment> AgreementKeyValue<AccountId, Hash, Moment> {
    pub fn new(id: super::ContractAgreementId) -> Self {
        Self { id, _m: Default::default() }
    }
}

impl<AccountId, Hash, Moment> KeyValueInfo for AgreementKeyValue<AccountId, Hash, Moment>
where
    AccountId: 'static + Decode + Send,
    Hash: 'static + Decode + Send,
    Moment: 'static + Decode + Send,
{
    type Key = super::ContractAgreementId;
    type KeyError = AgreementIdError;
    type Value = super::contract::Agreement<AccountId, Hash, Moment>;
    type ValueError = AgreementError;

    fn key(&self) -> &Self::Key {
        &self.id
    }
}

// Project contents

pub struct ProjectContentIdError;
impl GetError for ProjectContentIdError {
    fn get_error() -> Error {
        Error::ProjectContentIdDecodeFailed
    }
}

pub struct ProjectContentError;
impl GetError for ProjectContentError {
    fn get_error() -> Error {
        Error::ProjectContentDecodeFailed
    }
}

pub struct ProjectContentKeyValue<Hash, AccountId> {
    pub id: super::ProjectContentId,
    _m: std::marker::PhantomData<(Hash, AccountId)>,
}

impl<Hash, AccountId> ProjectContentKeyValue<Hash, AccountId> {
    pub fn new(id: super::ProjectContentId) -> Self {
        Self { id, _m: Default::default() }
    }
}

impl<Hash, AccountId> KeyValueInfo for ProjectContentKeyValue<Hash, AccountId>
where
    AccountId: 'static + Decode + Send,
    Hash: 'static + Decode + Send,
{
    type Key = super::ProjectContentId;
    type KeyError = ProjectContentIdError;
    type Value = super::ProjectContent<Hash, AccountId>;
    type ValueError = ProjectContentError;

    fn key(&self) -> &Self::Key {
        &self.id
    }
}

// Reviews

pub struct ReviewIdError;
impl GetError for ReviewIdError {
    fn get_error() -> Error {
        Error::ReviewIdDecodeFailed
    }
}

pub struct ReviewError;
impl GetError for ReviewError {
    fn get_error() -> Error {
        Error::ReviewDecodeFailed
    }
}

pub struct ReviewKeyValue<Hash, AccountId> {
    pub id: super::ReviewId,
    _m: std::marker::PhantomData<(Hash, AccountId)>,
}

impl<Hash, AccountId> ReviewKeyValue<Hash, AccountId> {
    pub fn new(id: super::ReviewId) -> Self {
        Self { id, _m: Default::default() }
    }
}

impl<Hash, AccountId> KeyValueInfo for ReviewKeyValue<Hash, AccountId>
where
    AccountId: 'static + Decode + Send,
    Hash: 'static + Decode + Send,
{
    type Key = super::ReviewId;
    type KeyError = ReviewIdError;
    type Value = super::Review<Hash, AccountId>;
    type ValueError = ReviewError;

    fn key(&self) -> &Self::Key {
        &self.id
    }
}

// Upvotes

pub struct UpvoteIdError;
impl GetError for UpvoteIdError {
    fn get_error() -> Error {
        Error::UpvoteIdDecodeFailed
    }
}

pub struct UpvoteError;
impl GetError for UpvoteError {
    fn get_error() -> Error {
        Error::UpvoteDecodeFailed
    }
}

pub struct UpvoteKeyValue<AccountId, Moment> {
    pub id: (super::ReviewId, AccountId, super::DomainId),
    _m: std::marker::PhantomData<(AccountId, Moment)>,
}

impl<AccountId, Moment> UpvoteKeyValue<AccountId, Moment> {
    pub fn new(id: (super::ReviewId, AccountId, super::DomainId)) -> Self {
        Self { id, _m: Default::default() }
    }
}

impl<AccountId, Moment> KeyValueInfo for UpvoteKeyValue<AccountId, Moment>
where
    AccountId: 'static + codec::Codec + Send,
    Moment: 'static + Decode + Send,
{
    type Key = (super::ReviewId, AccountId, super::DomainId);
    type KeyError = UpvoteIdError;
    type Value = super::DeipReviewVote<AccountId, Moment>;
    type ValueError = UpvoteError;

    fn key(&self) -> &Self::Key {
        &self.id
    }
}

// Ndas

pub struct NdaIdError;
impl GetError for NdaIdError {
    fn get_error() -> Error {
        Error::NdaIdDecodeFailed
    }
}

pub struct NdaError;
impl GetError for NdaError {
    fn get_error() -> Error {
        Error::NdaDecodeFailed
    }
}

pub struct NdaKeyValue<Hash, AccountId, Moment> {
    pub id: super::NdaId,
    _m: std::marker::PhantomData<(Hash, AccountId, Moment)>,
}

impl<Hash, AccountId, Moment> NdaKeyValue<Hash, AccountId, Moment> {
    pub fn new(id: super::NdaId) -> Self {
        Self { id, _m: Default::default() }
    }
}

impl<Hash, AccountId, Moment> KeyValueInfo for NdaKeyValue<Hash, AccountId, Moment>
where
    AccountId: 'static + Decode + Send,
    Moment: 'static + Decode + Send,
    Hash: 'static + Decode + Send,
{
    type Key = super::NdaId;
    type KeyError = NdaIdError;
    type Value = super::Nda<Hash, AccountId, Moment>;
    type ValueError = NdaError;

    fn key(&self) -> &Self::Key {
        &self.id
    }
}
