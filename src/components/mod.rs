pub mod button1;
pub mod button2;
pub mod input;

use button1::Button1;
use button2::Button2;
use input::Input;

use log::*;
use yew::prelude::*;

pub struct LearnComponents {
    link: ComponentLink<Self>,
    value: i32,
    text: String,
}

pub enum Msg {
    AddOne,
    MinusOne,
    CustomValue(String),
    TextChange(String),
    Nope,
}

impl Component for LearnComponents {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            text: String::from(""),
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
            Msg::MinusOne => {
                self.value -= 1;
                info!("Minut one dispatched");
                true
            }
            Msg::CustomValue(value) => {
                info!("Log from update method: {}", value);
                false
            }
            Msg::TextChange(text) => {
                self.text = text;
                true
            }
            Msg::Nope => false,
        }
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
            <div class="learn-component">
                <h2>{"Creating simple components"}</h2>
                <div class="card card-custom-width">
                    <h3>{"Passing Callback as props"}</h3>
                    <div class="action-wrapper">
                        <Button1 onclick=self.callback_click() value="Click from callback" />
                        <Button2 onclick=self.event_click() value="Click from mouse event callback"/>
                    </div>
                </div>
                <div class="card card-custom-width">
                    <h3>{format!("Counter {}", self.value)}</h3>
                    <div class="action-wrapper">
                        <div class="small-margin">
                            <h3>{"Passing Callback directly to dom element"}</h3>
                            <button class="button" onclick=self.simple_callback()>{ "Click from simple callback" }</button>
                        </div>
                        <div class="small-margin">
                            <h3>{"Passing Callback as props"}</h3>
                            // this one will dispatch 2 messages: MinusOne & Nope
                            <button class="button" onclick=self.callback_to_reform().reform(|_| Msg::Nope)>{ "Test from reform click"}</button>
                        </div>
                    </div>
                </div>
                <div class="card card-custom-width">
                    <h3>{format!("Passing callback to input, here is result: {}", &self.text)}</h3>
                    <div class="action-wrapper">
                        <Input oninput=self.oninput() text=&self.text />
                    </div>
                </div>
            </div>
        }
    }
}

// Implement methods for callback passed as props
impl LearnComponents {
    fn callback_click(&self) -> Callback<String> {
        self.link.callback(Msg::CustomValue)
    }
    fn event_click(&self) -> Callback<MouseEvent> {
        Callback::from(|_| info!("Click from event"))
    }
}

// Implemment methods for callback directly in DOM elements
impl LearnComponents {
    fn callback_to_reform(&self) -> Callback<Msg> {
        self.link.callback(|_| Msg::MinusOne)
    }
    fn simple_callback(&self) -> Callback<MouseEvent> {
        self.link.callback(|_| Msg::AddOne)
    }
}

// Implement methods for input callback
impl LearnComponents {
    fn oninput(&self) -> Callback<InputData> {
        self.link
            .callback(|event: InputData| Msg::TextChange(event.value))
    }
}
