use std::error;
use std::result;

use penrose::PenroseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PwmError {
    #[error("{0}")]
    PenroseError(PenroseError),
    #[error("{0}")]
    UnknownError(Box<dyn error::Error>),
    #[error("{0}")]
    UnknownString(String),
}

impl From<PenroseError> for PwmError {
    fn from(err: PenroseError) -> Self {
        Self::PenroseError(err)
    }
}

impl From<Box<dyn error::Error>> for PwmError {
    fn from(err: Box<dyn error::Error>) -> Self {
        Self::UnknownError(err)
    }
}

impl From<String> for PwmError {
    fn from(err: String) -> Self {
        Self::UnknownString(err)
    }
}

pub type Result<T> = result::Result<T, PwmError>;
