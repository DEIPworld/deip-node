## Portal Pallet

The Portal pallet provides operations for Dapp's (Portals) built on top of DEIP blockchain to verify transactions of their users and track them by attaching additional metadata to extrinsic calls.
This pallet exposes the following extrinsic calls:


### Create Portal transactions verifier

```rust
pub fn create(
    origin: OriginFor<T>,
    delegate: PortalDelegate<T>,
    metadata: PortalMetadata,
) -> DispatchResultWithPostInfo
```


### Update Portal transactions verifier

```rust
pub fn update(
    origin: OriginFor<T>, 
    update: PortalUpdate<T>
) -> DispatchResultWithPostInfo
```


### Schedule a call on behalf of Portal

```rust
pub fn schedule(
    origin: OriginFor<T>,
    xt: Box<T::UncheckedExtrinsic>,
) -> DispatchResultWithPostInfo
```


### Exec a call on behalf of Portal

```rust
pub fn exec(
    origin: OriginFor<T>,
    portal_id: PortalId<T>,
    call: Box<<T as Config>::Call>,
) -> DispatchResultWithPostInfo
```