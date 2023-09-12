use std::path::Path;

use crate::utils::{extension, filename};

pub trait FilterCreator {
    fn create(self) -> Box<dyn Fn(&Path) -> bool>;
}

#[derive(Debug, Clone, Copy)]
struct Size {
    min: u64,
    max: u64,
}

impl FilterCreator for Size {
    fn create(self) -> Box<dyn Fn(&Path) -> bool> {
        Box::new(move |path: &Path| -> bool {
            let Ok(meta) = path.metadata() else {
                return false;
            };

            let filesize = meta.len();

            if filesize > self.max {
                return false;
            }

            if filesize < self.min {
                return false;
            }

            true
        })
    }
}

/// TODO
pub struct Regex(String);

#[derive(Debug, Clone)]
struct Date {
    before: Option<chrono::DateTime<chrono::Utc>>,
    after: Option<chrono::DateTime<chrono::Utc>>,
}

type Items = Option<Vec<String>>;

#[derive(Debug, Default, Clone)]
struct Name {
    contains: Items,
    starts_with: Items,
    ends_with: Items,
    has_extension: Items,
}

pub trait HasItem {
    type Item<'a>;
    fn has(&self, item: Self::Item<'_>) -> bool;
}

impl HasItem for Vec<String> {
    type Item<'a> = &'a str;

    fn has(&self, item: &str) -> bool {
        if self.is_empty() {
            return true;
        }
        contains(self, item)
    }
}

pub struct File {}

fn contains(items: &Vec<String>, item: &str) -> bool {
    items.iter().any(|f| f == item)
}
// fn
pub struct Filter(Vec<Box<dyn Fn(&Path) -> bool>>);

impl Filter {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, func: Box<dyn Fn(&Path) -> bool>) -> &mut Self {
        self.0.push(func);
        self
    }
}
