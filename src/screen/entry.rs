use std::path::{Path, PathBuf};

use crate::app::Message;

use crate::screen::tracker_info::TrackerInfo;
use crate::utils::filename;
use crate::widget::helpers::{centered_container, centered_text, fill_container, text_adv};
use crate::widget::{self, Element};
use crate::{icon, theme};

use iced::widget::{button, checkbox, column, row, scrollable, Space};
use iced::{Alignment, Length};

#[derive(Default)]
pub struct Entry {
    pub selected: bool,
    pub path: PathBuf,
}

impl Entry {
    pub fn new(path: PathBuf) -> Self {
        Self {
            selected: false,
            path,
        }
    }
    pub fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    pub fn is_file(&self) -> bool {
        self.path.is_file()
    }

    pub fn filename(&self) -> &str {
        filename(&self.path)
    }
}

#[derive(Default)]
pub struct Entries {
    pub entries: Vec<Entry>,
}

impl Entries {
    pub fn contains(&self, path: &Path) -> bool {
        self.entries.iter().any(|x| &*x.path == path)
    }

    pub fn all_selected(&self) -> bool {
        !self.entries.is_empty() && self.total_selected() == self.entries.len()
    }

    pub fn add(&mut self, path: PathBuf) {
        if !self.contains(&path) {
            self.entries.push(Entry::new(path));
        }
    }

    pub fn add_multiple(&mut self, paths: Vec<PathBuf>) {
        paths.into_iter().for_each(|path| self.add(path));
    }

    pub fn total_selected(&self) -> usize {
        self.entries.iter().filter(|f| f.selected).count()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn select(&mut self, index: usize, selected: bool) {
        if let Some(entry) = self.entries.get_mut(index) {
            entry.selected = selected;
        }
    }

    pub fn select_all(&mut self, selected: bool) {
        self.entries
            .iter_mut()
            .for_each(|entry| entry.selected = selected);
    }

    pub fn take_selected(&mut self) -> Vec<Entry> {
        if self.all_selected() {
            return std::mem::take(&mut self.entries);
        }

        let mut selected = Vec::with_capacity(self.total_selected());
        let entries = &mut self.entries;

        let mut i = 0;

        while i < entries.len() {
            if entries[i].selected {
                selected.push(entries.remove(i));
            } else {
                i += 1;
            }
        }
        selected
    }

    pub fn take(&mut self) -> Vec<PathBuf> {
        match self.none_selected() {
            true => std::mem::take(&mut self.entries),
            false => self.take_selected(),
        }
        .into_iter()
        .map(|f| f.path)
        .collect()
    }

    pub fn delete_selected(&mut self, current_tracker_info: &mut TrackerInfo) {
        // clear the entries if everything is selected
        if self.all_selected() {
            self.entries.clear()
        } else {
            self.entries.retain(|entry: &Entry| {
                if current_tracker_info.matches_path(&entry.path) {
                    current_tracker_info.clear();
                }

                !entry.selected
            })
        }
    }

    pub fn files(&self) -> usize {
        self.entries.iter().filter(|f| f.is_file()).count()
    }

    pub fn invert(&mut self) {
        self.entries
            .iter_mut()
            .for_each(|f| f.selected = !f.selected)
    }

    pub fn none_selected(&self) -> bool {
        self.total_selected() == 0
    }

    pub fn get(&self, idx: usize) -> Option<&Path> {
        self.entries.get(idx).map(|f| f.path.as_ref())
    }

    pub fn view(&self, hovered: bool, show_gif: bool) -> Element<Message> {
        let entries = &self.entries;

        if entries.is_empty() {
            return centered_container(
                column![]
                    .push(centered_text("Drag and Drop"))
                    .push_maybe(show_gif.then(|| widget::animation::GIF.idle()).flatten())
                    .align_items(Alignment::Center),
            )
            .style(theme::Container::BlackHovered(hovered))
            .into();
        }

        fill_container(scrollable(row![
            column(entries.iter().enumerate().map(view_entry))
                .spacing(10)
                .padding(5),
            Space::with_width(15)
        ]))
        .style(theme::Container::BlackHovered(hovered))
        .padding(5)
        .into()
    }
}

fn view_entry((index, entry): (usize, &Entry)) -> Element<Message> {
    let check = checkbox("", entry.selected)
        .on_toggle(move |selected| Message::Select { index, selected })
        .style(theme::CheckBox::Entry);

    let filename = text_adv(entry.filename());

    let view = row![check, filename]
        .push_maybe(
            entry
                .is_dir()
                .then(|| row![Space::with_width(Length::Fill), icon::folder().size(14)]),
        )
        .spacing(4)
        .padding(1)
        .align_items(Alignment::Center);

    button(view)
        .width(Length::Fill)
        .on_press(Message::Probe(index))
        .padding(4)
        .style(theme::Button::Entry)
        .into()
}
