use ansi_term::{Color, Style};
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use chrono::{DateTime, Local};
use std::error::Error;
use std::fmt::{Debug, Display};

pub struct CustomErr {
    msg: String,
}

impl CustomErr {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

impl Display for CustomErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Debug for CustomErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " debug : {}", self.msg)
    }
}

impl Error for CustomErr {}

pub fn current_timestamp() -> u64 {
    let local: DateTime<Local> = Local::now();
    local.timestamp_millis() as u64
}
pub fn now_string() -> String {
    let local: DateTime<Local> = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`
    local.to_string()
}

pub fn mark_with_style(title: &str, style: &Style) {
    let add_space = format!("  {}  ", &title);
    let padding_str = format!("{:*^72}", add_space);
    println!("\n[{}] - {}", now_string(), style.paint(padding_str));
}

pub fn mark_line(title: &str) {
    let style: Style = Color::Green.bold();
    mark_with_style(title, &style);
    // println!("{:10}", "hello");
    // println!("{:*<10}", "hello");
    // println!("{:*>10}", "hello");
    // println!("{:*^30}", "hello world");
}

pub fn base64_decode(data_b64: &str) -> Result<Vec<u8>, base64::DecodeError> {
    let engine = engine::GeneralPurpose::new(&alphabet::STANDARD, general_purpose::PAD);
    engine.decode(data_b64)
}

pub fn base64_encode(data: &[u8]) -> String {
    let engine = engine::GeneralPurpose::new(&alphabet::STANDARD, general_purpose::PAD);
    engine.encode(data)
}
