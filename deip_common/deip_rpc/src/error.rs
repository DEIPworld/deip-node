pub use jsonrpc_core::{Error as RpcError, ErrorCode as RpcErrorCode};

pub trait GetError {
    fn get_error() -> Error;
}

#[repr(i64)]
pub enum Error {
    ScRpcApiError,
    AssetDetailsDecodeFailed = 1,
    AssetIdDecodeFailed = 2,
    AccountIdDecodeFailed = 3,
    AssetBalanceDecodeFailed = 4,
    NoneForReturnedKey = 5,
    DaoDecodeFailed = 6,
    DaoApiGetFailed = 7,
    DaoApiGetMultiFailed = 8,
    DomainDecodeFailed = 9,
    DaoIdDecodeFailed = 10,
    DomainIdDecodeFailed = 11,
    ProjectIdDecodeFailed = 12,
    ProjectDecodeFailed = 13,
    InvestmentOpportunityApiGetFailed = 14,
    InvestmentIdDecodeFailed = 15,
    InvestmentOpportunityDecodeFailed = 16,
    AgreementIdDecodeFailed = 17,
    AgreementDecodeFailed = 18,
    AgreementApiGetFailed = 19,
    DomainApiGetFailed = 20,
    ProjectApiGetFailed = 21,
    ProjectContentApiGetFailed = 22,
    ProjectContentIdDecodeFailed = 23,
    ProjectContentDecodeFailed = 24,
    ReviewApiGetFailed = 25,
    ReviewIdDecodeFailed = 26,
    ReviewDecodeFailed = 27,
    UpvoteIdDecodeFailed = 28,
    UpvoteDecodeFailed = 29,
    ProposalIdDecodeFailed = 30,
    ProposalDecodeFailed = 31,
    NdaApiGetFailed = 32,
    NdaIdDecodeFailed = 33,
    NdaDecodeFailed = 34,
}

impl Into<RpcErrorCode> for Error {
    fn into(self) -> RpcErrorCode {
        const BASE: i64 = 9900;

        RpcErrorCode::ServerError(BASE + self as i64)
    }
}

pub fn to_rpc_error(e: Error, data: Option<String>) -> RpcError {
    RpcError {
        message: String::new(),
        code: e.into(),
        data: data.map(|d| d.into()),
    }
}
