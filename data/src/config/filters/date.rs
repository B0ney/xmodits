use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Date {
    before: DateTime<Utc>,
    after: DateTime<Utc>,
}
