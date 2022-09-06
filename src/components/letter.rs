use yew::prelude::*;

enum Status{
    Done,
    NotDone,
    WronglyDone
}

struct LetterStatus{
    status: Status,
    character: char,
}

#[function_component(Letter)]
fn letter(LetterStatus) -> Html {
    html! {
        <h1>{ "Hello World!" }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
