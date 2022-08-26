use yew::prelude::*;

struct SomeOtherModel {
    a_num: i32,
}

struct SomeModel {
    a_num: f64,
    a_bool: bool,
    a_string: String,
    an_array: [u8; 3],
    a_vec: Vec<u8>,
    other_model: SomeOtherModel,
}

impl Component for SomeModel {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            a_num: 22.1,
            a_bool: true,
            a_string: "hello".to_string(),
            an_array: [1, 2, 3],
            a_vec: vec!(4, 5, 6),
            other_model: SomeOtherModel {
                a_num: 4,
            },
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
       html! {
            <div>
                <p>{ self.a_num }</p>
                <p>{ self.a_bool }</p>
                <p>{ &self.a_string }</p>
                <ul>
                    { self.an_array.iter().collect::<Html>() }
                </ul>
                <ul>
                    { self.a_vec.iter().collect::<Html>() }
                </ul>
                <p>{ self.other_model.a_num }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<SomeModel>();
}
