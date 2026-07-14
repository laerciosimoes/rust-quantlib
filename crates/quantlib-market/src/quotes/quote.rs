pub trait Quote: Send + Sync {
    fn value(&self) -> f64;
    fn is_valid(&self) -> bool;
}

pub trait Quote: Send + Sync {
    fn value(&self) -> f64;
    fn is_valid(&self) -> bool;
}