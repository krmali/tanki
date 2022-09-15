use yew::{Properties, Children, Html, use_state, html, ContextProvider, UseStateHandle};
use yew::function_component;
use crate::data::card::Card;

#[derive(PartialEq, Clone, Debug)]
pub struct CardsContext{
    pub current_cards: Vec<Card>,
    pub current_card_index: usize,
    pub current_wpm: f64,
    pub show_diacritic_marks: bool,

}

#[allow(non_camel_case_types)]
#[derive(Properties, PartialEq)]
pub struct CurrentCardsProviderProps{
    pub children: Children,
}

#[function_component(CurrentCardsProvider)]
pub fn current_cards_provider(
    CurrentCardsProviderProps { children }: &CurrentCardsProviderProps) -> Html {
    let current_cards:UseStateHandle<CardsContext> = use_state(|| CardsContext{
        current_cards: vec![],
        current_card_index: 0,
        current_wpm: 0.0,
        show_diacritic_marks: false
    });

    html!(
        <ContextProvider<CardsContext> context={(*current_cards).clone()}>
            { for children.iter() }
        </ContextProvider<CardsContext>>
        )
}
