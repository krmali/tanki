use yew::prelude::*;
use super::letter::{Letter, Status};
use gloo::console::log;

#[derive(Properties, PartialEq)]
pub struct TypingProps{
    pub text: String,
}

impl Default for TypingProps{
    fn default() -> Self {
        TypingProps { text: "The quick brown fox jumps over the lazy dog".to_owned() }
    }
}

#[function_component(Typing)]
pub fn typing(TypingProps { text }: &TypingProps) -> Html {
    let current_index = use_state(|| 0);
    // let vec : [bool; text.len()] = use_state(|| [0; text.len()]);
    let vec : UseStateHandle<Vec<bool>> = use_state(|| Vec::with_capacity(text.len()));
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
    let on_key_down = Callback::from(move |event: KeyboardEvent| {
        // let current_index = current_index.clone();
        log!(event.clone());
        let input = event.key();
        if input == "Backspace"{
            current_index.set(*current_index-1);
            return;
        }
        if input.len() > 1 {return;}
        if input.bytes().nth(0) == text.bytes().nth(*current_index){
            current_index.set(*current_index+1);
        }
    });

    let letters : Html = text.chars().map(|letter| html!(
                    <Letter status={Status::Doing} character={letter}/>
                    )).collect();
    html!(
        <div onkeydown={on_key_down} tabindex={0}>
            {letters}
        </div>
        )
}
