use yew::prelude::*;

use crate::click_msg_component::ClickMsgComponent;
use crate::no_props_component::NoPropsComponent;
use crate::some_model_component::SomeModelComponent;
use crate::some_props_component::{SomePropsComponent, SomeProps};

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App
    }

    /// Describe how component should be rendered in DOM.
    /// Similar to JSX style
    /// Allows shorthand syntax for properties, similar to Svelte,
    /// where instead of writing onclick={onclick}, you can just write {onclick}.
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let props = SomeProps {
            prop1: "Hello".to_owned(),
            prop2: "World".to_owned(),
        };

        html! {
        <div class="app">
            <NoPropsComponent />

            <SomeModelComponent />

            <SomePropsComponent prop1="lorem" prop2="ipsum" />

            <SomePropsComponent ..props.clone() />

            <SomePropsComponent prop2="lorem" ..props />

            <ClickMsgComponent />
        </div>
        }
    }
}

