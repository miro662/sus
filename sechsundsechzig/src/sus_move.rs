use std::str::FromStr;

use crate::error::SechsUndSechzigError;

pub struct SusMove;

impl FromStr for SusMove {
    type Err = SechsUndSechzigError;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(SusMove)
    }
}
