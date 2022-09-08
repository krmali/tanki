use yew::prelude::*;
mod components;
use components::typing::Typing;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    let wpm_state : UseStateHandle<f64> = use_state(|| 0.0);
    // let text = "The quick brown fox jumps over the lazy dog";
    let text = "This makes sense because proof-reading and correcting errors takes up more time than simply typing a passage correctly in the first place. Less mistakes also means less chance for errors being missed during proof-reading and making their way into the final product.";
    let wpm_callback = {
        let wpm_state = wpm_state.clone();
        Callback::from(move |wpm: f64| {
            wpm_state.set(wpm);
    })};
    html!(
        <div>
            <Typing text={text} callback={wpm_callback}/>
            <div>
                {*wpm_state}
            </div>
        </div>
        )
}
