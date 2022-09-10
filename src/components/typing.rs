use chrono::prelude::*;

use super::letter::{Letter, LetterStatus};
use gloo::console::log;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TypingProps {
    pub text: String,
    pub callback: Callback<f64>,
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
pub fn typing(TypingProps { text, callback }: &TypingProps) -> Html {
    let current_index = use_state(|| 0);
    let start = use_state(Utc::now);
    let mut statuses = vec![LetterStatus::NotDone; text.len()];
    statuses[0] = LetterStatus::Doing;
    let vec = use_state(|| statuses);
    let on_key_down = {
        let text = text.clone();
        let vec = vec.clone();
        let callback = callback.clone();
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
