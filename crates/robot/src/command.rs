pub trait Command {
    fn init(&mut self) {}
    fn update(&mut self, dt: f32);
    fn is_finished(&self) -> bool;
    fn end(&mut self) {}
}

