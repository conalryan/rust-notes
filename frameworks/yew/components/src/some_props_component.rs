use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct SomeProps {
    pub prop1: String,
    pub prop2: String,
}

pub struct SomePropsComponent;


impl Component for SomePropsComponent {
    type Message = ();
    type Properties = SomeProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="some-props-component">
                { "SomeProps works!" }
                {
                    format!("prop1: {}, prop2: {}", _ctx.props().prop1, _ctx.props().prop2)
                }
            </div>
        }
    }
}

