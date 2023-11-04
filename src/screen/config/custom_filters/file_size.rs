use crate::widget::helpers::control;
use crate::widget::Element;
use data::config::filters::Size;
use iced::widget::column;

pub enum Message {
    SetMin(u64),
    SetMax(u64),
}

pub fn view<'a>(filter: &Size) -> Element<'a, Message> {
    control("File Size", column![]).into()
}

pub fn update(filter: &mut Size, msg: Message) {
    match msg {
        Message::SetMin(min) => filter.min = min,
        Message::SetMax(max) => filter.max = max,
    }
}
