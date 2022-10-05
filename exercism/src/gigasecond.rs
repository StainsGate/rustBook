use time::Duration;
use time::PrimitiveDateTime as DateTime;

pub fn after(start: DateTime) -> DateTime {
    start.saturating_add(Duration::new(1000000000, 0))
}
