## Uniques Pallet

The Uniques pallet is a wrapper over substrate `pallet_uniques` that contains some adapter functions for DEIP types.
This pallet exposes the following extrinsic calls:


### Create a new class of non-fungible assets from a public origin

```rust
pub fn deip_create(
      origin: OriginFor<T>,
      class: DeipNftClassIdOf<T>,
      admin: T::DeipAccountId,
      project_id: Option<DeipProjectIdOf<T>>,
  ) -> DispatchResultWithPostInfo
```


### Destroy a class of fungible assets

```rust
pub fn deip_destroy(
    origin: OriginFor<T>,
    class: DeipNftClassIdOf<T>,
    witness: DestroyWitness,
) -> DispatchResultWithPostInfo
```


### Mint an asset instance of a particular class

```rust
pub fn deip_mint(
    origin: OriginFor<T>,
    class: DeipNftClassIdOf<T>,
    instance: T::InstanceId,
    owner: T::DeipAccountId,
) -> DispatchResultWithPostInfo
```


### Destroy a single asset instance

```rust
pub fn deip_burn(
    origin: OriginFor<T>,
    class: DeipNftClassIdOf<T>,
    instance: T::InstanceId,
    check_owner: Option<T::DeipAccountId>,
) -> DispatchResultWithPostInfo
```


### Move an asset from the sender account to another

```rust
pub fn deip_transfer(
    origin: OriginFor<T>,
    class: DeipNftClassIdOf<T>,
    instance: T::InstanceId,
    dest: T::DeipAccountId,
) -> DispatchResultWithPostInfo
```


### Reevaluate the deposits on some assets

```rust
pub fn deip_redeposit(
    origin: OriginFor<T>,
    class: DeipNftClassIdOf<T>,
    instances: Vec<T::InstanceId>,
) -> DispatchResult
```


### Disallow further unprivileged transfer of an asset instance

```rust
pub fn deip_freeze(
    origin: OriginFor<T>,
    class: DeipNftClassIdOf<T>,
    instance: T::InstanceId,
) -> DispatchResultWithPostInfo
```


### Re-allow unprivileged transfer of an asset instance

```rust
pub fn deip_thaw(
    origin: OriginFor<T>,
    class: DeipNftClassIdOf<T>,
    instance: T::InstanceId,
) -> DispatchResultWithPostInfo
```


### Disallow further unprivileged transfers for a whole asset class

```rust
pub fn deip_freeze_class(
    origin: OriginFor<T>,
    class: DeipNftClassIdOf<T>,
) -> DispatchResultWithPostInfo
```


### Re-allow unprivileged transfers for a whole asset class

```rust
pub fn deip_thaw_class(
    origin: OriginFor<T>,
    class: DeipNftClassIdOf<T>,
) -> DispatchResultWithPostInfo 
```


### Change the Owner of an asset class

```rust
pub fn deip_transfer_ownership(
    origin: OriginFor<T>,
    class: DeipNftClassIdOf<T>,
    owner: T::DeipAccountId,
) -> DispatchResultWithPostInfo
```


### Change the Issuer, Admin and Freezer of an asset class

```rust
pub fn deip_set_team(
    origin: OriginFor<T>,
    class: DeipNftClassIdOf<T>,
    issuer: T::DeipAccountId,
    admin: T::DeipAccountId,
    freezer: T::DeipAccountId,
) -> DispatchResultWithPostInfo
```


### Approve an instance to be transferred by a delegated third-party account

```rust
pub fn deip_approve_transfer(
    origin: OriginFor<T>,
    class: DeipNftClassIdOf<T>,
    instance: T::InstanceId,
    delegate: T::DeipAccountId,
) -> DispatchResult
```


### Cancel the prior approval for the transfer of an asset by a delegate

```rust
pub fn deip_cancel_approval(
    origin: OriginFor<T>,
    class: DeipNftClassIdOf<T>,
    instance: T::InstanceId,
    maybe_check_delegate: Option<T::DeipAccountId>,
) -> DispatchResult
```


### Set an attribute for an asset class or instance

```rust
pub fn deip_set_attribute(
    origin: OriginFor<T>,
    class: DeipNftClassIdOf<T>,
    maybe_instance: Option<T::InstanceId>,
    key: BoundedVec<u8, T::KeyLimit>,
    value: BoundedVec<u8, T::ValueLimit>,
) -> DispatchResult
```


### Set an attribute for an asset class or instance

```rust
pub fn deip_clear_attribute(
    origin: OriginFor<T>,
    class: DeipNftClassIdOf<T>,
    maybe_instance: Option<T::InstanceId>,
    key: BoundedVec<u8, T::KeyLimit>,
) -> DispatchResult
```


### Set the metadata for an asset instance

```rust
pub fn deip_set_metadata(
    origin: OriginFor<T>,
    class: DeipNftClassIdOf<T>,
    instance: T::InstanceId,
    data: BoundedVec<u8, T::StringLimit>,
    is_frozen: bool,
) -> DispatchResultWithPostInfo
```


### Clear the metadata for an asset instance

```rust
pub fn deip_clear_metadata(
    origin: OriginFor<T>,
    class: DeipNftClassIdOf<T>,
    instance: T::InstanceId,
) -> DispatchResult
```


### Set the metadata for an asset class

```rust
pub fn deip_set_class_metadata(
    origin: OriginFor<T>,
    class: DeipNftClassIdOf<T>,
    data: BoundedVec<u8, T::StringLimit>,
    is_frozen: bool,
) -> DispatchResult
```


### Clear the metadata for an asset class

```rust
pub fn deip_clear_class_metadata(
    origin: OriginFor<T>,
    class: DeipNftClassIdOf<T>,
) -> DispatchResult
```