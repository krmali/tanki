use yew::{Properties, Children, Html, use_state, html, ContextProvider, UseStateHandle};
use yew::function_component;

use crate::data::generators::random_cards_generator::RandomCardsGenerator;

#[derive(Properties, PartialEq)]
pub struct RandomCardsGeneratorProviderProps{
    pub children: Children,
}

#[function_component(CardsGeneratorProvider)]
pub fn random_cards_generator_provider(
    RandomCardsGeneratorProviderProps { children }: &RandomCardsGeneratorProviderProps) -> Html {
    let cards_generator :UseStateHandle<RandomCardsGenerator> = use_state(|| RandomCardsGenerator::default());

    html!(
        <ContextProvider<RandomCardsGenerator> context={(*cards_generator).clone()}>
            { for children.iter() }
        </ContextProvider<RandomCardsGenerator>>
        )
}
