// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use codec::{Decode, Encode};
use frame_system::Config;
use scale_info::TypeInfo;
use sp_runtime::{traits::SignedExtension, transaction_validity::TransactionValidityError};

/// Genesis hash check to provide replay protection between different networks.
#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo)]
pub struct CheckGenesis<T: Config + Send + Sync>(std::marker::PhantomData<T>);

impl<T: Config + Send + Sync> std::fmt::Debug for CheckGenesis<T> {
    #[cfg(feature = "std")]
    fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
        write!(f, "CheckGenesis")
    }

    #[cfg(not(feature = "std"))]
    fn fmt(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}

impl<T: Config + Send + Sync> Default for CheckGenesis<T> {
    /// Creates new `SignedExtension` to check genesis hash.
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<T: Config + Send + Sync + TypeInfo> SignedExtension for CheckGenesis<T> {
    type AccountId = T::AccountId;
    type Call = <T as Config>::Call;
    type AdditionalSigned = super::Hash;
    type Pre = ();
    const IDENTIFIER: &'static str = "CheckGenesis";

    fn additional_signed(&self) -> Result<Self::AdditionalSigned, TransactionValidityError> {
        Ok(super::genesis_hash())
    }
}
