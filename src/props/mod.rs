use log::info;
use yew::prelude::*;

pub mod card;
pub mod empty_props;
pub mod props_card;
pub mod props_card_variants;
pub mod struct_like;

use card::Card;
use empty_props::EmptyProps;
use props_card::PropsCard;

pub struct LearnProps {
    link: ComponentLink<Self>,
}

impl Component for LearnProps {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
            <div class="learn-component-props">
                <h2>{"Creating simple components"}</h2>
                <div class="card-props card-custom-width">
                    <h3>{"Passing Callback to element that is Children props"}</h3>
                    <Card>
                        <button class="button" onclick=self.create_simple_callback()>{"Click in Wrapper"}</button>
                    </Card>
                </div>
                <div class="card-props card-custom-width">
                    <h3>{"Passing element with no props to card with Children"}</h3>
                    <Card>
                        <EmptyProps />
                    </Card>
                    <h3>{"Passing element with no props to card with ChildrenWithProps"}</h3>
                    <PropsCard>
                        <EmptyProps />
                    </PropsCard>
                    <h3>{"Passing vec of elements with no props to card with ChildrenWithProps"}</h3>
                    <PropsCard>
                        <EmptyProps />
                        <EmptyProps />
                        <EmptyProps />
                    </PropsCard>
                </div>
            </div>
        }
    }
}

impl LearnProps {
    fn create_simple_callback(&self) -> Callback<MouseEvent> {
        self.link.callback(|_| {
            info!("props simple callback");
            ()
        })
    }
}
