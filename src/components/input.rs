use yew::prelude::*;

pub struct Input {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub oninput: Callback<InputData>,
    pub text: String,
}

impl Component for Input {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        if _props.text == self.props.text {
            false
        } else {
            true
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <input class="input" placeholder="text input" value=self.props.text oninput=&self.props.oninput />
        }
    }
}
