pub trait Lockable<T, Id> {
    fn lock(id: Id) -> Result<(), Error>;
}

pub enum Error {
    AlreadyLocked,
    AssetNotFound,
    Unknown,
}
