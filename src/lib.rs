use wasm_bindgen::prelude::*;
use cron::Schedule;
use std::str::FromStr;
use chrono::{DateTime, Utc};

#[wasm_bindgen]
pub fn convert_epoch_to_date(epoch: i64) -> String {
    // Create a DateTime<Utc> directly from the epoch timestamp
    let datetime: DateTime<Utc> = DateTime::<Utc>::from_timestamp(epoch, 0).unwrap();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}


#[wasm_bindgen]
pub fn parse_cron_expression(cron_expr: &str) -> String {
    match Schedule::from_str(cron_expr) {
        Ok(schedule) => {
            let upcoming_times = schedule.upcoming(Utc).take(5).map(|dt| dt.to_string()).collect::<Vec<_>>().join("\n");
            upcoming_times
        },
        Err(e) => format!("Error parsing cron expression: {}", e)
    }
}

