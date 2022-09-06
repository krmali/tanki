use yew::prelude::*;

#[derive(PartialEq)]
pub enum Status{
    Done,
    NotDone,
    WronglyDone,
    Doing
}

#[derive(Properties, PartialEq)]
pub struct LetterProps{
    pub status: Status,
    pub character: char,
}

#[function_component(Letter)]
pub fn letter(LetterProps{status, character} : &LetterProps) -> Html {
    html! {
        <span style={match status{
            Status::Done => "color: green;",
            Status::Doing => "text-decoration: underline;",
            Status::WronglyDone => "color: red",
            _ => ""
        }}>{character}</span>
    }
}
