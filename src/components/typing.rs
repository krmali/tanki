use chrono::prelude::*;

use crate::data::card::Card;

use super::letter::{Letter, LetterStatus};
use gloo::console::log;
use yew::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Properties, PartialEq)]
pub struct TypingProps {
    pub cards: Vec<Card>,
    pub wpm_callback: Callback<f64>,
    pub card_index_callback: Callback<usize>,
    pub show_diacritic_marks: bool,
}

// impl Default for TypingProps {
//     fn default() -> Self {
//         TypingProps {
//             text: "The quick brown fox jumps over the lazy dog".to_owned(),
//             callback: Callback::from(move|wpm: f64| {})
//         }
//     }
// }

pub fn calculate_wpm(start: DateTime<Utc>, text: &str, vec: &Vec<LetterStatus>) -> f64 {
    let end = Utc::now();
    let elapsed_duration_millis = end.signed_duration_since(start).num_milliseconds() as f64;
    let mut errors_no: f64 = 0.0;
    for i in vec {
        if *i == LetterStatus::WronglyDone {
            errors_no += 1.0
        }
    }
    let words: f64 = (text.chars().count() as f64) / (5_f64);
    (words / elapsed_duration_millis) * (60000_f64)
        - (errors_no / elapsed_duration_millis) * (60000_f64)
}


#[function_component(Typing)]
pub fn typing(TypingProps { cards, wpm_callback, card_index_callback, show_diacritic_marks}: &TypingProps) -> Html {
    let current_index = use_state(|| 0);
    let current_card_index = use_state(|| 0);
    let start = use_state(Utc::now);
    let text_vec : Vec<String> = match show_diacritic_marks {
        true =>  cards.iter().map(|c| c.to_string()).collect(),
        false =>  cards.iter().map(|c| c.to_string_without_diacritic()).collect(),
    };
    let text = text_vec.iter().fold("".to_string(), |acc, c| acc + &c + " | ");
    let cards_position : Vec<usize> = vec![];
    let sum = 0;
    for i in text_vec{
        cards_position.push(sum);
        sum += i.len()+3;
    }
    let mut statuses = vec![LetterStatus::NotDone; text.len()];
    statuses[0] = LetterStatus::Doing;
    let vec = use_state(|| statuses);
    let on_key_down = {
        let text = text.clone();
        let vec = vec.clone();
        let callback = wpm_callback.clone();
        Callback::from(move |event: KeyboardEvent| {
            log!(event.clone());
            let input = event.key();
            if input == "Backspace" {
                if *current_index == 0 {
                    return;
                }
                let mut new_vec = vec![LetterStatus::NotDone; text.len()];
                for (i, _) in vec.iter().enumerate() {
                    new_vec[i] = vec[i];
                }
                new_vec[*current_index] = LetterStatus::NotDone;
                new_vec[*current_index - 1] = LetterStatus::Doing;
                vec.set(new_vec);
                current_index.set(*current_index - 1);

                //update card_index
                if cards_position[*current_card_index] > *current_index{
                    current_card_index.set(*current_card_index-1);
                    card_index_callback.emit(*current_card_index);
                }
                return;
            }
            if input.len() > 1 {
                return;
            }

            // character now is either a right or a wrong character
            if *current_index == 0 {
                start.set(Utc::now());
            }
            let text_len = text.len() - 1;
            let mut new_vec = vec![LetterStatus::NotDone; text.len()];
            for (i, _) in vec.iter().enumerate() {
                new_vec[i] = vec[i];
            }
            new_vec[*current_index] = LetterStatus::Done;
            if *current_index + 1 < text.len() {
                new_vec[*current_index + 1] = LetterStatus::Doing;
                current_index.set(*current_index + 1);
            }
            // if input.bytes().nth(0) != text.bytes().nth(*current_index) {
            if input.chars().next() != text.chars().nth(*current_index) {
                new_vec[*current_index] = LetterStatus::WronglyDone;
            }
            vec.set(new_vec);
            if (*current_index) == text_len {
                callback.emit(calculate_wpm(*start, &text, &*vec));
            }

            //update card_index
            if cards_position[*current_card_index] < *current_index{
                current_card_index.set(*current_card_index+1);
                card_index_callback.emit(*current_card_index);
            }
        })
    };

    let letters: Html = text
        .chars()
        .enumerate()
        .map(|(index, letter)| {
            html!(
            <Letter status={(*vec)[index]} character={letter}/>
            )
        })
        .collect();
    html!(
    <div onkeydown={on_key_down} tabindex={0}>
        {letters}
    </div>
    )
}
