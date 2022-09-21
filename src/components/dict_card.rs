use stylist::{style, yew::styled_component};
use yew::{html, use_context};

use crate::{
    components::typing::Typing,
    data::providers::cards_context_provider::{CardsContextAction, ReducibleCardsContext},
};

#[styled_component(DictCard)]
pub fn dict_card() -> Html {
    let cards_context =
        use_context::<ReducibleCardsContext>().expect("could not find cards context");

    let style = style!(
        r#"
           color: var(--text-color);
           background-color: --var(--dict-card-bg-color);
           display:flex;
           flex-direction: column;
                       
        "#
    )
    .expect("could not load styles");

    html!(
        <div class={style}>

            <div>
                <div class={css!("font-size: 4rem;line-height: 4rem; font-family: 'FiraSans-Bold';")}>
                    {&cards_context.current_cards[cards_context.current_card_index].front}
                </div>
                <div class={css!("align-self: flex-end;")}>
                    {&cards_context.current_cards[cards_context.current_card_index].back}
                </div>
            </div>
             <div class={css!("align-self: flex-start;")}>
                {&cards_context.current_cards[cards_context.current_card_index].back_example}
            </div>
            <div>
                <div class={css!("")}>
                    {"frequency:"}
                </div>
                <div class={css!("align-self: flex-end;")}>
                    {&cards_context.current_cards[cards_context.current_card_index].frequency}
                </div>
            </div>
        </div>
    )
}
