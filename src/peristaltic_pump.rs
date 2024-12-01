use crate::Stepper;
use crate::VolumetricPump;

/// A peristaltic pump that takes two steppers
/// lcpp -> Low Cost Peristaltic Pump
/// 28byj -> Steppers used in the pump
///
/// from: https://www.thingiverse.com/thing:254956
pub struct Lcpp28byj {
    stepper0: Stepper,
    stepper1: Stepper,
}

impl Lcpp28byj {
    pub fn new(stepper0: Stepper, stepper1: Stepper) -> Self {
        Self { stepper0, stepper1 }
    }

    /// manually step the LCPP28BYJ
    pub fn step(&self, direction: bool) {
        self.stepper0.step(direction);
        self.stepper1.step(!direction);
    }
}

// TODO: get volumetric calibration for the lcpp28byj
impl VolumetricPump for Lcpp28byj {
    fn pump_ml(ml: usize) {
        todo!("implement pump_ml for LCPP28BYJ");
    }
}
