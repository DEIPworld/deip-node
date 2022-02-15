## Proposal Pallet

The Proposal pallet provides operations for creating postponed transactions, that can be interpreted as on-chain smart contracts consisting of a set of DEIP operations.
This pallet exposes the following extrinsic calls:


### Create Proposal

```rust
pub fn propose(
    origin: OriginFor<T>,
    batch: Vec<InputProposalBatchItem<T>>,
    external_id: Option<ProposalId>,
) -> DispatchResultWithPostInfo
```


### Decide on a proposal by involved party

```rust
pub fn decide(
    origin: OriginFor<T>,
    proposal_id: ProposalId,
    decision: ProposalMemberDecision,
    batch_weight: Weight,
) -> DispatchResultWithPostInfo
```