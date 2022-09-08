use super::letter::{Letter, LetterStatus};
use gloo::console::log;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TypingProps {
    pub text: String,
}

impl Default for TypingProps {
    fn default() -> Self {
        TypingProps {
            text: "The quick brown fox jumps over the lazy dog".to_owned(),
        }
    }
}

#[function_component(Typing)]
pub fn typing(TypingProps { text }: &TypingProps) -> Html {
    let current_index = use_state(|| 0);
    let mut statuses = vec![LetterStatus::NotDone; text.len()];
    statuses[0] = LetterStatus::Doing;
    let vec = use_state(|| statuses);
    // let on_key_down = Callback::from(|event: KeyboardEvent| {
    //     let target = event.target();
    //     let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
    //     // let input = target.and_then(|t| t.into());
    //     // match target{
    //     //     Some(trgt) => log!(trgt.char_code()),
    //     //     None => log!("no key down")
    //     // };
    //     log!(input.map(|input| input.value()))
    // });
    let on_key_down = {
        let current_index = current_index.clone();
        let text = text.clone();
        let vec = vec.clone();
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
            if input.bytes().nth(0) == text.bytes().nth(*current_index) {
                let mut new_vec = vec![LetterStatus::NotDone; text.len()];
                for (i, _) in vec.iter().enumerate() {
                    new_vec[i] = vec[i];
                }
                new_vec[*current_index] = LetterStatus::Done;
                if *current_index + 1 < text.len() {
                    new_vec[*current_index + 1] = LetterStatus::Doing;
                    current_index.set(*current_index + 1);
                }
                vec.set(new_vec);
            } else {
                let text_len = text.len() -1;
                if (*current_index) == text_len {
                    return;
                }
                let mut new_vec = vec![LetterStatus::NotDone; text.len()];
                for (i, _) in vec.iter().enumerate() {
                    new_vec[i] = vec[i];
                }
                new_vec[*current_index] = LetterStatus::WronglyDone;
                if *current_index + 1 < text.len() {
                    new_vec[*current_index + 1] = LetterStatus::Doing;
                    current_index.set(*current_index + 1);
                }
                vec.set(new_vec);
            }
        })
    };

    let letters: Html = text
        .chars()
        .enumerate()
        .map(|(index, letter)| {
            html!(
            <Letter status={(*vec)[index]} character={letter}/>
            // <Letter status={LetterStatus::Done} character={letter}/>
            )
        })
        .collect();
    html!(
    <div onkeydown={on_key_down} tabindex={0}>
        {letters}
        <p>{*current_index}</p>
    </div>
    )
}
