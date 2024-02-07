/// https://github.com/squidowl/halloy/blob/9393792c43d705740ccddce561c52931ae098472/src/widget/collection.rs#L1C1-L1C1
use iced::widget::{Column, Row};
use iced::Element;

pub trait Collection<'a, Message, Theme, Renderer>: Sized {
    fn push_maybe(self, element: Option<impl Into<Element<'a, Message, Theme, Renderer>>>) -> Self;
}

impl<'a, Message, Theme, Renderer> Collection<'a, Message, Theme, Renderer> for Column<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn push_maybe(self, element: Option<impl Into<Element<'a, Message, Theme, Renderer>>>) -> Self {
        match element {
            Some(element) => self.push(element),
            None => self,
        }
    }
}

impl<'a, Message, Theme, Renderer> Collection<'a, Message,Theme, Renderer> for Row<'a, Message,Theme,Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn push_maybe(self, element: Option<impl Into<Element<'a, Message, Theme, Renderer>>>) -> Self {
        match element {
            Some(element) => self.push(element),
            None => self,
        }
    }
}
