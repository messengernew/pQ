use std::process;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use serde::Serialize;
use serde_json;
use chrono_tz::Tz;
use crate::timezone::get_timezone;

#[derive(Serialize)]
struct ExitInfo {
    message: String,
    time: String,
    code: i32,
}

impl ExitInfo {
    fn new(message: &str, code: i32) -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let time = format_time(now);
        ExitInfo {
            message: message.to_string(),
            time,
            code,
        }
    }
}

#[allow(deprecated)]
fn format_time(seconds: u64) -> String {
    let datetime = NaiveDateTime::from_timestamp(seconds as i64, 0);

    if let Some(tz) = get_timezone() {
        let datetime_tz: DateTime<Tz> = tz.from_utc_datetime(&datetime);
        datetime_tz.format("%H:%M | %d-%m-%Y").to_string()
    } else {
        let datetime_local: DateTime<Local> = Local.from_utc_datetime(&datetime);
        datetime_local.format("%H:%M | %d-%m-%Y").to_string()
    }
}

pub struct ExitWrapper(i32);

impl ExitWrapper {
    pub(crate) fn msg(self, message: &str) {
        let exit_info = ExitInfo::new(message, self.0);
        let json_output = serde_json::to_string_pretty(&exit_info).unwrap();
        println!("ExitInfo {}", json_output);
        process::exit(self.0);
    }
}

pub(crate) fn exit(code: i32) -> ExitWrapper {
    ExitWrapper(code)
}