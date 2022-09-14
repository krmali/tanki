use std::{fmt::Display, cmp::Ordering};

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card{
    pub front: String,
    pub back: String,
    pub frequencey: u32,
    pub front_example: String,
    pub back_example: String
}

struct Cards(Vec<Card>);

impl Display for Card{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.front, self.back)
    }
}

impl Ord for Card{
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequencey.cmp(&other.frequencey)
    }
}

impl PartialOrd for Card{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card{
    fn eq(&self, other: &Self) -> bool {
        self.frequencey == other.frequencey
    }
}

impl Eq for Card{}
