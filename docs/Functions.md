# Description of DEIP Pallets

DEIP blockchain runtime includes the following custom pallets to provide Creator Economy Protocol functions

## [Deip Pallet](../pallets/deip/README.md)

The Deip pallet provides operations for basic functions of Creators Economy Protocol

## [Dao Pallet](../pallets/deip_dao/README.md)

The Dao pallet provides operations for DAO (Decentralized Autonomous Organization) functionality

## [Proposal Pallet](../pallets/deip_proposal/README.md)

The Proposal pallet provides operations for creating postponed transactions, that can be interpreted as on-chain smart contracts consisting of a set of DEIP operations.

## [Portal Pallet](../pallets/deip_portal/README.md)

The Portal pallet provides operations for Dapp's (Portals) built on top of DEIP protocol to verify transactions of their users and track them by attaching additional metadata to calls.

## [Vesting Pallet](../pallets/deip_vesting/README.md)

The Vesting pallet provides operations for creating vesting contracts with a cliff vesting schedule.

## [Asset Pallet](../pallets/deip_assets/README.md)

The Asset pallet is a wrapper over substrate `pallet_assets` that contains some adapter functions for DEIP types

## [Uniques Pallet](../pallets/deip_uniques/README.md)

The Uniques pallet is a wrapper over substrate `pallet_uniques` that contains some adapter functions for DEIP types
