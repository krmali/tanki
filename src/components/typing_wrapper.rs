use std::rc::Rc;

use yew::{function_component, Html, html, use_context, Callback};

use crate::{data::{card::Card, providers::current_cards_provider::CardsContext, generators::random_cards_generator::RandomCardsGenerator}, components::typing::Typing};


// #[allow(non_camel_case_types)]
// #[derive(Properties, PartialEq)]
// pub struct TypingWrapperProps{
//     pub cards: Vec<Card>,
// }
//

#[function_component(TypingWrapper)]
pub fn typing_wrapper() -> Html {
    let cards_context = use_context::<Rc<CardsContext>>().expect("could not find cards context");
    let cards_generartor = use_context::<RandomCardsGenerator>().expect("could not find cards generator");

    let wpm_callback = {
        // let wpm_state = wpm_state.clone();
        Callback::from(move |wpm: f64| {
            cards_context.current_wpm = wpm;
    })};

    let card_index_callback = {
        let mut cards_context = cards_context.clone();
        Callback::from(move |index: usize| {
            cards_context.current_card_index = index;
    })};

    html!(
        <div>
            <Typing cards={cards_context.current_cards} 
                wpm_callback={wpm_callback} 
                card_index_callback={card_index_callback}
                show_diacritic_marks={cards_context.show_diacritic_marks}/>
            <div>
                {cards_context.current_wpm}
            </div>
        </div>
        )
}
