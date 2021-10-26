use crate::*;

use crate::traits::DeipAssetSystem;
use sp_runtime::traits::Saturating;
use sp_std::vec;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Contribution<AccountId, Balance, Moment> {
    pub sale_id: InvestmentId,
    pub owner: AccountId,
    pub amount: Balance,
    pub time: Moment,
}

impl<T: Config> Module<T> {
    pub(super) fn invest_to_crowdfunding_impl(
        account: T::AccountId,
        sale_id: InvestmentId,
        asset: DeipAssetOf<T>,
    ) -> DispatchResult {
        let sale = SimpleCrowdfundingMap::<T>::try_get(sale_id)
            .map_err(|_| Error::<T>::InvestingNotFound)?;

        ensure!(
            matches!(sale.status, SimpleCrowdfundingStatus::Active),
            Error::<T>::InvestingNotActive
        );

        ensure!(sale.asset_id == *asset.id(), Error::<T>::InvestingWrongAsset);

        let is_hard_cap_reached = sale.total_amount.0.saturating_add(*asset.amount()) >= sale.hard_cap.0;
        let amount_to_contribute = if is_hard_cap_reached {
            sale.hard_cap.0.saturating_sub(sale.total_amount.0)
        } else {
            *asset.amount()
        };

        ensure!(
            T::AssetSystem::transfer_to_reserved(&account, sale.external_id, amount_to_contribute)
                .is_ok(),
            Error::<T>::InvestingNotEnoughFunds
        );

        InvestmentMap::<T>::mutate_exists(sale_id, |contributions| {
            let mut_contributions = match contributions.as_mut() {
                None => {
                    // If the account executes the extrinsic then it exists, so it should have at least one provider
                    // so this cannot fail... but being defensive anyway.
                    let _ = system::pallet::Pallet::<T>::inc_consumers(&account);

                    *contributions = Some(vec![(
                        account.clone(),
                        Contribution {
                            sale_id: sale_id,
                            owner: account.clone(),
                            amount: amount_to_contribute,
                            time: pallet_timestamp::Pallet::<T>::get(),
                        },
                    )]);
                    return;
                }
                Some(c) => c,
            };

            match mut_contributions.binary_search_by_key(&&account, |&(ref a, _)| a) {
                Err(i) => {
                    // see comment above
                    let _ = system::pallet::Pallet::<T>::inc_consumers(&account);

                    mut_contributions.insert(
                        i,
                        (
                            account.clone(),
                            Contribution {
                                sale_id: sale_id,
                                owner: account.clone(),
                                amount: amount_to_contribute,
                                time: pallet_timestamp::Pallet::<T>::get(),
                            },
                        ),
                    );
                }
                Ok(i) => {
                    mut_contributions[i].1.amount =
                        amount_to_contribute.saturating_add(mut_contributions[i].1.amount);
                }
            };
        });

        Self::collect_funds(sale_id, amount_to_contribute).expect("collect; already found");

        Self::deposit_event(RawEvent::Invested(sale_id, account.clone()));

        if is_hard_cap_reached {
            Self::finish_crowdfunding_by_id(sale_id).expect("finish; already found");
        }

        Ok(())
    }
}
