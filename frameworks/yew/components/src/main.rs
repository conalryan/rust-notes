use yew::prelude::*;

mod app;
mod click_msg_component;
mod no_props_component;
mod some_model_component;
mod some_props_component;

use app::App;

fn main() {
    yew::start_app::<App>();
}
