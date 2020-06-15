use yew::html::Children;
use yew::prelude::*;

// Card component have to be in props because it have to handle props in the was that will allow
// passing props from parent to children and reciving proper response in parent when Callback
// called from children
pub struct Card {
    children: Children,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    children: Children,
}

impl Component for Card {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            children: props.children,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="card">
                {self.children.render()}
            </div>
        }
    }
}
