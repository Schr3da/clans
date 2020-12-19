pub trait Updateable {
    fn needs_update(&self) -> bool;
    fn force_update(&mut self, should_update: bool);
}
