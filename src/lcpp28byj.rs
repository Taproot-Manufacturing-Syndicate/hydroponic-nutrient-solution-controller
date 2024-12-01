use crate::pump::VolumetricPump;
use crate::stepper::Stepper;
/// A peristaltic pump that takes two steppers
/// lcpp -> Low Cost Peristaltic Pump
/// 28byj -> Steppers used in the pump
///
/// from: https://www.thingiverse.com/thing:254956
pub struct Lcpp28byj<STEPPER0, STEPPER1>
where
    STEPPER0: Stepper,
    STEPPER1: Stepper,
{
    stepper0: STEPPER0,
    stepper1: STEPPER1,
}

impl<STEPPER0, STEPPER1> Lcpp28byj<STEPPER0, STEPPER1>
where
    STEPPER0: Stepper,
    STEPPER1: Stepper,
{
    pub fn new(stepper0: STEPPER0, stepper1: STEPPER1) -> Self {
        Self { stepper0, stepper1 }
    }

    /// manually step the LCPP28BYJ
    pub fn step(&mut self, direction: bool) {
        self.stepper0.step(direction);
        self.stepper1.step(!direction);
    }
}

// TODO: get volumetric calibration for the lcpp28byj
impl<STEPPER0, STEPPER1> VolumetricPump for Lcpp28byj<STEPPER0, STEPPER1>
where
    STEPPER0: Stepper,
    STEPPER1: Stepper,
{
    fn pump_ml(_ml: usize) {
        todo!("implement pump_ml for LCPP28BYJ");
    }
}
