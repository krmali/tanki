use gloo::{net::http::Request, console::log};
use yew::{Html, function_component, html, UseStateHandle, use_state, use_effect_with_deps, use_context };
use crate::{data::{card::Card, providers::cards_context_provider::{ReducibleCardsContext, CardsContextAction}, generators::cards_generator::CardsGenerator}, data::providers::cards_context_provider::CardsContextProvider, 
    components::typing_wrapper::TypingWrapper,};



#[function_component(Tanki)]
pub fn tanki() -> Html {
    let cards : UseStateHandle<Vec<Card>>= use_state(Vec::new); 
    let cards_context = use_context::<ReducibleCardsContext>().expect("could not find cards context");

    {
        // let cards = cards.clone();
        use_effect_with_deps( move |_| {
            // let cards = cards.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_cards : Vec<Card> = Request::get("http://localhost:8081/german.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                
                // log!(serde_json::to_string_pretty(&fetched_cards).unwrap());
                // log!("this is the parsed file");
                let mut generator = CardsGenerator::default();
                let generated_cards = generator.generate_random(&fetched_cards);
                cards.set(fetched_cards);
                cards_context.dispatch(CardsContextAction::SetCurrentCards(generated_cards));
            });
            || ()
        }, ());
    }

    html!(
            <TypingWrapper/>
        )
}
