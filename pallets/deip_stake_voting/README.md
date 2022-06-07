## Stake Voting Pallet

The Stake Voting pallet provides multisignature operations based on asset ownership (stakeholders).
This pallet exposes the following extrinsic calls:


### Create and start new voting

Creates and starts a new voting operation for the asset holders.
This operation automatically puts a new positive vote for the call.
May be called by the asset holders only.
The call can be executed instantly if the threshold is reached.

```rust
pub fn create(
    origin: OriginFor<T>,
    asset: <T as Config>::AssetId,
    start: Option<Timepoint<T::BlockNumber>>,
    end: Option<Timepoint<T::BlockNumber>>,
    threshold: Threshold<<T as Config>::AssetBalance>,
    call: WrapperKeepOpaque<<T as Config>::Call>,
) -> DispatchResultWithPostInfo
```


### Vote for/against the call

Puts a new positive, neutral or negative vote into previously created voting.
May be called by the asset holder.
This operation can fully remove the voting or execute it if the threshold is reached.

```rust
pub fn vote(
    origin: OriginFor<T>,
    voting: VotingId,
    sign: Sign,
) -> DispatchResultWithPostInfo
```

### Cancel voting participation (unvote)

Cancels caller's participation in the active voting and removes previously added vote.
May be called by any voter and voting author.
This operation can fully remove the voting or execute it if the threshold is reached.

```rust
pub fn cancel(
    origin: OriginFor<T>,
    voting: VotingId,
) -> DispatchResultWithPostInfo
```

### Return control of the asset to its holder

Unlock caller's asset that has been locked by voting operations.
Should be called after cancelling or execution all votings for this asset.
This only applies to locking an asset through voting.

```rust
pub fn retain_asset(
    origin: OriginFor<T>,
    asset: <T as Config>::AssetId,
) -> DispatchResultWithPostInfo
```