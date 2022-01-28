use anyhow;

/// `Error` is an alias for `failure::Error`
pub type Error = anyhow::Error;

/// `Result<T>` is an alias for `Result<T, Error>`
pub type Result<T> = ::std::result::Result<T, Error>;
