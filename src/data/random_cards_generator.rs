use super::{card::Card, cards_generator::CardsGenerator};
use rand::Rng;

pub struct RandomCardsGenerator{
    cards: Vec<Card>,
    cards_no_to_generate: u32,
    generations_no: u32,
}

impl CardsGenerator for RandomCardsGenerator{
    fn new(&self, map : &Vec<Card>, cards_no_to_generate: u32) -> Self {
        RandomCardsGenerator{
            cards : map.to_vec(),
            cards_no_to_generate,
            generations_no : 0
        }
    }

    fn generate(& mut self) -> Vec::<Card> {
        let cards_no = self.cards.len();

        let mut res = Vec::<Card>::new();
        for _ in 0..self.cards_no_to_generate{
            let num = rand::thread_rng().gen_range(0..cards_no);
            res.push(self.cards[num].clone());
        }
        self.generations_no +=1;
        res
    }

}
