use gloo::{net::http::Request, console::log};
use yew::{Html, function_component, html, UseStateHandle, use_state, Callback, use_effect_with_deps};

use crate::{data::card::Card, components::typing::Typing};


#[function_component(Tanki)]
pub fn tanki() -> Html {
    let cards : UseStateHandle<Vec<Card>>= use_state(Vec::new); 
    let wpm_state : UseStateHandle<f64> = use_state(|| 0.0);
    let text = "think you number early back same a through should or all these mean consider here they might show even state group eye thing not there";


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

            });
            || ()
        }, ());
    }

    html!(
        <>
            <Typing text={text} callback={wpm_callback}/>
            <div>
                {*wpm_state}
            </div>
            </>
        )
}
