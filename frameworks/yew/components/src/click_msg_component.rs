use yew::prelude::*;

pub enum ClickMsg {
    Click,
    ClickNoUpdate,
}

pub struct ClickMsgComponent {
    show_message: bool,
}

impl Component for ClickMsgComponent {
    type Message = ClickMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { 
            show_message: false,
        }
    }

    /// Update lifecycle hook
    /// Return true to rerender the component.
    /// Use _ctx.link().callback(...) to pass messages to the component and possibly rerender.
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // or Self::Message::Click
            ClickMsg::Click => {
                // Alter state then rerender.
                if self.show_message {
                    self.show_message = false;
                } else {
                    self.show_message = true;
                }
                true
            },
            ClickMsg::ClickNoUpdate => {
                self.show_message = false;
                false
            }
        }
    }

    fn view (&self, _ctx: &Context<Self>) -> Html {
        let onclick = _ctx.link().callback(|_|ClickMsg::Click);
        let onclick_no_update = _ctx.link().callback(|_|ClickMsg::ClickNoUpdate);
        html! {
            <div class="click-msg-component">
                <button {onclick}>{ "Click and update aka rerender" }</button>
                if self.show_message {
                    <div>
                        { "Must be rerendered to see this message" }
                    </div>
                }
                <button onclick={onclick_no_update}>{ "Click and don't render (try to hide message)" }</button>
            </div>
        }
    }
}
