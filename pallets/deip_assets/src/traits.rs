pub trait DeipProjectsInfo<AccountId> {
    type ProjectId: sp_std::fmt::Debug + Clone + frame_support::codec::FullCodec + PartialEq + AsRef<[u8]>;
    type InvestmentId: sp_std::fmt::Debug + Clone + frame_support::codec::FullCodec + PartialEq + AsRef<[u8]>;

    fn try_get_project_team(id: &Self::ProjectId) -> Option<AccountId>;
}
