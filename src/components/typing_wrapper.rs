use yew::{function_component, Html, html, use_context, Callback};

use crate::{data::{providers::cards_context_provider::{CardsContextAction, ReducibleCardsContext}, 
        generators::random_cards_generator::RandomCardsGenerator}, 
    components::typing::Typing};

#[function_component(TypingWrapper)]
pub fn typing_wrapper() -> Html {
    let cards_context = use_context::<ReducibleCardsContext>().expect("could not find cards context");
    // let cards_generartor = use_context::<RandomCardsGenerator>().expect("could not find cards generator");

    let wpm_callback = {
        Callback::from(move |wpm: f64| {
            cards_context.dispatch(CardsContextAction::SetWPM(wpm));
    })};

    let card_index_callback = {
        Callback::from(move |index: usize| {
            cards_context.dispatch(CardsContextAction::SetCardsIndex(index));
        })
    };

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
