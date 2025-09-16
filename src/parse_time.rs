use chrono::NaiveTime;

pub fn parse_time(time: String) -> Result<NaiveTime, String> {
    match NaiveTime::parse_from_str(&time, "%H:%M") {
        Ok(time) => Ok(time),
        Err(_) => Err(format!("Failed to parse time")),
    }
}
