# Stake Voting Pallet

The Stake Voting pallet provides multisignature operations based on asset ownership (stakeholders).
This pallet exposes the following extrinsic calls:

#

## Create and start new voting

Creates and starts a new voting operation for the asset holders.
This operation automatically puts a new positive vote for the call.
May be called by the asset holders only.

Arguments:
- `origin` - caller | account owner;
- `id` - unique voting identifier (32 bytes proposal id);
- `asset` - asset identifier (commonly F-NFT hash);
- `start` - starting timepoint;
- `end` - ending timepoint;
- `threshold` - absolute or relative threshold;
- `call` - encoded call data;

Dispatch events:
```Created { id: VotingId, voting: VotingOf<T> }```

> The `threshold` argument is a minimum sum of asset holders shares (fractions) for the `call` to be executed. It can take absolute value (asset balance) or value that is relative to the limit constant 100 000 000 (see runtime config). For example, if you need to use a threshold of 25% then you should set ```Relative(25 000 000)```. If you need to use a threshold of more than 50% (50% + 1) then you should set ```RelativeExcept(50 000 000)```.

```rust
pub fn create(
    origin: OriginFor<T>,
    id: VotingId,
    asset: T::AssetId,
    start: Option<Timepoint<T::BlockNumber>>,
    end: Option<Timepoint<T::BlockNumber>>,
    threshold: Threshold<T::AssetBalance>,
    call: WrapperKeepOpaque<T::Call>,
) -> DispatchResultWithPostInfo
```

#

## Vote for / against the call

Puts a new positive, neutral or negative vote into the active voting.
May be called by the asset shares holder.
This operation can fully remove the voting or execute it if the threshold is reached.

Arguments:
- `origin` - caller | account owner;
- `id` - unique voting identifier;
- `sign` - positive (upvote) | negative (downvote) | neutral (abstein);
- `max_weight` - call weight witness;

Dispatch events:
```Updated { id: VotingId, author: T::AccountId }```
```Executed { id: VotingId, voting: VotingOf<T> }```

> The `max_weight` argument is a necessary parameter for the transaction cost calculation, it compares with the call execution weight.

> The vote `sign` argument can take one of the values: `positive`, `negative` or `neutral`. So you can vote up, down, or abstein.

```rust
pub fn vote(
    origin: OriginFor<T>,
    id: VotingId,
    sign: Sign,
    max_weight: Weight,
) -> DispatchResultWithPostInfo
```

#

## Cancel voting participation (unvote)

Cancels caller's participation in the active voting and removes previously added vote.
May be called by any voter and voting author.
This operation can execute the call if the threshold is reached.

Arguments:
- `origin` - caller | account owner;
- `id` - unique voting identifier;
- `max_weight` - call weight witness;

Dispatch events:
```Updated { id: VotingId, author: T::AccountId }```
```Executed { id: VotingId, voting: VotingOf<T> }```

> The `max_weight` argument is a necessary parameter for the transaction cost calculation, it compares with the call execution weight.

```rust
pub fn unvote(
    origin: OriginFor<T>,
    id: VotingId,
    max_weight: Weight,
) -> DispatchResultWithPostInfo
```

#

## Close inactive or insolvable voting

Closes the voting and drops the call information from storage.
May be called by voting author only.
This operation fully removes the voting data and returns reserved currency to the author if there is no votes or the threshold can't be reached or the voting is out of time.

Arguments:
- `origin` - caller | account owner;
- `id` - unique voting identifier;

Dispatch events:
```Closed { id: VotingId, author: T::AccountId }```

```rust
pub fn close(
    origin: OriginFor<T>,
    id: VotingId,
) -> DispatchResultWithPostInfo
```

#

## Return control of the asset to its holder

Unlock caller's asset that has been locked by voting operations.
Should be called after closing or execution all votings for this asset.
This only applies to locking an asset through voting.

Arguments:
- `origin` - caller | account owner;
- `asset` - asset identifier (commonly F-NFT hash);

```rust
pub fn retain_asset(
    origin: OriginFor<T>,
    asset: T::AssetId,
) -> DispatchResultWithPostInfo
```
