## Asset Pallet

The Asset pallet is a wrapper over substrate `pallet_assets` that contains some adapter functions for DEIP types.
This pallet exposes the following extrinsic calls:


### Create a new class of fungible assets from a public origin

```rust
pub fn deip_create_asset(
    origin: OriginFor<T>,
    id: DeipAssetIdOf<T>,
    admin: T::DeipAccountId,
    min_balance: AssetsBalanceOf<T>,
    project_id: Option<DeipProjectIdOf<T>>,
) -> DispatchResultWithPostInfo
```


### Destroy a class of fungible assets

```rust
pub fn deip_destroy(
    origin: OriginFor<T>,
    id: DeipAssetIdOf<T>,
    witness: pallet_assets::DestroyWitness,
) -> DispatchResultWithPostInfo
```


### Mint assets of a particular class

```rust
pub fn deip_issue_asset(
    origin: OriginFor<T>,
    id: DeipAssetIdOf<T>,
    beneficiary: T::DeipAccountId,
    #[pallet::compact] amount: AssetsBalanceOf<T>,
) -> DispatchResultWithPostInfo
```


### Decreases the asset balance of an account; called by the asset class's Admin

```rust
pub fn deip_burn(
    origin: OriginFor<T>,
    id: DeipAssetIdOf<T>,
    who: T::DeipAccountId,
    #[pallet::compact] amount: AssetsBalanceOf<T>,
) -> DispatchResultWithPostInfo
```


### Move some assets from the sender account to another

```rust
pub fn deip_transfer(
    origin: OriginFor<T>,
    id: DeipAssetIdOf<T>,
    target: T::DeipAccountId,
    #[pallet::compact] amount: AssetsBalanceOf<T>,
) -> DispatchResultWithPostInfo
```


### Disallow further unprivileged transfers from an account


```rust
pub fn deip_freeze(
    origin: OriginFor<T>,
    id: DeipAssetIdOf<T>,
    who: T::DeipAccountId,
) -> DispatchResultWithPostInfo
```


### Disallow further unprivileged transfers for the asset class

```rust
pub fn deip_freeze_asset(
    origin: OriginFor<T>,
    id: DeipAssetIdOf<T>,
) -> DispatchResultWithPostInfo
```


### Allow unprivileged transfers from an account again

```rust
pub fn deip_thaw(
    origin: OriginFor<T>,
    id: DeipAssetIdOf<T>,
    who: T::DeipAccountId,
) -> DispatchResultWithPostInfo
```


### Allow unprivileged transfers for the asset again

```rust
pub fn deip_thaw_asset(
    origin: OriginFor<T>,
    id: DeipAssetIdOf<T>,
) -> DispatchResultWithPostInfo
```


### Changes an asset class's Owner; called by the asset class's Owner

```rust
pub fn deip_transfer_ownership(
    origin: OriginFor<T>,
    id: DeipAssetIdOf<T>,
    owner: T::DeipAccountId,
) -> DispatchResultWithPostInfo
```


### Change the Issuer, Admin and Freezer of an asset

```rust
pub fn deip_set_team(
    origin: OriginFor<T>,
    id: DeipAssetIdOf<T>,
    issuer: T::DeipAccountId,
    admin: T::DeipAccountId,
    freezer: T::DeipAccountId,
) -> DispatchResultWithPostInfo
```


### Set the metadata for an asset

```rust
pub fn deip_set_metadata(
    origin: OriginFor<T>,
    id: DeipAssetIdOf<T>,
    name: Vec<u8>,
    symbol: Vec<u8>,
    decimals: u8,
) -> DispatchResultWithPostInfo
```