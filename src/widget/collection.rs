/// https://github.com/squidowl/halloy/blob/9393792c43d705740ccddce561c52931ae098472/src/widget/collection.rs#L1C1-L1C1
use iced::widget::{Column, Row};
use iced::Element;

pub trait Collection<'a, Message, Renderer>: Sized {
    fn push_maybe(self, element: Option<impl Into<Element<'a, Message, Renderer>>>) -> Self;
}

impl<'a, Message, Renderer> Collection<'a, Message, Renderer> for Column<'a, Message, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn push_maybe(self, element: Option<impl Into<Element<'a, Message, Renderer>>>) -> Self {
        match element {
            Some(element) => self.push(element),
            None => self,
        }
    }
}

impl<'a, Message, Renderer> Collection<'a, Message, Renderer> for Row<'a, Message, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn push_maybe(self, element: Option<impl Into<Element<'a, Message, Renderer>>>) -> Self {
        match element {
            Some(element) => self.push(element),
            None => self,
        }
    }
}
