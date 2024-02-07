use std::path::Path;

pub mod date;
pub mod name;
pub mod size;
pub mod regex;

pub use size::Size;
pub use name::Name;

pub trait Filter {
    fn matches(&self, path: &Path) -> bool;
}
