use super::props_card_variants::ListVariant;
use yew::html::ChildrenRenderer;
use yew::prelude::*;

pub struct PropsCard {
    props: Props,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: ChildrenRenderer<ListVariant>,
}

impl Component for PropsCard {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="props-card">
                {for self.props.children.iter()}
            </div>
        }
    }
}
