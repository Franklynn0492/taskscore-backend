use std::collections::HashMap;

use bolt_client::bolt_proto::Value;
use chrono::{DateTime, Utc, FixedOffset};


pub fn get_string(properties: &HashMap<String, Value>, key: &str, alternative: &str) -> String  {
    match properties.get(key) {
        Some(Value::String(val)) => val.clone(),
        _ => alternative.to_owned()
    }
}

pub fn get_bool(properties: &HashMap<String, Value>, key: &str, alternative: bool) -> bool  {
    match properties.get(key) {
        Some(Value::Boolean(val)) => *val,
        _ => alternative
    }
}

pub fn get_u16(properties: &HashMap<String, Value>, key: &str, alternative: u16) -> u16  {
    match properties.get(key) {
        Some(Value::Integer(val)) => *val as u16,
        _ => alternative
    }
}

pub fn get_u32(properties: &HashMap<String, Value>, key: &str, alternative: u32) -> u32  {
    match properties.get(key) {
        Some(Value::Integer(val)) => *val as u32,
        _ => alternative
    }
}

pub fn get_utc(properties: &HashMap<String, Value>, key: &str, alternative: DateTime<Utc>) -> DateTime<Utc>  {
    match properties.get(key) {
        Some(Value::String(val)) => {
            let fixed_offset_res = DateTime::<FixedOffset>::parse_from_rfc3339(val.as_str());
            match fixed_offset_res {
                Ok(fixed_offset) => DateTime::<Utc>::from(fixed_offset),
                _ => alternative
            }
        },
        _ => alternative
    }
}