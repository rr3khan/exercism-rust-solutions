use time::PrimitiveDateTime as DateTime;
use time::OffsetDateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let gigasecond = 1e9 as i64;
    let dateInSeconds = start.assume_utc().unix_timestamp() as i64;
    let totalSeconds = gigasecond + dateInSeconds;

    let offset_date_time = OffsetDateTime::from_unix_timestamp(totalSeconds).unwrap();
    let endDateTime = DateTime::new(offset_date_time.date(), offset_date_time.time());

    return endDateTime;
}
