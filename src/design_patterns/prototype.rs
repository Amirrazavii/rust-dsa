#[derive(Debug, Clone)]
pub struct Circle {
    pub radius: f64,
}

pub trait Prototype: std::fmt::Debug {
    fn clone_box(&self) -> Box<dyn Prototype>;
}

impl Prototype for Circle {
    fn clone_box(&self) -> Box<dyn Prototype> {
        Box::new(self.clone())
    }
}
