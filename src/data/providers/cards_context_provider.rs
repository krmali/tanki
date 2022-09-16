use std::rc::Rc;

use yew::{Properties, Children, Html, html, ContextProvider, use_reducer, Reducible, UseReducerHandle};
use yew::function_component;
use crate::data::card::Card;

pub enum CardsContextAction{
    SetCurrentCards(Vec<Card>),
    SetCardsIndex(usize),
    SetWPM(f64)
}

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct CardsContext{
    pub current_cards: Vec<Card>,
    pub current_card_index: usize,
    pub current_wpm: f64,
    pub show_diacritic_marks: bool,
}

impl Default for CardsContext{
    fn default() -> Self {
        CardsContext{
                current_cards: vec![],
                current_card_index: 0,
                current_wpm: 0.0,
                show_diacritic_marks: false
            }
    }
}

impl Reducible for CardsContext{
    type Action = CardsContextAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action{
            CardsContextAction::SetCurrentCards(cards) => CardsContext{
                current_cards: cards,
                current_card_index: self.current_card_index,
                current_wpm: self.current_wpm,
                show_diacritic_marks: self.show_diacritic_marks,
            }.into(),
            CardsContextAction::SetCardsIndex(index) => CardsContext{
                current_cards: self.current_cards.clone(),
                current_card_index: index,
                current_wpm: self.current_wpm,
                show_diacritic_marks: self.show_diacritic_marks,
            }.into(),
            CardsContextAction::SetWPM(wpm) => CardsContext{
                current_cards: self.current_cards.clone(),
                current_card_index: self.current_card_index,
                current_wpm: wpm,
                show_diacritic_marks: self.show_diacritic_marks,
            }.into(),
        } 
    }
}

impl CardsContext{
    pub fn set_current_cards(&mut self, cards: Vec<Card>) {
        self.current_cards = cards;
    }
    
    pub fn set_current_card_index(&mut self, index: usize) {
        self.current_card_index = index;
    }
}

pub type ReducibleCardsContext = UseReducerHandle<CardsContext>;

#[allow(non_camel_case_types)]
#[derive(Properties, PartialEq)]
pub struct CurrentCardsProviderProps{
    pub children: Children,
}

#[function_component(CardsContextProvider)]
pub fn cards_context_provider(
    CurrentCardsProviderProps { children }: &CurrentCardsProviderProps) -> Html {
    let current_cards = use_reducer(|| CardsContext::default());

    html!(
        <ContextProvider<ReducibleCardsContext> context={current_cards}>
            { for children.iter() }
        </ContextProvider<ReducibleCardsContext>>
        )
}
