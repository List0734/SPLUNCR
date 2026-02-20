#[derive(Debug)]
pub struct Message<T> {
    pub timestamp: u64,
    pub payload: T,
}