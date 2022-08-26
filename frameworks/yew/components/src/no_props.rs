use yew::prelude::*;

pub struct NoPropsComponent;

impl Component for NoPropsComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            { "This component has no properties!" }
        }
    }
}

