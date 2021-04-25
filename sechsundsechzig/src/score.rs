use core::fmt;
use std::collections::HashMap;

use crate::{team::Team, variant::Variant};
#[derive(Debug, Clone)]
pub struct Score {
    scores: HashMap<Team, i32>,
}

impl Score {
    const MAX_POINTS: i32 = 66;

    pub fn empty(variant: Variant) -> Score {
        let teams = Team::teams(variant);
        Score {
            scores: teams.map(|t| (t, 0)).collect(),
        }
    }

    pub fn add_points(&mut self, team: &Team, points: i32) {
        *self.scores.get_mut(&team).expect("Invaild team") += points;
    }

    pub fn winner(&self) -> Option<&Team> {
        self.scores
            .iter()
            .filter(|(_, score)| **score >= Score::MAX_POINTS)
            .max_by_key(|(_, score)| **score)
            .map(|(team, _)| team)
    }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let winner = self.winner();
        for (team, score) in &self.scores {
            let tag = if winner == Some(&team) {
                " [winner]"
            } else {""};
            write!(f, "{} | {}{}", team, score, tag)?;
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_returns_correct_score_for_three_players() {
        let variant = Variant::ThreePlayers;
        let empty_scores = Score::empty(variant);
        assert_eq!(3, empty_scores.scores.len());
        for team in Team::teams(variant) {
            assert!(empty_scores.scores.contains_key(&team));
            assert_eq!(0, empty_scores.scores[&team]);
        }
    }

    #[test]
    fn empty_returns_correct_score_for_four_players() {
        let variant = Variant::FourPlayers;
        let empty_scores = Score::empty(variant);
        assert_eq!(2, empty_scores.scores.len());
        for team in Team::teams(variant) {
            assert!(empty_scores.scores.contains_key(&team));
            assert_eq!(0, empty_scores.scores[&team]);
        }
    }

    #[test]
    fn returns_winner() {
        let mut scores = Score::empty(Variant::ThreePlayers);
        let winner = Team::teams(Variant::ThreePlayers).next().unwrap();
        scores.add_points(&winner, Score::MAX_POINTS);

        assert_eq!(Some(&winner), scores.winner());
    }

    #[test]
    fn returns_winner_with_greater_score() {
        let mut scores = Score::empty(Variant::ThreePlayers);
        let almost_winner = Team::teams(Variant::ThreePlayers).next().unwrap();
        let winner = Team::teams(Variant::ThreePlayers).next().unwrap();
        scores.add_points(&almost_winner, Score::MAX_POINTS);
        scores.add_points(&winner, Score::MAX_POINTS + 1);

        assert_eq!(Some(&winner), scores.winner());
    }

    #[test]
    fn does_not_return_winner_when_no_one_won() {
        let scores = Score::empty(Variant::ThreePlayers);

        assert_eq!(None, scores.winner());
    }
}
