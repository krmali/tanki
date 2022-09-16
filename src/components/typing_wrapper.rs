use yew::{function_component, Html, html, use_context, Callback};

use crate::{data::providers::cards_context_provider::{CardsContextAction, ReducibleCardsContext},
    components::typing::Typing};

#[function_component(TypingWrapper)]
pub fn typing_wrapper() -> Html {
    let cards_context = use_context::<ReducibleCardsContext>().expect("could not find cards context");

    let wpm_callback = {
        let cards_context = cards_context.clone();
        Callback::from(move |wpm: f64| {
            cards_context.dispatch(CardsContextAction::SetWPM(wpm));
    })};

    html!(
        <div>
            <div>
                {cards_context.current_wpm}
            </div>
            <Typing cards={cards_context.current_cards.clone()} 
                wpm_callback={wpm_callback} 
                // card_index_callback={card_index_callback}
                show_diacritic_marks={cards_context.show_diacritic_marks}/>
                {if !cards_context.current_cards.is_empty(){
                        html!(

                                <div>
                                    <ul>
                                        <li>{&cards_context.current_cards[cards_context.current_card_index].front}</li>
                                        <li>{&cards_context.current_cards[cards_context.current_card_index].back}</li>
                                        <li>{&cards_context.current_cards[cards_context.current_card_index].frequencey}</li>
                                        <li>{&cards_context.current_cards[cards_context.current_card_index].back_example}</li>
                                    </ul>
                                </div>
                            )
                    }else{
                        html!(
                            <></>
                            )
                    }
                }
        </div>
        )
}
