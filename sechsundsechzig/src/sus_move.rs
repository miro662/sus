use std::str::FromStr;

use crate::{bidding::Bid, cards::Card, contract::GameType, error::SechsUndSechzigError};

pub enum SusMove {
    BiddingMove(Bid),
    PlayMove(Card),
}

impl FromStr for SusMove {
    type Err = SechsUndSechzigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Bid::*;
        use GameType::*;
        use SusMove::*;

        let split_str: Vec<_> = s.split(" ").collect();
        let other_str = if split_str.len() > 1 {
            Some(split_str[1..].join(" "))
        } else {
            None
        };
        if split_str.len() > 0 {
            match (&split_str[0].to_ascii_lowercase() as &str, other_str) {
                ("pass", None) => Ok(BiddingMove(Pass)),
                ("p", None) => Ok(BiddingMove(Pass)),

                ("raise", None) => Ok(BiddingMove(Raise)),
                ("r", None) => Ok(BiddingMove(Raise)),

                ("ask-about", Some(c)) => Ok(BiddingMove(Game(AskingAbout(c.parse()?)))),
                ("as", Some(c)) => Ok(BiddingMove(Game(AskingAbout(c.parse()?)))),
                ("?", Some(c)) => Ok(BiddingMove(Game(AskingAbout(c.parse()?)))),

                ("look-for", Some(c)) => Ok(BiddingMove(Game(LookingFor(c.parse()?)))),
                ("l", Some(c)) => Ok(BiddingMove(Game(LookingFor(c.parse()?)))),

                ("misery", None) => Ok(BiddingMove(Game(Misery))),
                ("m", None) => Ok(BiddingMove(Game(Misery))),

                ("shower", None) => Ok(BiddingMove(Game(Shower))),
                ("s", None) => Ok(BiddingMove(Game(Shower))),

                _ => Ok(PlayMove(s.parse()?)),
            }
        } else {
            Err(SechsUndSechzigError::CardParseError)
        }
    }
}
