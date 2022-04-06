use crate::*;

use sp_std::vec;

pub type Id = H160;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum Terms {
    GeneralContractAgreement,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum IndexTerms {
    LicenseAgreement,
    GeneralContractAgreement,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum Agreement<AccountId, Hash, Moment> {
    None,
    GeneralContract(GeneralContractStatus<AccountId, Hash, Moment>),
}

pub type AgreementOf<T> = Agreement<AccountIdOf<T>, HashOf<T>, MomentOf<T>>;

impl<AccountId, Hash, Moment> Default for Agreement<AccountId, Hash, Moment> {
    fn default() -> Self {
        Agreement::None
    }
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct License<AccountId, Hash, Moment, Asset> {
    pub(crate) id: Id,
    pub(crate) creator: AccountId,
    pub(crate) licenser: AccountId,
    pub(crate) licensee: AccountId,
    pub(crate) hash: Hash,
    pub(crate) activation_time: Option<Moment>,
    pub(crate) expiration_time: Option<Moment>,
    pub(crate) project_id: ProjectId,
    pub(crate) price: Asset,
}

pub type LicenseOf<T> = License<AccountIdOf<T>, HashOf<T>, MomentOf<T>, DeipAsset<T>>;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum LicenseStatus<AccountId, Hash, Moment, Asset> {
    Unsigned(License<AccountId, Hash, Moment, Asset>),
    SignedByLicenser(License<AccountId, Hash, Moment, Asset>),
    Signed(License<AccountId, Hash, Moment, Asset>),
    Rejected(License<AccountId, Hash, Moment, Asset>),
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct GeneralContract<AccountId, Hash, Moment> {
    pub(crate) id: Id,
    pub(crate) creator: AccountId,
    pub(crate) parties: Vec<AccountId>,
    pub(crate) hash: Hash,
    pub(crate) activation_time: Option<Moment>,
    pub(crate) expiration_time: Option<Moment>,
}

pub type GeneralContractOf<T> = GeneralContract<AccountIdOf<T>, HashOf<T>, MomentOf<T>>;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
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
        parties: Vec<T::AccountId>,
        hash: HashOf<T>,
        activation_time: Option<MomentOf<T>>,
        expiration_time: Option<MomentOf<T>>,
        terms: Terms,
    ) -> DispatchResultWithPostInfo {
        ensure!(account == creator, Error::<T>::NoPermission);
        ensure!(!parties.is_empty(), Error::<T>::ContractAgreementNoParties);

        let mut parties = parties;
        parties.sort();
        parties.dedup();

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

            ensure!(activation_time < e, Error::<T>::ContractAgreementEndTimeMustBeLaterStartTime);
        }

        ensure!(
            !ContractAgreementMap::<T>::contains_key(id),
            Error::<T>::ContractAgreementAlreadyExists
        );
        match terms {
            Terms::GeneralContractAgreement => Self::create_general_contract(
                id,
                creator,
                parties,
                hash,
                activation_time,
                expiration_time,
            ),
        }
    }

    pub(super) fn accept_contract_agreement_impl(
        account: AccountIdOf<T>,
        id: Id,
        party: AccountIdOf<T>,
    ) -> DispatchResultWithPostInfo {
        ensure!(account == party, Error::<T>::NoPermission);

        let agreement = ContractAgreementMap::<T>::try_get(id)
            .map_err(|_| Error::<T>::ContractAgreementNotFound)?;

        match agreement {
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
            Agreement::GeneralContract(status) => Self::reject_general_contract(party, status),
        }
    }

    fn create_general_contract(
        id: Id,
        creator: AccountIdOf<T>,
        parties: Vec<T::AccountId>,
        hash: HashOf<T>,
        activation_time: Option<MomentOf<T>>,
        expiration_time: Option<MomentOf<T>>,
    ) -> DispatchResultWithPostInfo {
        let contract =
            GeneralContract { id, creator, parties, hash, activation_time, expiration_time };

        ContractAgreementMap::<T>::insert(
            id,
            Agreement::GeneralContract(GeneralContractStatus::PartiallyAccepted {
                contract,
                accepted_by: vec![],
            }),
        );
        ContractAgreementIdByType::insert(IndexTerms::GeneralContractAgreement, id, ());

        Self::deposit_event(RawEvent::ContractAgreementCreated(id));

        Ok(Some(T::DeipWeightInfo::create_contract_agreement_general_contract()).into())
    }

    fn accept_general_contract(
        party: AccountIdOf<T>,
        status: GeneralContractStatus<AccountIdOf<T>, HashOf<T>, MomentOf<T>>,
    ) -> DispatchResultWithPostInfo {
        match status {
            GeneralContractStatus::Rejected(_) => Err(Error::<T>::ContractAgreementRejected.into()),
            GeneralContractStatus::Accepted(_) =>
                Err(Error::<T>::ContractAgreementAlreadyAccepted.into()),
            GeneralContractStatus::PartiallyAccepted { contract, accepted_by } =>
                Self::accept_general_contract_impl(party, contract, accepted_by),
        }
    }

    fn accept_general_contract_impl(
        party: AccountIdOf<T>,
        contract: GeneralContract<AccountIdOf<T>, HashOf<T>, MomentOf<T>>,
        mut accepted_by: Vec<AccountIdOf<T>>,
    ) -> DispatchResultWithPostInfo {
        ensure!(!accepted_by.contains(&party), Error::<T>::ContractAgreementAlreadyAcceptedByParty);

        ensure!(contract.parties.contains(&party), Error::<T>::ContractAgreementPartyIsNotListed);

        accepted_by.push(party.clone());
        let id = contract.id;
        if accepted_by.len() == contract.parties.len() {
            ContractAgreementMap::<T>::insert(
                id,
                Agreement::GeneralContract(GeneralContractStatus::Accepted(contract)),
            );

            Self::deposit_event(RawEvent::ContractAgreementAccepted(id, party));
            Self::deposit_event(RawEvent::ContractAgreementFinalized(id));
            Ok(Some(T::DeipWeightInfo::accept_contract_agreement_general_contract_finalized())
                .into())
        } else {
            ContractAgreementMap::<T>::insert(
                id,
                Agreement::GeneralContract(GeneralContractStatus::PartiallyAccepted {
                    contract,
                    accepted_by,
                }),
            );

            Self::deposit_event(RawEvent::ContractAgreementAccepted(id, party));
            Ok(Some(
                T::DeipWeightInfo::accept_contract_agreement_general_contract_partially_accepted(),
            )
            .into())
        }
    }

    fn reject_general_contract(
        party: AccountIdOf<T>,
        status: GeneralContractStatus<AccountIdOf<T>, HashOf<T>, MomentOf<T>>,
    ) -> DispatchResult {
        match status {
            GeneralContractStatus::Rejected(_) => Err(Error::<T>::ContractAgreementRejected.into()),
            GeneralContractStatus::Accepted(_) =>
                Err(Error::<T>::ContractAgreementAlreadyAccepted.into()),

            GeneralContractStatus::PartiallyAccepted { contract, accepted_by } => {
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
            },
        }
    }
}
