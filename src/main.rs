use gloo::net::http::Request;
use yew::prelude::*;
use gloo::console::log;
mod components;
mod data; 
use components::typing::Typing;
use data::card::Card;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    let cards : UseStateHandle<Vec<Card>>= use_state(Vec::new); 
    let wpm_state : UseStateHandle<f64> = use_state(|| 0.0);
    // let text = "The quick brown fox jumps over the lazy dog";
    // let text = "This makes sense because proof-reading and correcting errors takes up more time than simply typing a passage correctly in the first place. Less mistakes also means less chance for errors being missed during proof-reading and making their way into the final product.";
    let text = "think you number early back same a through should or all these mean consider here they might show even state group eye thing not there";

    let wpm_callback = {
        let wpm_state = wpm_state.clone();
        Callback::from(move |wpm: f64| {
            wpm_state.set(wpm);
    })};

    {
        // let cards = cards.clone();
        use_effect_with_deps( move |_| {
            // let cards = cards.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_cards : Vec<Card> = Request::get("assets/cards.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                // cards.set(fetched_cards);
                // let v = vec![1,2,3];
                // log!(v);
                // let card = fetched_cards[0];
                // log!(format!("{}", fetched_cards));
                // log!("{:#?}", fetched_cards[0]);

            });
            || ()
        }, ());
    }

    html!(
        <div>
            <Typing text={text} callback={wpm_callback}/>
            <div>
                {*wpm_state}
            </div>
        </div>
        )
}
