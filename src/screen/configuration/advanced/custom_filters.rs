use std::num::NonZeroU32;

#[derive(Debug, Clone, Copy)]
struct Size {
    min: Option<NonZeroU32>,
    max: Option<NonZeroU32>,
}

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

#[derive(Debug, Default, Clone)]
struct Path {
    has_root: Items,
}

/// TODO
pub struct Regex(String);
