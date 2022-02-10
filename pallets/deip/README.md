## Deip Pallet

The Deip pallet provides operations for basic functions of Creators Economy Protocol
This pallet exposes the following extrinsic calls:


## Project module

Project represents a digital form of intangible asset. Any project can be tokenized with NFT to boost the asset liquidity and apply specific governance and revenue flow settings.


### Create project

```rust
fn create_project(
    origin: OriginFor<T>,
    is_private: bool,
    external_id: ProjectId,
    team_id: T::DeipAccountId,
    description: T::Hash,
    domains: Vec<DomainId>
) -> DispatchResult
```


### Update project

```rust
fn update_project(
    origin: OriginFor<T>, 
    project_id: ProjectId, 
    description: Option<T::Hash>, 
    is_private: Option<bool>
) -> DispatchResult
```


### Contribute to project

```rust
fn create_project_content(
    origin: OriginFor<T>, 
    external_id: ProjectContentId,
    project_external_id: ProjectId,
    team_id: T::DeipAccountId,
    content_type: ProjectContentType,
    description: T::Hash,
    content: T::Hash,
    authors: Vec<T::DeipAccountId>,
    references: Option<Vec<ProjectContentId>>
) -> DispatchResult
```



## Review module

Projects can be peer-reviewed and curated by domain experts that help to define the value of the underlying asset. These operations are a part of the Decentralized Assessment System (DAS)

### Create review

```rust
fn create_review(
    origin: OriginFor<T>, 
    external_id: ReviewId,
    author: T::DeipAccountId,
    content: T::Hash,
    domains: Vec<DomainId>,
    assessment_model: u32,
    weight: Vec<u8>,
    project_content_external_id: ProjectContentId,
) -> DispatchResult
```

### Upvote review

```rust
fn upvote_review(
    origin: OriginFor<T>, 
    review_id: ReviewId,
    domain_id: DomainId,
) -> DispatchResult
```


## Investment opportunity module

Projects can attract investments using various funding models such as Crowdfunding in exchange of FT/NFT assets for investors.

### Create investment opportunity

```rust
fn create_investment_opportunity(
    origin: OriginFor<T>, 
    external_id: InvestmentId,
    creator: T::DeipAccountId,
    shares: Vec<DeipAssetOf<T>>,
    funding_model: FundingModelOf<T>,
) -> DispatchResult
```

### Invest using FT

```rust
fn invest(
    origin: OriginFor<T>, 
    id: InvestmentId,
    asset: DeipAssetOf<T>
) -> DispatchResult
```


## Contract agreement module

Contract agreement enables the signing of generic digital documents between N parties and verifying their authenticity and integrity later.


### Create contract agreement

```rust
fn create_contract_agreement(
    origin: OriginFor<T>, 
    id: ContractAgreementId,
    creator: T::DeipAccountId,
    parties: Vec<T::DeipAccountId>,
    hash: HashOf<T>,
    activation_time: Option<MomentOf<T>>,
    expiration_time: Option<MomentOf<T>>,
    terms: ContractAgreementTermsOf<T>,
) -> DispatchResultWithPostInfo
```


### Accept contract agreement by a party

```rust
fn accept_contract_agreement(
    origin: OriginFor<T>, 
    id: ContractAgreementId,
    party: T::DeipAccountId,
) -> DispatchResultWithPostInfo
```


### Reject contract agreement by a party

```rust
fn reject_contract_agreement(
    origin: OriginFor<T>,
    id: ContractAgreementId,
    party: T::DeipAccountId,
) -> DispatchResult
```