use chrono::{DateTime, TimeZone, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    return Utc.timestamp_millis(start.timestamp_millis() + 1_000_000_000_000);
}
