extern crate chrono;
use chrono::*;

pub fn after(n: DateTime<UTC>) -> DateTime<UTC> {
    n + Duration::seconds(1_000_000_000)
}
