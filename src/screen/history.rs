//! Keeps track of previously ripped modules and its configuration.

use crate::logger::history;

#[derive(Debug, Default)]
pub struct History(data::History);

