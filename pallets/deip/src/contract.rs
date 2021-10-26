use crate::traits::DeipAssetSystem;
use crate::*;

use sp_runtime::{traits::Zero, Percent, SaturatedConversion};
use sp_std::vec;

pub type Id = H160;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum Terms<Asset> {
    LicenseAgreement { source: ProjectId, price: Asset },
    GeneralContractAgreement,
}

pub type TermsOf<T> = Terms<DeipAssetOf<T>>;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum IndexTerms {
    LicenseAgreement,
    GeneralContractAgreement,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum Agreement<AccountId, Hash, Moment, Asset> {
    None,
    License(LicenseStatus<AccountId, Hash, Moment, Asset>),
    GeneralContract(GeneralContractStatus<AccountId, Hash, Moment>),
}

pub type AgreementOf<T> = Agreement<AccountIdOf<T>, HashOf<T>, MomentOf<T>, DeipAssetOf<T>>;

impl<AccountId, Hash, Moment, Asset> Default for Agreement<AccountId, Hash, Moment, Asset> {
    fn default() -> Self {
        Agreement::None
    }
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct License<AccountId, Hash, Moment, Asset> {
    id: Id,
    creator: AccountId,
    licenser: AccountId,
    licensee: AccountId,
    hash: Hash,
    activation_time: Option<Moment>,
    expiration_time: Option<Moment>,
    project_id: ProjectId,
    price: Asset,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum LicenseStatus<AccountId, Hash, Moment, Asset> {
    Unsigned(License<AccountId, Hash, Moment, Asset>),
    SignedByLicenser(License<AccountId, Hash, Moment, Asset>),
    Signed(License<AccountId, Hash, Moment, Asset>),
    Rejected(License<AccountId, Hash, Moment, Asset>),
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct GeneralContract<AccountId, Hash, Moment> {
    id: Id,
    creator: AccountId,
    parties: Vec<AccountId>,
    hash: Hash,
    activation_time: Option<Moment>,
    expiration_time: Option<Moment>,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum GeneralContractStatus<AccountId, Hash, Moment> {
    PartiallyAccepted {
        contract: GeneralContract<AccountId, Hash, Moment>,
        accepted_by: Vec<AccountId>,
    },
    Accepted(GeneralContract<AccountId, Hash, Moment>),
    Rejected(GeneralContract<AccountId, Hash, Moment>),
}

impl<T: Config> Module<T> {
    pub(super) fn create_contract_agreement_impl(
        account: AccountIdOf<T>,
        id: Id,
        creator: AccountIdOf<T>,
        parties: Vec<T::DeipAccountId>,
        hash: HashOf<T>,
        activation_time: Option<MomentOf<T>>,
        expiration_time: Option<MomentOf<T>>,
        terms: TermsOf<T>,
    ) -> DispatchResult {
        ensure!(account == creator, Error::<T>::NoPermission);
        ensure!(!parties.is_empty(), Error::<T>::ContractAgreementNoParties);

        for (i, party) in parties.iter().enumerate() {
            for other_party in parties.iter().skip(i + 1) {
                ensure!(
                    party != other_party,
                    Error::<T>::ContractAgreementDuplicateParties
                );
            }
        }

        let now = pallet_timestamp::Pallet::<T>::get();
        if let Some(s) = activation_time {
            ensure!(
                now <= s,
                Error::<T>::ContractAgreementStartTimeMustBeLaterOrEqualCurrentMoment
            );
        }

        if let Some(e) = expiration_time {
            let activation_time = match activation_time {
                None => now,
                Some(s) => s,
            };

            ensure!(
                activation_time < e,
                Error::<T>::ContractAgreementEndTimeMustBeLaterStartTime
            );
        }

        ensure!(
            !ContractAgreementMap::<T>::contains_key(id),
            Error::<T>::ContractAgreementAlreadyExists
        );
        match terms {
            Terms::LicenseAgreement { source, price } => Self::create_project_license(
                id, creator, parties, hash, activation_time, expiration_time, source, price,
            ),
            Terms::GeneralContractAgreement => {
                Self::create_general_contract(id, creator, parties, hash, activation_time, expiration_time)
            }
        }
    }

    fn create_project_license(
        id: Id,
        creator: AccountIdOf<T>,
        mut parties: Vec<T::DeipAccountId>,
        hash: HashOf<T>,
        activation_time: Option<MomentOf<T>>,
        expiration_time: Option<MomentOf<T>>,
        project_id: ProjectId,
        price: DeipAssetOf<T>,
    ) -> DispatchResult {
        ensure!(
            price.amount() > &Zero::zero(),
            Error::<T>::ContractAgreementFeeMustBePositive
        );

        ensure!(
            parties.len() == 2,
            Error::<T>::ContractAgreementLicenseTwoPartiesRequired
        );

        let project =
            ProjectMap::<T>::try_get(project_id).map_err(|_| Error::<T>::NoSuchProject)?;

        let second: AccountIdOf<T> = parties.pop().unwrap().into();
        let first: AccountIdOf<T> = parties.pop().unwrap().into();
        let (licenser, licensee) = if first == project.team_id {
            (first, second)
        } else if second == project.team_id {
            (second, first)
        } else {
            return Err(Error::<T>::ContractAgreementLicenseProjectTeamIsNotListedInParties.into());
        };

        let license = License {
            id,
            creator,
            licenser,
            licensee,
            hash,
            activation_time,
            expiration_time,
            project_id,
            price,
        };

        ContractAgreementMap::<T>::insert(id, Agreement::License(LicenseStatus::Unsigned(license)));
        ContractAgreementIdByType::insert(IndexTerms::LicenseAgreement, id, ());

        Self::deposit_event(RawEvent::ContractAgreementCreated(id));

        Ok(())
    }

    pub(super) fn accept_contract_agreement_impl(
        account: AccountIdOf<T>,
        id: Id,
        party: AccountIdOf<T>,
    ) -> DispatchResult {
        ensure!(account == party, Error::<T>::NoPermission);

        let agreement = ContractAgreementMap::<T>::try_get(id)
            .map_err(|_| Error::<T>::ContractAgreementNotFound)?;

        match agreement {
            Agreement::License(status) => Self::accept_project_license(party, status),
            Agreement::GeneralContract(status) => Self::accept_general_contract(party, status),
            Agreement::None => Err(Error::<T>::ContractAgreementWrongAgreement.into()),
        }
    }

    pub(super) fn reject_contract_agreement_impl(
        account: AccountIdOf<T>,
        id: Id,
        party: AccountIdOf<T>,
    ) -> DispatchResult {
        ensure!(account == party, Error::<T>::NoPermission);

        let agreement = ContractAgreementMap::<T>::try_get(id)
            .map_err(|_| Error::<T>::ContractAgreementNotFound)?;

        match agreement {
            Agreement::None => Err(Error::<T>::ContractAgreementWrongAgreement.into()),
            Agreement::License(status) => Self::reject_license(party, status),
            Agreement::GeneralContract(status) => Self::reject_general_contract(party, status),
        }
    }

    fn accept_project_license(
        party: AccountIdOf<T>,
        status: LicenseStatus<AccountIdOf<T>, HashOf<T>, MomentOf<T>, DeipAssetOf<T>>,
    ) -> DispatchResult {
        match status {
            LicenseStatus::Unsigned(license) => {
                Self::accept_project_license_by_licenser(party, license)
            }
            LicenseStatus::SignedByLicenser(license) => {
                Self::accept_project_license_by_licensee(party, license)
            }
            LicenseStatus::Signed(_) => Err(Error::<T>::ContractAgreementAlreadyAccepted.into()),
            LicenseStatus::Rejected(_) => Err(Error::<T>::ContractAgreementRejected.into()),
        }
    }

    fn accept_project_license_by_licenser(
        licenser: AccountIdOf<T>,
        license: License<AccountIdOf<T>, HashOf<T>, MomentOf<T>, DeipAssetOf<T>>,
    ) -> DispatchResult {
        ensure!(
            licenser == license.licenser,
            Error::<T>::ContractAgreementLicensePartyIsNotLicenser
        );

        let now = pallet_timestamp::Pallet::<T>::get();
        ensure!(
            license.activation_time.unwrap_or(now) <= now,
            Error::<T>::ContractAgreementLicenseIsNotActive
        );
        ensure!(
            now <= license.expiration_time.unwrap_or(now),
            Error::<T>::ContractAgreementLicenseExpired
        );

        let id = license.id;
        let status = LicenseStatus::SignedByLicenser(license);
        ContractAgreementMap::<T>::insert(id, Agreement::License(status));

        Self::deposit_event(RawEvent::ContractAgreementAccepted(id, licenser));

        Ok(())
    }

    fn accept_project_license_by_licensee(
        licensee: AccountIdOf<T>,
        license: License<AccountIdOf<T>, HashOf<T>, MomentOf<T>, DeipAssetOf<T>>,
    ) -> DispatchResult {
        ensure!(
            licensee == license.licensee,
            Error::<T>::ContractAgreementLicensePartyIsNotLicensee
        );

        let now = pallet_timestamp::Pallet::<T>::get();
        ensure!(
            license.activation_time.unwrap_or(now) <= now,
            Error::<T>::ContractAgreementLicenseIsNotActive
        );
        ensure!(
            now <= license.expiration_time.unwrap_or(now),
            Error::<T>::ContractAgreementLicenseExpired
        );

        // this percent should be specified in the corresponding revenue stream
        let distribute_percent = Percent::from_percent(100);
        Self::distribute_revenue(
            &licensee,
            license.price.id(),
            license.price.amount(),
            distribute_percent,
            &license.project_id,
        )?;

        let id = license.id;
        let status = LicenseStatus::Signed(license);
        ContractAgreementMap::<T>::insert(id, Agreement::License(status));

        Self::deposit_event(RawEvent::ContractAgreementAccepted(id, licensee));
        Self::deposit_event(RawEvent::ContractAgreementFinalized(id));

        Ok(())
    }

    fn distribute_revenue(
        from: &AccountIdOf<T>,
        asset: &DeipAssetIdOf<T>,
        fee: &DeipAssetBalanceOf<T>,
        distribute_percent: Percent,
        project_id: &ProjectId,
    ) -> DispatchResult {
        ensure!(
            T::AssetSystem::account_balance(&from, &asset) >= *fee,
            Error::<T>::ContractAgreementLicenseNotEnoughBalance
        );

        let fee_to_distribute = distribute_percent.mul_floor(*fee);

        let mut total_revenue: DeipAssetBalanceOf<T> = Zero::zero();
        let mut transfer_info = vec![];
        let beneficiary_tokens = T::AssetSystem::get_project_nfts(project_id);
        // simple model is used: if there are several (F-)NFT classes then
        // the whole amount is distributed uniformly among the classes
        let token_count: u128 = beneficiary_tokens.len().saturated_into();
        for token in &beneficiary_tokens {
            let token_supply: u128 = T::AssetSystem::total_supply(token).saturated_into();
            let token_balances = if let Some(balances) = T::AssetSystem::get_nft_balances(token) {
                balances
            } else {
                continue;
            };

            for token_balance in &token_balances {
                let balance = T::AssetSystem::account_balance(&token_balance, token);
                let revenue: u128 = (fee_to_distribute * balance).saturated_into();
                let revenue: DeipAssetBalanceOf<T> =
                    (revenue / (token_supply * token_count)).saturated_into();
                if revenue.is_zero() {
                    continue;
                }

                transfer_info.push((revenue, token_balance.clone()));

                total_revenue += revenue;
            }
        }

        if total_revenue < *fee {
            // transfer the rest to the project team
            let project = ProjectMap::<T>::get(*project_id);
            transfer_info.push((*fee - total_revenue, project.team_id.clone()));
        }

        ensure!(
            T::AssetSystem::transactionally_transfer(from, *asset, &transfer_info).is_ok(),
            Error::<T>::ContractAgreementLicenseFailedToChargeFee
        );

        Ok(())
    }

    fn create_general_contract(
        id: Id,
        creator: AccountIdOf<T>,
        parties: Vec<T::DeipAccountId>,
        hash: HashOf<T>,
        activation_time: Option<MomentOf<T>>,
        expiration_time: Option<MomentOf<T>>,
    ) -> DispatchResult {
        let contract = GeneralContract {
            id,
            creator,
            parties: parties.into_iter().map(|p| p.into()).collect(),
            hash,
            activation_time,
            expiration_time,
        };

        ContractAgreementMap::<T>::insert(
            id,
            Agreement::GeneralContract(GeneralContractStatus::PartiallyAccepted {
                contract,
                accepted_by: vec![],
            }),
        );
        ContractAgreementIdByType::insert(IndexTerms::GeneralContractAgreement, id, ());

        Self::deposit_event(RawEvent::ContractAgreementCreated(id));

        Ok(())
    }

    fn accept_general_contract(
        party: AccountIdOf<T>,
        status: GeneralContractStatus<AccountIdOf<T>, HashOf<T>, MomentOf<T>>,
    ) -> DispatchResult {
        match status {
            GeneralContractStatus::Rejected(_) => Err(Error::<T>::ContractAgreementRejected.into()),
            GeneralContractStatus::Accepted(_) => {
                Err(Error::<T>::ContractAgreementAlreadyAccepted.into())
            }
            GeneralContractStatus::PartiallyAccepted {
                contract,
                accepted_by,
            } => Self::accept_general_contract_impl(party, contract, accepted_by),
        }
    }

    fn accept_general_contract_impl(
        party: AccountIdOf<T>,
        contract: GeneralContract<AccountIdOf<T>, HashOf<T>, MomentOf<T>>,
        mut accepted_by: Vec<AccountIdOf<T>>,
    ) -> DispatchResult {
        ensure!(
            !accepted_by.contains(&party),
            Error::<T>::ContractAgreementAlreadyAcceptedByParty
        );

        ensure!(
            contract.parties.contains(&party),
            Error::<T>::ContractAgreementPartyIsNotListed
        );

        accepted_by.push(party.clone());
        let id = contract.id;
        if accepted_by.len() == contract.parties.len() {
            ContractAgreementMap::<T>::insert(
                id,
                Agreement::GeneralContract(GeneralContractStatus::Accepted(contract)),
            );

            Self::deposit_event(RawEvent::ContractAgreementAccepted(id, party));
            Self::deposit_event(RawEvent::ContractAgreementFinalized(id));
        } else {
            ContractAgreementMap::<T>::insert(
                id,
                Agreement::GeneralContract(GeneralContractStatus::PartiallyAccepted {
                    contract,
                    accepted_by,
                }),
            );

            Self::deposit_event(RawEvent::ContractAgreementAccepted(id, party));
        }

        Ok(())
    }

    fn reject_general_contract(
        party: AccountIdOf<T>,
        status: GeneralContractStatus<AccountIdOf<T>, HashOf<T>, MomentOf<T>>,
    ) -> DispatchResult {
        match status {
            GeneralContractStatus::Rejected(_) => Err(Error::<T>::ContractAgreementRejected.into()),
            GeneralContractStatus::Accepted(_) => {
                Err(Error::<T>::ContractAgreementAlreadyAccepted.into())
            }

            GeneralContractStatus::PartiallyAccepted {
                contract,
                accepted_by,
            } => {
                ensure!(
                    !accepted_by.contains(&party),
                    Error::<T>::ContractAgreementAlreadyAcceptedByParty
                );

                ensure!(
                    contract.parties.contains(&party),
                    Error::<T>::ContractAgreementPartyIsNotListed
                );

                let id = contract.id;
                ContractAgreementMap::<T>::insert(
                    id,
                    Agreement::GeneralContract(GeneralContractStatus::Rejected(contract)),
                );

                Self::deposit_event(RawEvent::ContractAgreementRejected(id, party));

                Ok(())
            }
        }
    }

    fn reject_license(
        party: AccountIdOf<T>,
        status: LicenseStatus<AccountIdOf<T>, HashOf<T>, MomentOf<T>, DeipAssetOf<T>>,
    ) -> DispatchResult {
        match status {
            LicenseStatus::Rejected(_) => Err(Error::<T>::ContractAgreementRejected.into()),
            LicenseStatus::Signed(_) => Err(Error::<T>::ContractAgreementAlreadyAccepted.into()),

            LicenseStatus::SignedByLicenser(license) => {
                ensure!(
                    party == license.licensee,
                    Error::<T>::ContractAgreementLicensePartyIsNotLicensee
                );

                Self::reject_license_common(party, license)
            }

            LicenseStatus::Unsigned(license) => {
                ensure!(
                    party == license.licensee || party == license.licenser,
                    Error::<T>::ContractAgreementPartyIsNotListed
                );

                Self::reject_license_common(party, license)
            }
        }
    }

    fn reject_license_common(
        party: AccountIdOf<T>,
        license: License<AccountIdOf<T>, HashOf<T>, MomentOf<T>, DeipAssetOf<T>>,
    ) -> DispatchResult {
        let now = pallet_timestamp::Pallet::<T>::get();
        ensure!(
            license.activation_time.unwrap_or(now) <= now,
            Error::<T>::ContractAgreementLicenseIsNotActive
        );
        ensure!(
            now <= license.expiration_time.unwrap_or(now),
            Error::<T>::ContractAgreementLicenseExpired
        );

        let id = license.id;
        let status = LicenseStatus::Rejected(license);
        ContractAgreementMap::<T>::insert(id, Agreement::License(status));

        Self::deposit_event(RawEvent::ContractAgreementRejected(id, party));

        Ok(())
    }
}
