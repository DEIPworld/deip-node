pub trait Error {
    fn other() -> Self;
    fn bad_value() -> Self;
    fn bad_target() -> Self;
    fn wrong_owner() -> Self;
    fn unknown_collection() -> Self;
    fn unknown_f_token_id() -> Self;
    fn unknown_item() -> Self;
    fn overflow() -> Self;
    fn insufficient_balance() -> Self;
    fn no_permission() -> Self;
    fn not_fractionalized() -> Self;
}
