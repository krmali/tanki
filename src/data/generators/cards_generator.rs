use gloo::console::log;

use crate::data::card::Card;


#[derive(PartialEq)]
pub struct CardsGenerator{
    pub cards_no_to_generate: u32,
}

impl Default for CardsGenerator{
    fn default() -> Self {
        CardsGenerator{
            cards_no_to_generate : 3,
        }
    }
}

impl CardsGenerator{
    pub fn new(cards_no_to_generate: u32) -> Self {
        CardsGenerator{
            cards_no_to_generate,
        }
    }

    pub fn generate_random(&self, all_cards: &Vec<Card>) -> Vec::<Card> {
        let mut res = Vec::<Card>::new();
        for _ in 0..self.cards_no_to_generate{
            let mut rand_arry = [0u8; 128];
            let window_instance = web_sys::window().unwrap();
            let crypto = window_instance.crypto().unwrap();
            let _nums = crypto.get_random_values_with_u8_array(&mut rand_arry).unwrap();
            // let y = rand_arry[0] as usize;
            res.push(all_cards[rand_arry[0] as usize].clone());
        }
        res
    }

}

// impl Clone for CardsGenerator{
//     fn clone(&self) -> Self {
//         *self
//     }
// }

//
// #[derive(Properties, PartialEq)]
// pub struct TypingWrapperProps{
//     pub Cards: Vec<Card>,
//     pub CardsNoToGenerate: u32
// }
//
// #[function_component(TypingWrapper)]
// pub fn typing_wrapper(TypingWrapperProps { Cards, CardsNoToGenerate}: &TypingWrapperProps) -> Html {
//     // let cards_generator = use_context::<CardsGeneratorProvider<RandomCardsGenerator>>();
//     let wpm_state : UseStateHandle<f64> = use_state(|| 0.0);
//     let cards_generator = use_state(|| RandomCardsGenerator::new(&Cards, *CardsNoToGenerate));
//     // let cards_generator = use_state(|| RandomCardsGenerator::);
//     let text = "The quick brown fox jumps over the lazy dog";
//
//     let wpm_callback = {
//         let wpm_state = wpm_state.clone();
//         Callback::from(move |wpm: f64| {
//             wpm_state.set(wpm);
//     })};
//
//
//     html!(
//         <CardsGeneratorProvider<RandomCardsGenerator> >
//         <>
//             <Typing text={text} callback={wpm_callback}/>
//             <div>
//                 {*wpm_state}
//             </div>
//             </>
//         </CardsGeneratorProvider<RandomCardsGenerator>>
//         )
// }
