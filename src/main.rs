use std::fs;

use gloo::net::http::Request;
use serde::{Deserialize,Serialize};
use yew::prelude::*;
use gloo::console::log;
mod components;
mod data; 
use components::typing::Typing;
use data::card::Card;

use crate::data::{cards_generator_provider::CardsGeneratorProvider, random_cards_generator::RandomCardsGenerator};

// pub type Welcome = Vec<WelcomeElement>;

// #[derive(Serialize, Deserialize)]
// pub struct WelcomeElement {
//     #[serde(rename = "__type__")]
//     welcome_type: String,
//     fields: Vec<String>,
//     guid: String,
//     note_model_uuid: String,
//     tags: Vec<Option<serde_json::Value>>,
// }
//
// fn test(){
//     let json = fs::read_to_string("./german.json")
//         .unwrap();
//     let welcomes: Welcome = serde_json::from_str(&json)
//         .unwrap();
//     let mut cards = Vec::<Card>::new();
//     for w in welcomes{
//         let n_card = Card{
//             front: w.fields[0].clone(),
//             back: w.fields[3].clone(),
//             frequencey: w.fields[2].clone(),
//             front_example: w.fields[1].clone(),
//             back_example: w.fields[5].clone(),
//         };
//         cards.push(n_card);
//     }
//     fs::write("german_2.json", serde_json::to_string_pretty(&cards).unwrap().as_bytes());
// }

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    let cards : UseStateHandle<Vec<Card>>= use_state(Vec::new); 
    let wpm_state : UseStateHandle<f64> = use_state(|| 0.0);
    // let text = "The quick brown fox jumps over the lazy dog";
    // let text = "This makes sense because proof-reading and correcting errors takes up more time than simply typing a passage correctly in the first place. Less mistakes also means less chance for errors being missed during proof-reading and making their way into the final product.";
    let text = "think you number early back same a through should or all these mean consider here they might show even state group eye thing not there";

    let wpm_callback = {
        let wpm_state = wpm_state.clone();
        Callback::from(move |wpm: f64| {
            wpm_state.set(wpm);
    })};

    {
        // let cards = cards.clone();
        use_effect_with_deps( move |_| {
            // let cards = cards.clone();
            wasm_bindgen_futures::spawn_local(async move {
                // let fetched_cards = Request::get("http://localhost:8081/german.json")
                //     .send()
                //     .await
                //     .unwrap()
                //     .text()
                //     .await
                //     .unwrap();

                // log!(fetched_cards);
                let fetched_cards : Vec<Card> = Request::get("http://localhost:8081/german.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                
                log!(serde_json::to_string_pretty(&fetched_cards).unwrap());
                // cards.set(fetched_cards);
                // let v = vec![1,2,3];
                // log!(v);
                // let card = fetched_cards[0];
                // log!(format!("{}", fetched_cards));
                // log!("{:#?}", fetched_cards[0]);

            });
            || ()
        }, ());
    }

    html!(
        <div>
            <Typing text={text} callback={wpm_callback}/>
            <div>
                {*wpm_state}
            </div>
        </div>
        )
}

#[derive(Properties, PartialEq)]
pub struct TypingWrapperProps{
    pub Cards: Vec<Card>,
    pub CardsNoToGenerate: u32
}

#[function_component(TypingWrapper)]
pub fn typing_wrapper(TypingWrapperProps { Cards, CardsNoToGenerate}: &TypingWrapperProps) -> Html {
    // let cards_generator = use_context::<CardsGeneratorProvider<RandomCardsGenerator>>();
    let wpm_state : UseStateHandle<f64> = use_state(|| 0.0);
    // let cards_generator = use_state(|| RandomCardsGenerator::new(&Cards, CardsNoToGenerate));
    let cards_generator = use_state(|| RandomCardsGenerator::);
    let text = "The quick brown fox jumps over the lazy dog";

    let wpm_callback = {
        let wpm_state = wpm_state.clone();
        Callback::from(move |wpm: f64| {
            wpm_state.set(wpm);
    })};


    html!(
        // <CardsGeneratorProvider cards_generator={}>
        <>
            <Typing text={text} callback={wpm_callback}/>
            <div>
                {*wpm_state}
            </div>
            </>
        // </CardsGeneratorProvider>
        )
}
