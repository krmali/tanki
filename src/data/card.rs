use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(Debug,Clone, PartialEq, Serialize, Deserialize)]
pub struct Card{
    pub front: String,
    pub back: String,
}

// type Cards = Vec<Card>;
struct Cards(Vec<Card>);

impl Display for Card{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.front, self.back)
    }
}

// impl Display for Cards{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.into_iter().map(|card| {format!("{}", card)}).collect()
//         // write!(f, "({},{})", self.front, self.back)
//     }
// }
