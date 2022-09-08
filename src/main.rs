use yew::prelude::*;
mod components;
use components::typing::Typing;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    let text = "The quick brown fox jumps over the lazy dog";
    html!(
        <div>
            <Typing text={text}/>
        </div>
        )
}
