use super::card::Card;

pub trait CardsGenerator {
    fn new(&self, map : &Vec<Card>, cards_no_to_generate: u32) -> Self;
    fn generate(& mut self) -> Vec::<Card>;
}

// pub enum CardsGenerator {
//     RandomCardGenerator,
// }
