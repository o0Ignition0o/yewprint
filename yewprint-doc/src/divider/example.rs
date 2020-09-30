use yew::prelude::*;
use yewprint::{Button, Divider};

pub struct Example {
    link: ComponentLink<Self>,
}

pub enum Msg {
    ToggleCollapse,
}

impl Component for Example {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Button>{"File"}</Button>
                <Button>{"Edit"}</Button>
                <Divider />
                <Button>{"Create"}</Button>
                <Button>{"Delete"}</Button>
                <Divider />
                // <Button icon=IconName::Add />
                // <Button icon=IconName::Remove />
            </div>
        }
    }
}
