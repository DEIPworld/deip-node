pub trait Error {
    fn bad_value() -> Self;
    fn bad_target() -> Self;
    fn unknown_collection() -> Self;
    fn other() -> Self;
    fn overflow() -> Self;
    fn insufficient_balance() -> Self;
    fn forbidden_for_fractionalized() -> Self;
    fn no_permission() -> Self;
}
