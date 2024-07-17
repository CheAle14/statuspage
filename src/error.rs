use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("underlying reqwest error")]
    Reqwest(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
