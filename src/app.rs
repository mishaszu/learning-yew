use crate::components::{button1, button2, input};
use button1::Button1;
use button2::Button2;
use input::Input;
use log::*;
use yew::prelude::*;

#[derive(Debug)]
pub struct Test {
    pub value1: i32,
    pub value2: i32,
}

pub struct App {
    link: ComponentLink<Self>,
    value: i32,
    text: String,
}

pub enum Msg {
    AddOne,
    MinusOne,
    CustomValue(String),
    StructValue(Test),
    TextChange(String),
    Nope,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
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
            Msg::StructValue(value) => {
                info!("Log from update method: {:?}", value);
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
        let oninput = self
            .link
            .callback(|event: InputData| Msg::CustomValue(event.value));
        let click: Callback<MouseEvent> = self.link.callback(|_| Msg::Nope);

        let click2: Callback<MouseEvent> = self
            .link
            .callback(|_| {
                Msg::StructValue(Test {
                    value1: 10,
                    value2: 100,
                })
            })
            .reform(|value| {
                info!("{:?}", value);
                ()
            });

        html! {
            <div class="main">
                <h1>{ "Learing yew" }</h1>
                <a href="https://yew.rs/docs/" target="blank">{"yew.rs"}</a>
                <a href="https://docs.rs/yew/0.16.2/yew/" target="blank">{"Yew docs"}</a>
                <div class="section">
                    <h2>{"Passing Callback as props"}</h2>
                    <div class="action-wrapper">
                        <Button1 onclick=self.callback_click() value="Click from callback" />
                        <Button2 onclick=self.event_click() value="Click from mouse event callback"/>
                    </div>
                </div>
                <div class="section">
                    <div class="action-wrapper">
                        <div class="small-margin">
                            <h2>{"Passing Callback directly to dom element"}</h2>
                            <button class="button" onclick=self.simple_callback()>{ "Click from simple callback" }</button>
                        </div>
                        <div class="small-margin">
                            <h2>{"Passing Callback as props"}</h2>
                            // this one will dispatch 2 messages: MinusOne & Nope
                            <button class="button" onclick=self.callback_to_reform().reform(|_| Msg::Nope)>{ "Test from reform click"}</button>
                        </div>
                    </div>
                </div>
                <div class="section">
                    <h2>{format!("Passing callback to input, here is result: {}", &self.text)}</h2>
                    <div class="action-wrapper">
                        <Input oninput=self.oninput() text=&self.text />
                    </div>
                </div>
            </div>
        }
    }
}

// Implement methods for callback passed as props
impl App {
    fn callback_click(&self) -> Callback<String> {
        self.link.callback(Msg::CustomValue)
    }
    fn event_click(&self) -> Callback<MouseEvent> {
        Callback::from(|_| info!("Click from event"))
    }
}

// Implemment methods for callback directly in DOM elements
impl App {
    fn callback_to_reform(&self) -> Callback<Msg> {
        self.link.callback(|_| Msg::MinusOne)
    }
    fn simple_callback(&self) -> Callback<MouseEvent> {
        self.link.callback(|_| Msg::AddOne)
    }
}

// Implement methods for input callback
impl App {
    fn oninput(&self) -> Callback<InputData> {
        self.link
            .callback(|event: InputData| Msg::TextChange(event.value))
    }
}
