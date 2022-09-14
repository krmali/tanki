use super::{card::Card, cards_generator::CardsGenerator};

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
            let mut rand_arry = [0u8; 128];
            let window_instance = web_sys::window().unwrap();
            let crypto = window_instance.crypto().unwrap();
            let nums = crypto.get_random_values_with_u8_array(&mut rand_arry).unwrap();
            // let y = rand_arry[0] as usize;
            res.push(self.cards[rand_arry[0] as usize].clone());
        }
        self.generations_no +=1;
        res
    }

}