use yew::prelude::*;
mod components;
use components::typing::Typing;

fn main() {
    yew::Renderer::<Typing>::new().render();
}
