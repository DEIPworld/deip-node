## Vesting Pallet

The Vesting pallet provides operations for creating vesting contracts with a cliff vesting schedule.
This pallet exposes the following extrinsic calls:


### Create vesting contract

```rust
pub fn vested_transfer(
    origin: OriginFor<T>,
    target: <T::Lookup as StaticLookup>::Source,
    plan: VestingPlan<BalanceOf<T>>,
) -> DispatchResult
```


### Claim unlocked funds

```rust
pub fn unlock(
    origin: OriginFor<T>
) -> DispatchResult
```
