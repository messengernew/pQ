use std::fs;
use std::process::Command;
use chrono_tz::Tz;

pub(crate) fn get_timezone() -> Option<Tz> {
    get_timezone_from_file()
        .or_else(|| get_timezone_from_env())
        .or_else(|| get_timezone_from_command())
}

fn get_timezone_from_file() -> Option<Tz> {
    let tz_file = "/etc/timezone";
    if let Ok(tz_string) = fs::read_to_string(tz_file) {
        tz_string.trim().parse::<Tz>().ok()
    } else {
        None
    }
}

fn get_timezone_from_env() -> Option<Tz> {
    if let Ok(tz_string) = std::env::var("TZ") {
        tz_string.parse::<Tz>().ok()
    } else {
        None
    }
}

fn get_timezone_from_command() -> Option<Tz> {
    if let Ok(output) = Command::new("timedatectl").arg("show").output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines() {
            if let Some(index) = line.find("Timezone=") {
                let tz_string = line[index + 9..].trim();
                return tz_string.parse::<Tz>().ok();
            }
        }
    }
    None
}