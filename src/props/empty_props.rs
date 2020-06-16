use yew::prelude::*;

pub struct EmptyProps {}

impl Component for EmptyProps {
    type Message = ();
    type Properties = ();
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
            <p>{"Here is simple elemenet with no props"}</p>
        }
    }
}
