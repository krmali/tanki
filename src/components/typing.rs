use chrono::prelude::*;
use gloo::console::log;

use crate::data::card::Card;

use super::letter::{Letter, LetterStatus};
use yew::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Properties, PartialEq)]
pub struct TypingProps {
    pub cards: Vec<Card>,
    pub wpm_callback: Callback<f64>,
    pub card_index_callback: Callback<usize>,
    pub show_diacritic_marks: bool,
}

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
    let mut text = text_vec.iter().fold("".to_string(), |acc, c| acc + c + " | ");
    if !text.is_empty(){text = text[0..text.len()-3].to_string();}
    let mut cards_position : Vec<usize> = vec![];
    let mut sum = 0;
    for (i, el) in text_vec.into_iter().enumerate(){
        if i < el.len()-1 {sum += el.len() + 2;}
        cards_position.push(sum);
    }
    let mut statuses = vec![LetterStatus::NotDone; text.len()];
    if !text.is_empty(){
        statuses = vec![LetterStatus::NotDone; text.len()];
        statuses[0] = LetterStatus::Doing;
    }
    let statuses_vec = use_state(|| statuses);

    let start_callback = {
        let statuses_vec = statuses_vec.clone();
        let text = text.clone();
        Callback::from(move |_event: KeyboardEvent| {
            if (*statuses_vec).is_empty(){
                let mut new_vec = vec![LetterStatus::NotDone; text.len()];
                new_vec[0] = LetterStatus::Doing;
                statuses_vec.set(new_vec);
            }
        })
    };

    let on_key_down = {
        let text = text.clone();
        let vec = statuses_vec.clone();
        let callback = wpm_callback.clone();
        let card_index_callback = card_index_callback.clone();
        Callback::from(move |event: KeyboardEvent| {
            // log!(event.clone());
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
            if input.chars().next() != text.chars().nth(*current_index) {
                new_vec[*current_index] = LetterStatus::WronglyDone;
            }
            vec.set(new_vec);
            if (*current_index) == text_len {
                callback.emit(calculate_wpm(*start, &text, &vec));
            }

            //update card_index
            if cards_position[*current_card_index] < *current_index{
                current_card_index.set(*current_card_index+1);
                card_index_callback.emit(*current_card_index);
            }
        })
    };

    if cards.is_empty() || statuses_vec.is_empty() || text.is_empty(){
        return html!(<div onkeydown={start_callback} tabindex={0}>{"press any key to start.."}</div>);
    }else{
        let letters: Html = text
            .chars()
            .enumerate()
            .map(|(index, letter)| {
                html!(
                <Letter status={(*statuses_vec)[index]} character={letter}/>
                )
            })
            .collect();
        html!(
        <div onkeydown={on_key_down} tabindex={0}>
            {letters}
        </div>
        )
    }
}
