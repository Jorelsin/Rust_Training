use chrono::{DateTime, Utc, Duration};
// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {

let start_date = start + Duration::seconds(1000000000);

return start_date;
}
