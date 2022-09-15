use super::{card::Card, random_cards_generator::RandomCardsGenerator};

// pub trait CardsGenerator {
//     fn new(vec: &Vec<Card>, cards_no_to_generate: u32) -> Self;
//     fn generate(& mut self) -> Vec::<Card>;
// }

#[derive(PartialEq)]
pub enum CardsGenerator {
    RandomCardGenerator,
}


#[derive(PartialEq)]
pub enum CardsGeneratorType{
    Random,
}

