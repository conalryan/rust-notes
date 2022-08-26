use yew::prelude::*;

use crate::no_props::NoPropsComponent;
use crate::some_model::SomeModel;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <>
            <NoPropsComponent />
           
            <SomeModel />
        </>
        }
    }
}

