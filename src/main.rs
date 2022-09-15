use std::fs;

use gloo::net::http::Request;
use serde::{Deserialize,Serialize};
use yew::prelude::*;
use gloo::console::log;
mod components;
mod data; 
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
    // let cards_generator = use_context::<CardsGeneratorProvider<RandomCardsGenerator>>();
    let wpm_state : UseStateHandle<f64> = use_state(|| 0.0);
    let cards_generator = use_state(|| RandomCardsGenerator::new(&Cards, CardsNoToGenerate));
    // let cards_generator = use_state(|| RandomCardsGenerator::);
    let text = "The quick brown fox jumps over the lazy dog";

    let wpm_callback = {
        let wpm_state = wpm_state.clone();
        Callback::from(move |wpm: f64| {
            wpm_state.set(wpm);
    })};


    html!(
        <CardsGeneratorProvider<RandomCardsGenerator> >
            <Typing text={text} callback={wpm_callback}/>
            <div>
                {*wpm_state}
            </div>
        </CardsGeneratorProvider<RandomCardsGenerator>>
        )
}
