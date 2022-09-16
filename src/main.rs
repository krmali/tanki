use yew::prelude::*;
use crate::{components::tanki::Tanki, data::providers::cards_context_provider::CardsContextProvider};
mod components;
mod data; 

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    html!(
        <CardsContextProvider>
            <Tanki/>
        </CardsContextProvider>
    )

    // // let cards_generator = use_context::<CardsGeneratorProvider<RandomCardsGenerator>>();
    // let wpm_state : UseStateHandle<f64> = use_state(|| 0.0);
    // let cards_generator = use_state(|| RandomCardsGenerator::new(&Cards, CardsNoToGenerate));
    // // let cards_generator = use_state(|| RandomCardsGenerator::);
    // let text = "The quick brown fox jumps over the lazy dog";
    //
    // let wpm_callback = {
    //     let wpm_state = wpm_state.clone();
    //     Callback::from(move |wpm: f64| {
    //         wpm_state.set(wpm);
    // })};
    //
    //
    // html!(
    //     <CardsGeneratorProvider<RandomCardsGenerator> >
    //         <Typing text={text} callback={wpm_callback}/>
    //         <div>
    //             {*wpm_state}
    //         </div>
    //     </CardsGeneratorProvider<RandomCardsGenerator>>
    //     )
}
