use yew::prelude::*;

struct LetterComponent{
    color: string
}

enum Msg{
    AddOne,
}

impl Component for CounterComponent{
    type Message = Msg;
    type Properties = ();
    
    fn create(_ctx: &Context<Self>) -> Self{
        Self {count: 0}        
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count +=1;
                log::info!("hello");
                true
            } 
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let link: &html::Scope<CounterComponent> = _ctx.link();
        // todo!()
        html!{
            <div class="container">
                <p>{self.count}</p>
                <button onclick={link.callback(|_| Msg::AddOne)}>{"+1"}</button>
            </div>
        }
            
    }


}
