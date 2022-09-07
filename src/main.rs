use std::iter::Map;

use gloo::console::log;
use yew::prelude::*;
mod components;
use components::typing::Typing;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    let current_index = use_state(|| 0);
    // let map = use_state(|| Map<i32, bool>{});
    let text = "The quick brown fox jumps over the lazy dog";
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
    html!(
        <div onkeydown={on_key_down} tabindex={0}>
            <Typing text="The quick brown fox jumps over the lazy dog"/>
        </div>
        )
}
