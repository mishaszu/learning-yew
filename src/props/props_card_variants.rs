use super::empty_props::EmptyProps;
use yew::html::NodeRef;
use yew::prelude::*;
use yew::virtual_dom::{VChild, VComp, VNode};

#[derive(Clone, PartialEq)]
pub enum Variants {
    Empty(<EmptyProps as Component>::Properties),
}

impl From<()> for Variants {
    fn from(_: ()) -> Self {
        Variants::Empty(())
    }
}

#[derive(Clone, PartialEq)]
pub struct ListVariant {
    props: Variants,
}

impl<CHILD> From<VChild<CHILD>> for ListVariant
where
    CHILD: Component,
    CHILD::Properties: Into<Variants>,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        ListVariant {
            props: vchild.props.into(),
        }
    }
}

impl Into<VNode> for ListVariant {
    fn into(self) -> VNode {
        match self.props {
            Variants::Empty(props) => {
                VComp::new::<EmptyProps>(props, NodeRef::default(), None).into()
            }
        }
    }
}
