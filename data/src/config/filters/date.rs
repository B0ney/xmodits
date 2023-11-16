use std::{fs::metadata, str::FromStr};

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Date {
    pub condition: Condition,
    pub before: Option<NaiveDate>,
    pub after: Option<NaiveDate>,
}

impl Date {
    pub fn before(&self) -> NaiveDate {
        // NaiveDate
        self.before
            .unwrap_or_else(|| chrono::Utc::now().naive_utc().date())
    }

    pub fn after(&self) -> NaiveDate {
        self.after
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(1987, 1, 1).expect("A valid date"))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Condition {
    #[default]
    Created,
    Modified,
}

impl Condition {
    pub const ALL: &[Self] = &[Self::Created, Self::Modified];
}

impl std::fmt::Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Condition::Created => "Created",
                Condition::Modified => "Modified",
            }
        )
    }
}
