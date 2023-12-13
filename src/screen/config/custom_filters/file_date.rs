use std::fmt::Display;

use crate::widget::helpers::control;
use crate::widget::Element;
use crate::{icon, theme};
use crate::{theme::TextInputStyle, widget::helpers::text_icon};

use chrono::NaiveDate;
use data::config::filters::date::{Condition, Date};

use iced::widget::tooltip::Position;
use iced::widget::{
    button, checkbox, column, horizontal_rule, pick_list, row, slider, text, text_input, tooltip,
};

use iced_aw::{date_picker::Date as icedDate, helpers::date_picker};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    DatePicker(DateKind),
    CancelPicker,
    SubmitDate {
        kind: DateKind,
        date: iced_aw::core::date::Date,
    },
    Condition(Condition),
}

#[derive(Debug, Default)]
pub struct DateFilter {
    inner: Date,
    date_picker: DateKind,
    show_date_picker: bool,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum DateKind {
    #[default]
    Before,
    After,
}

impl DateFilter {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::DatePicker(datekind) => {
                self.date_picker = datekind;
                self.show_date_picker = true;
            },
            Message::CancelPicker => self.show_date_picker = false,
            Message::SubmitDate { date, kind } => {
                match kind {
                    DateKind::Before => self.inner.before = Some(date.into()),
                    DateKind::After => self.inner.after = Some(date.into()),
                }

                self.show_date_picker = false;
            },
            Message::Condition(condition) => self.inner.condition = condition,
        }
    }

    pub fn view(&self) -> Element<Message> {
        // IDEA: buttons could have a tooltip showing the date&time in more detail
        // TODO: buttons must bring up date and time picker, should be an overlay
        let after_date = self.inner.after();
        let after_date_long = after_date.format("%A, %-d %B, %C%y");

        let before_date = self.inner.before();
        let before_date_long = before_date.format("%A, %-d %B, %C%y");

        let settings = row![
            pick_list(Condition::ALL, Some(self.inner.condition), Message::Condition),
            tooltip(
                button(text(after_date.to_string()))
                    .on_press(Message::DatePicker(DateKind::After))
                    .padding(8)
                    .style(theme::Button::Dark),
                after_date_long,
                Position::Bottom,
            )
            .padding(6)
            .style(theme::Container::Frame),
            "-",
            tooltip(
                button(text(before_date))
                    .on_press(Message::DatePicker(DateKind::Before))
                    .padding(8)
                    .style(theme::Button::Dark),
                before_date_long,
                Position::Bottom,
            )
            .padding(6)
            .style(theme::Container::Frame),
        ]
        .align_items(iced::Alignment::Center)
        .spacing(8);

        control(
            "File Date",
            settings,
        )
        .into()
    }
}

// pub fn show_date_picker<'a>(
//     filter: &'a DateFilter,
//     underlay: impl Into<Element<'a, Message>>,
// ) -> Element<'a, Message> {
//     let date = match filter.date_picker {
//         DateKind::Before => filter.inner.before(),
//         DateKind::After => filter.inner.after(),
//     };

//     let show = filter.show_date_picker;
//     let kind = filter.date_picker;

//     date_picker(show, date, underlay, Message::CancelPicker, move |date| Message::SubmitDate {
//         kind,
//         date,
//     })
//     .into()
// }
