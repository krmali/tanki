use gloo::console::log;
use stylist::{style, yew::styled_component};
use yew::{function_component, html, use_context, Callback, Html, Properties};

use crate::{
    components::{dict_card::DictCard, typing::Typing},
    data::providers::cards_context_provider::{CardsContextAction, ReducibleCardsContext},
};

#[allow(non_camel_case_types)]
#[derive(Properties, PartialEq)]
pub struct TypingWrapperProps {
    pub finish_typing_callback: Callback<f64>,
}

#[styled_component(MyStyledComponent)]
fn my_styled_component() -> Html {
    html! {<div class={css!("color: red;")}>{"Hello World!"}</div>}
}

// #[function_component(TypingWrapper)]
#[styled_component(TypingWrapper)]
pub fn typing_wrapper(
    TypingWrapperProps {
        finish_typing_callback,
    }: &TypingWrapperProps,
) -> Html {
    let cards_context =
        use_context::<ReducibleCardsContext>().expect("could not find cards context");

    let wpm_callback = {
        let cards_context = cards_context.clone();
        let finish_typing_callback = finish_typing_callback.clone();
        Callback::from(move |wpm: f64| {
            log!("callback called from typing");
            cards_context.dispatch(CardsContextAction::SetWPM(wpm));
            finish_typing_callback.emit(1.0);
        })
    };
    let style = style!(
        r#"
            color: var(--colorful-error-color);
            height: 100%;
            display: flex;
            flex-direction: column;
            justify-content: space-around;
            align-items: center;
        "#
    )
    .expect("could not load styles");

    html!(
    <div class={style}>
        <div class={css!("font-size: 4rem;line-height: 4rem; color: var(--main-color);")}>
            {cards_context.current_wpm as usize}
        </div>

        <Typing  cards={cards_context.current_cards.clone()}
            wpm_callback={wpm_callback}
            // card_index_callback={card_index_callback}
            show_diacritic_marks={cards_context.show_diacritic_marks}/>
            {if !cards_context.current_cards.is_empty(){
                    html!(
                        // <DictCard />

                            <div>
                                <ul>
                                    <li>{&cards_context.current_cards[cards_context.current_card_index].front}</li>
                                    <li>{&cards_context.current_cards[cards_context.current_card_index].back}</li>
                                    <li>{&cards_context.current_cards[cards_context.current_card_index].frequency}</li>
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
