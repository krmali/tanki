use yew::prelude::*;
use super::letter::{Letter, Status};

#[derive(Properties, PartialEq)]
pub struct TypingProps{
    text: String,
}

impl Default for TypingProps{
    fn default() -> Self {
        TypingProps { text: "The quick brown fox jumps over the lazy dog".to_owned() }
    }
}

#[function_component(Typing)]
pub fn typing(TypingProps { text }: &TypingProps) -> Html {

    text.chars().map(|letter| html!(
            <Letter status={Status::Doing} character={letter}/>
            )).collect()
}
