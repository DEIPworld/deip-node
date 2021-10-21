use super::*;

/// Unique Review reference
pub type Id = H160;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Vote<AccountId, Moment> {
    dao: AccountId,
    review_id: ReviewId,
    domain_id: DomainId,
    voting_time: Moment,
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Review<Hash, AccountId> {
    /// Reference for external world and uniques control
    external_id: Id,
    /// Reference to the Team
    author: AccountId,
    /// Hash of content
    content: Hash,
    /// List of Domains aka tags Project matches
    domains: Vec<DomainId>,
    /// Model number by which the evaluation is carried out
    assessment_model: u32,
    /// percent in "50.00 %" format
    weight: Vec<u8>,
    /// Reference to Project Content
    project_content_external_id: ProjectContentId,
}

impl<T: Config> Module<T> {
    pub(super) fn create_review_impl(
        account: T::AccountId,
        external_id: Id,
        author: T::DeipAccountId,
        content: T::Hash,
        domains: Vec<DomainId>,
        assessment_model: u32,
        weight: Vec<u8>,
        project_content_external_id: ProjectContentId,
    ) -> DispatchResult {
        ensure!(!domains.is_empty(), Error::<T>::ReviewNoDomainSpecified);

        for domain in &domains {
            ensure!(Domains::contains_key(&domain), Error::<T>::DomainNotExists);
        }

        let review = Review {
            external_id,
            author: author.into(),
            content,
            domains,
            assessment_model,
            weight,
            project_content_external_id,
        };

        ensure!(!ReviewMap::<T>::contains_key(review.external_id), Error::<T>::ReviewAlreadyExists);

        let content = ProjectContentMap::<T>::try_get(review.project_content_external_id)
            .map_err(|_| Error::<T>::NoSuchProjectContent)?;

        ReviewMap::<T>::insert(review.external_id, review.clone());
        ReviewIdByProjectId::insert(content.project_external_id, review.external_id, ());
        ReviewIdByContentId::insert(content.external_id, review.external_id, ());
        ReviewIdByAccountId::<T>::insert(review.author.clone(), review.external_id, ());

        Self::deposit_event(RawEvent::ReviewCreated(account, review));

        Ok(())
    }

    pub(super) fn upvote_review_impl(
        account: T::AccountId,
        review_id: ReviewId,
        domain_id: DomainId,
    ) -> DispatchResult {
        ensure!(
            Domains::contains_key(domain_id),
            Error::<T>::ReviewVoteNoSuchDomain
        );

        let review =
            ReviewMap::<T>::try_get(review_id).map_err(|_| Error::<T>::ReviewVoteNoSuchReview)?;
        ensure!(
            review.domains.contains(&domain_id),
            Error::<T>::ReviewVoteUnrelatedDomain
        );

        ensure!(
            !ReviewVoteMap::<T>::contains_key((review_id, account.clone(), domain_id)),
            Error::<T>::ReviewAlreadyVotedWithDomain
        );

        let vote = Vote {
            dao: account.clone(),
            review_id,
            domain_id,
            voting_time: pallet_timestamp::Pallet::<T>::get(),
        };

        ReviewVoteMap::<T>::insert((review_id, account.clone(), domain_id), vote);
        VoteIdByReviewId::<T>::insert(review_id, (review_id, account.clone(), domain_id), ());
        VoteIdByAccountId::<T>::insert(account.clone(), (review_id, account.clone(), domain_id), ());

        Self::deposit_event(RawEvent::ReviewUpvoted(review_id, account, domain_id));

        Ok(())
    }
}
