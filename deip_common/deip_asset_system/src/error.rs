pub trait Error {
    fn bad_value() -> Self;
    fn unknown_collection() -> Self;
    fn other() -> Self;
    fn overflow() -> Self;
    fn insufficient_balance() -> Self;
    fn is_fractional() -> Self;
}
