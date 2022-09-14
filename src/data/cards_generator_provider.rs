use yew::{Properties, Children, Html, use_state, html, ContextProvider};
use yew::function_component;
use super::cards_generator::CardsGenerator;

#[derive(Properties, PartialEq)]
// pub struct CardsGeneratorProviderProps<'a, T: CardsGenerator + PartialEq>{
pub struct CardsGeneratorProviderProps<T> where T : CardsGenerator + PartialEq {
    pub cards_generator: T,
    pub children: Children,
}

#[function_component(CardsGeneratorProvider)]
pub fn cards_generator_provider<T: CardsGenerator + PartialEq + Clone + 'static>(
    CardsGeneratorProviderProps { cards_generator, children }: &CardsGeneratorProviderProps<T>) -> Html {
    let cards_generator = use_state(|| cards_generator.clone());

    html!{
        // <ContextProvider context={(*cards_generator).clone()}>
        <ContextProvider<T> context={(*cards_generator).clone()}>
            { for children.iter() }
        </ContextProvider<T>>
        }
}
