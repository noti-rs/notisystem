// NOTE: THIS IS A TEMPORARY SOLUTION, THE ENTIRE FILE WILL BE REFACTORED

use std::fs;
use std::path::Path;

static BATTERY_PATH: &str = "/sys/class/power_supply/BAT0/"; // TODO: find and use users main battery

fn read_battery_file<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    fs::read_to_string(path).map(|s| s.trim().to_string())
}

fn read_battery_value<P: AsRef<Path>>(path: P) -> std::io::Result<u64> {
    fs::read_to_string(path).map(|s| s.trim().parse::<u64>().expect("Failed to parse value"))
}

fn calculate_time_to_full() -> Option<f64> {
    let energy_now = read_battery_value(format!("{}/energy_now", BATTERY_PATH)).unwrap();
    let energy_full = read_battery_value(format!("{}/energy_full", BATTERY_PATH)).unwrap();
    let power_now = read_battery_value(format!("{}/power_now", BATTERY_PATH)).unwrap();

    if power_now > 0 {
        Some(((energy_full - energy_now) as f64 / power_now as f64) * 3600.0)
    } else {
        None
    }
}

fn calculate_time_to_empty() -> Option<f64> {
    let energy_now = read_battery_value(format!("{}/energy_now", BATTERY_PATH)).unwrap();
    let power_now = read_battery_value(format!("{}/power_now", BATTERY_PATH)).unwrap();

    if power_now > 0 {
        Some((energy_now as f64 / power_now as f64) * 3600.0)
    } else {
        None
    }
}

pub fn get_formatted_time_to() -> anyhow::Result<String> {
    let status = read_battery_file(format!("{}/status", BATTERY_PATH))?;

    dbg!(&status);

    let time_to = match status.as_str() {
        "Charging" => match calculate_time_to_full() {
            Some(t) => format!(
                "Time to full: {}",
                crate::util::datetime::format_seconds(t as u64)
            ),
            None => format!("Unable to calculate time to full"),
        },
        "Discharging" => match calculate_time_to_empty() {
            Some(t) => format!(
                "Time to empty: {}",
                crate::util::datetime::format_seconds(t as u64)
            ),
            None => format!("Unable to calculate time to empty"),
        },
        "Full" => format!("Battery is fully charged."),
        "Not charging" => format!("Battery is not charging."),
        _ => format!("Battery status is unknown or not supported."),
    };

    Ok(time_to)
}
