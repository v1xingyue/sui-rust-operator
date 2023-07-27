use ansi_term::{Color, Style};
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use chrono::{DateTime, Local};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{Debug, Display},
    fs::File,
    io::Read,
    thread,
    time::Duration,
    vec,
};

pub const ADVISE_GAS_BUDGET: u64 = 300_000_000;

pub struct CustomErr {
    msg: String,
}

impl CustomErr {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }

    pub fn new_box(msg: &str) -> Box<dyn Error> {
        Box::new(Self {
            msg: msg.to_string(),
        })
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

pub fn mark_with_style(title: String, style: &Style) {
    let add_space = format!(" {} ", &title);
    let padding_str = format!("{:<72}", add_space);
    println!(
        "🦈 [{}] - {}",
        style.to_owned().underline().paint(now_string()),
        style.paint(padding_str)
    );
}

pub fn random_style() -> Style {
    let styles = vec![
        Color::Red.italic(),
        Color::Green.italic(),
        Color::Yellow.italic(),
        Color::Blue.bold().underline().italic(),
        Color::Purple.underline().bold(),
        Color::Cyan.bold().bold(),
    ];
    let mut rng = rand::thread_rng();
    let random_style = rng.gen_range(0, styles.len());
    styles[random_style]
}

pub fn mark_line(title: String) {
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

pub fn sleep_with_message(message: String) {
    println!("{}", message);
    thread::sleep(Duration::from_secs(5));
}

#[derive(Serialize, Deserialize)]
pub struct CompiledModule {
    pub modules: Vec<String>,
    pub dependencies: Vec<String>,
    pub digest: Vec<u8>,
}

impl CompiledModule {
    pub fn from_file(path: String) -> Self {
        let mut file = File::open(path).expect("can't open dumped file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("read file error .");
        serde_json::from_str(&contents).expect("无法反序列化数据")
    }
}
