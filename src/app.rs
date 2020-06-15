use crate::components::LearnComponents;
use log::*;
use yew::prelude::*;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="main">
                <h1>{ "Learing yew" }</h1>
                <a href="https://yew.rs/docs/" target="blank">{"yew.rs"}</a>
                <a href="https://docs.rs/yew/0.16.2/yew/" target="blank">{"Yew docs"}</a>
                <LearnComponents />
            </div>
        }
    }
}
