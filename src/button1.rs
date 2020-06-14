use yew::prelude::*;

pub struct Button1 {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onclick: Callback<String>,
    pub value: String,
}

impl Component for Button1 {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <button onclick=self.onclick()>{&self.props.value}</button>
        }
    }
}

impl Button1 {
    fn onclick(&self) -> Callback<MouseEvent> {
        // We have to transform callback from props that expect enum constructor into callback from
        // mouse event. Returned value from mouse event callback will be passed into original
        // callback and will returned filled enum
        self.props.onclick.reform(|_| String::from("Test"))
    }
}
