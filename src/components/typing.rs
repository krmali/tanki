use std::collections::HashMap;

use chrono::prelude::*;
use gloo::console::log;
use stylist::{css, yew::styled_component, style};

use crate::data::{card::Card, providers::cards_context_provider::{ReducibleCardsContext, CardsContextAction}};

use super::letter::{Letter, LetterStatus};
use yew::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Properties, PartialEq)]
pub struct TypingProps {
    pub cards: Vec<Card>,
    pub wpm_callback: Callback<f64>,
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


#[styled_component(Typing)]
pub fn typing(TypingProps { cards, wpm_callback, show_diacritic_marks}: &TypingProps) -> Html {
    let cards_context = use_context::<ReducibleCardsContext>().expect("could not find cards context");
    let current_index = use_state(|| 0);
    let current_card_index = use_state(|| 0);
    let start = use_state(Utc::now);
    // let text_vec : Vec<String> = match show_diacritic_marks {
    //     true =>  cards.iter().map(|c| c.to_string()).collect(),
    //     false =>  cards.iter().map(|c| c.to_string_without_diacritic()).collect(),
    // };
    let text_vec : Vec<String> = cards.iter().map(|c| c.to_string()).collect();
    let map: HashMap<char, char> = HashMap::from([
               ('Ü', 'U'), ('ü', 'u'), ('Ä', 'A'), ('ä', 'a'),
               ('Ö', 'O'), ('ö', 'o'), ('ẞ', 's'), ('ß', 's'),
    ]);
    // let text  = text_vec.iter().fold("".to_string(), |acc, c| acc + c + " | ");
    // if !text.is_empty(){text = text[0..text.len()-3].to_string();}
    let mut temp_text = "".to_string();
    for (i, el) in (&text_vec).iter().enumerate(){
        temp_text += el;
        if i < &text_vec.len()-1{temp_text += " | ";}
    }
    let text = temp_text;
    let mut cards_position : Vec<(usize, usize)> = vec![];
    let mut sum = 0;
    for (i, el) in text_vec.into_iter().enumerate(){
        let el_len = el.chars().count();
        let mut temp: (usize, usize) = (sum+1,0);
        if i < el_len-1 {sum += el_len + 2;}
        else {sum += el_len;}
        temp.1 = sum;
        cards_position.push(temp);
    }
    let text_len = text.chars().count();
    let mut statuses = vec![LetterStatus::NotDone; text_len];
    if !text.is_empty(){
        statuses = vec![LetterStatus::NotDone; text_len];
        statuses[0] = LetterStatus::Doing;
    }
    let statuses_vec = use_state(|| statuses);

    let start_callback = {
        let statuses_vec = statuses_vec.clone();
        Callback::from(move |_event: KeyboardEvent| {
            if (*statuses_vec).is_empty(){
                let mut new_vec = vec![LetterStatus::NotDone; text_len];
                new_vec[0] = LetterStatus::Doing;
                statuses_vec.set(new_vec);
            }
        })
    };

    let on_key_down = {
        let text = text.clone();
        let vec = statuses_vec.clone();
        let callback = wpm_callback.clone();
        Callback::from(move |event: KeyboardEvent| {
            // log!(event.clone());
            if vec.len() != text_len{
                let mut statuses = vec![LetterStatus::NotDone; text_len];
                if !text.is_empty(){
                    statuses = vec![LetterStatus::NotDone; text_len];
                    statuses[0] = LetterStatus::Doing;
                }
                vec.set(statuses);
            }
            log!("index: ", *current_index, " | card: ", *current_card_index, " | size: ", text_len);
            let input = event.key();
            if input == "Backspace" {
                if *current_index == 0 {
                    return;
                }
                let mut new_vec = vec![LetterStatus::NotDone; text_len];
                for (i, _) in vec.iter().enumerate() {
                    new_vec[i] = vec[i];
                }
                new_vec[*current_index] = LetterStatus::NotDone;
                new_vec[*current_index - 1] = LetterStatus::Doing;
                vec.set(new_vec);
                current_index.set(*current_index - 1);

                //update card_index
                if cards_position[*current_card_index].0 > *current_index{
                    current_card_index.set(*current_card_index-1);
                    log!("decremented card index: " , *current_card_index-1);
                    cards_context.dispatch(CardsContextAction::SetCardsIndex(*current_card_index-1));
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
            let mut new_vec = vec![LetterStatus::NotDone; text_len];
            for (i, _) in vec.iter().enumerate() {
                new_vec[i] = vec[i];
            }
            new_vec[*current_index] = LetterStatus::Done;
            if *current_index + 1 < text_len {
                new_vec[*current_index + 1] = LetterStatus::Doing;
                current_index.set(*current_index + 1);
            }
            log!(*current_index);
            let correct_char = text.chars().nth(*current_index).unwrap();
            let typed_char = input.chars().next().unwrap();
            match map.get(&correct_char){
                Some(char) => {
                    log!(correct_char.to_string() , (*char).to_string(), typed_char.to_string());
                    if *char != typed_char{
                        new_vec[*current_index] = LetterStatus::WronglyDone;
                    }
                },
                None => {
                    if typed_char != correct_char {
                        new_vec[*current_index] = LetterStatus::WronglyDone;
                    }
                }
            }
            vec.set(new_vec);
            if (*current_index) + 1 == text_len {
                let wpm = calculate_wpm(*start, &text, &vec);
                callback.emit(wpm);
                current_index.set(0);
                current_card_index.set(0);
                // vec.set(vec![]);
            }

            //update card_index
            if cards_position[*current_card_index].1 < *current_index{
                current_card_index.set(*current_card_index+1);
                log!("incremented card index: " , *current_card_index+1);
                cards_context.dispatch(CardsContextAction::SetCardsIndex(*current_card_index+1));
            }
        })
    };

    let style = style!(r#"
                    font-size: 2rem;
                    line-height: 3rem;
                    width: 66%;
                       "#).unwrap();

    if cards.is_empty() || statuses_vec.is_empty() || text.is_empty(){
        return html!(<div class={css!("font-size: 2rem;")} onkeydown={start_callback} tabindex={0}>{"press tab then any key to start.."}</div>);
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
        <div class={style} onkeydown={on_key_down} tabindex={0}>
            {letters}
        </div>
        )
    }
}
