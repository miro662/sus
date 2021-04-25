use std::{error, fmt};

#[derive(Debug)]
pub enum SechsUndSechzigError {
    InvaildTeam,
}

impl fmt::Display for SechsUndSechzigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for SechsUndSechzigError {}

pub type SusResult<T> = Result<T, SechsUndSechzigError>;