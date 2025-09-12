mod design_patterns;
pub mod problems;
use design_patterns::prototype::{Circle, Prototype};
use design_patterns::singleton::{Config};
fn main() {
    let config = Config::get_instance();

    {
        let mut cfg = config.lock().unwrap();
        cfg.set_value("Hello Singleton!");
    }

    {
        let cfg = config.lock().unwrap();
        println!("Config value: {}", cfg.get_value());
    }
}
