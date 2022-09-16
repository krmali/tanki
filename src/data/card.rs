use std::{cmp::Ordering};

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card{
    pub front: String,
    pub back: String,
    pub frequencey: u32,
    pub front_example: String,
    pub back_example: String
}

impl ToString for Card{
    fn to_string(&self) -> String {
        self.front_example.to_string()
    }
}

impl Card{
    pub fn to_string_without_diacritic(&self) -> String {
        let mut tmp = [0;4];
        let mut result = "".to_owned();
        for c in self.front_example.chars(){
            let char = match c {
               'Ü' => "U",
               'ü' => "u",
               'Ä' => "A",
               'ä' => "a",
               'Ö' => "O",
               'ö' => "o",
               'ẞ' => "ss",
               'ß' => "ss",
               _ => c.encode_utf8(&mut tmp),
            };
            result.push_str(char);
        }
        result
    }
}

struct Cards(Vec<Card>);

// impl Display for Card{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "({},{})", self.front, self.back)
//     }
// }

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
