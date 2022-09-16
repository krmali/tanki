use gloo::{net::http::Request, console::log};
use yew::{Html, function_component, html, UseStateHandle, use_state, use_effect_with_deps };
use crate::{data::card::Card, data::providers::cards_context_provider::CardsContextProvider, 
    components::typing_wrapper::TypingWrapper, 
    data::providers::random_cards_generator_provider::RandomCardsGeneratorProvider};



#[function_component(Tanki)]
pub fn tanki() -> Html {
    let cards : UseStateHandle<Vec<Card>>= use_state(Vec::new); 
    // let cards_context = use_context::<Rc<CardsContext>>().expect("could not find cards context from tanki component");

    {
        // let cards = cards.clone();
        use_effect_with_deps( move |_| {
            // let cards = cards.clone();
            wasm_bindgen_futures::spawn_local(async move {
                // log!(fetched_cards);
                let fetched_cards : Vec<Card> = Request::get("http://localhost:8081/german.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                
                log!(serde_json::to_string_pretty(&fetched_cards).unwrap());
                log!("this is the parsed file");
                cards.set(fetched_cards);

            });
            || ()
        }, ());
    }

    html!(
        <CardsContextProvider>
            <RandomCardsGeneratorProvider>
                <TypingWrapper/>
            </RandomCardsGeneratorProvider>
        </CardsContextProvider>
        )
}
