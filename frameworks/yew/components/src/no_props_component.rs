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
            // If you don't wrap in div then you'll have text you can't target with css
            <div class="no-props-component">
                { "This component has no properties!" }
            </div>
        }
    }
}

