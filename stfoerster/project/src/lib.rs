use chrono::{TimeZone, DateTime, Utc};
use chrono_tz::Europe::Berlin;
use chrono_tz::Tz;

use shell_words;
use std::collections::HashMap;

pub mod math;
pub mod dto;

pub fn prt_red(txt: &str) {
    println!("\x1b[1;{}m{}\x1b[0m", 31, txt);
}

pub fn prt_green(txt: &str) {
    println!("\x1b[1;{}m{}\x1b[0m", 32, txt);
}

pub fn prt_blue(txt: &str) {
    println!("\x1b[1;{}m{}\x1b[0m", 34, txt);
}

/// Gibt die aktuelle Zeit in der Zeitzone Europe/Berlin zurück
pub fn now() -> DateTime<Tz> {
    let utc_now = Utc::now().naive_utc();
    Berlin.from_utc_datetime(&utc_now)
}

pub fn parse_input(input: &str) -> (String, Vec<String>, HashMap<String, Option<String>>) {
    let tokens = shell_words::split(input).unwrap_or_default();

    if tokens.is_empty() {
        return ("".to_string(), vec![], HashMap::new());
    }

    let command = tokens[0].to_string();
    let mut args = vec![];
    let mut options = HashMap::new();

    for token in &tokens[1..] {
        if token.starts_with("--") {
            let parts: Vec<&str> = token[2..].splitn(2, '=').collect();
            let key = parts[0].to_string();
            let value = parts.get(1).map(|s| s.to_string());
            options.insert(key, value);
        } else {
            args.push(token.to_string());
        }
    }

    (command, args, options)
}

