use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct Config {
    pub value: String,
}

impl Config {
    fn new() -> Self {
        Config {
            value: "Default config".to_string(),
        }
    }

    pub fn get_instance() -> &'static Mutex<Config> {
        static INSTANCE: Lazy<Mutex<Config>> = Lazy::new(|| Mutex::new(Config::new()));
        &INSTANCE
    }

    pub fn set_value(&mut self, val: &str) {
        self.value = val.to_string();
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}
