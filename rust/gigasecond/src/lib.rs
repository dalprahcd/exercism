use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    const ONE_GIGA_SECOND: i64 = 1_000_000_000;
    start + time::Duration::seconds(ONE_GIGA_SECOND)
}
