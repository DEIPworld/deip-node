use frame_support::codec::FullCodec;
use scale_info::TypeInfo;
use sp_std::fmt::Debug;

pub trait DeipProjectsInfo<AccountId> {
    type ProjectId: TypeInfo + Debug + Clone + FullCodec + PartialEq + AsRef<[u8]>;
    type InvestmentId: TypeInfo + Debug + Clone + FullCodec + PartialEq + AsRef<[u8]>;

    fn try_get_project_team(id: &Self::ProjectId) -> Option<AccountId>;
}
