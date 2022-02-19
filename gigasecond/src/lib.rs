use std::time::Duration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let duration = Duration::new(1_000_000_000_u64, 0);
    let new_start = start + duration;
    return new_start;
}
