pub trait Hooks {
    fn setup(&mut self) -> Result<(), String>;
}
