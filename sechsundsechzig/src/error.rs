use std::{error, fmt};

#[derive(Debug, PartialEq, Eq)]
pub enum SechsUndSechzigError {
    InvaildTeam,
    InvaildPlayer,
    InvaildBid,

    SuitParseError,
    RankParseError,
    CardParseError,

    WrongStage,

    FullTable,

    CardNotInHand
}

impl fmt::Display for SechsUndSechzigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for SechsUndSechzigError {}

pub type SusResult<T> = Result<T, SechsUndSechzigError>;
