## Dao Pallet

The Dao pallet provides operations for DAO (Decentralized Autonomous Organization) functionality. It allows creating DAOs with a flexible governance model that can be modified on demand.
This pallet exposes the following extrinsic calls:


### Create Dao

```rust
pub fn create(
    origin: OriginFor<T>,
    name: DaoId,
    authority: InputAuthority<T::AccountId>,
    metadata: Option<H256>,
) -> DispatchResultWithPostInfo
```


### Update Dao metadata

```rust
pub fn update_dao(
    origin: OriginFor<T>,
    new_metadata: Option<H256>,
) -> DispatchResultWithPostInfo
```


### Change Dao authority settings

```rust
pub fn alter_authority(
    origin: OriginFor<T>,
    authority: AlterAuthority<T::AccountId>,
) -> DispatchResultWithPostInfo
```


### Execute a call on behalf of specified Dao

```rust
pub fn on_behalf(
    origin: OriginFor<T>,
    name: DaoId,
    call: Box<<T as Config>::Call>,
) -> DispatchResultWithPostInfo
```