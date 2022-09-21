use yew::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum LetterStatus{
    Done,
    NotDone,
    WronglyDone,
    Doing
}

#[derive(Properties, PartialEq)]
pub struct LetterProps{
    pub status: LetterStatus,
    pub character: char,
}

#[function_component(Letter)]
pub fn letter(LetterProps{status, character} : &LetterProps) -> Html {
    html! {
        <span style={match status{
            LetterStatus::Done => "color: var(--text-color);",
            LetterStatus::Doing => "text-decoration: underline;",
            LetterStatus::WronglyDone => "color: var(--error-color)",
            _ => "color: var(--sub-color)"
        }}>{character}</span>
    }
}
