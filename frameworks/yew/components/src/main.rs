use yew::prelude::*;

mod app;
mod no_props;
mod some_model;

use app::App;

fn main() {
    yew::start_app::<App>();
}
