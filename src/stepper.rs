/// trait to define shared behavior across stepper types
pub trait Stepper {
    fn step(&mut self, dir: bool);
}
