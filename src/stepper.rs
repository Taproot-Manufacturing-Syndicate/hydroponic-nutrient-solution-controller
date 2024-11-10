use defmt::*;

pub struct Stepper<In1, In2, In3, In4>
where
    In1: embedded_hal::digital::OutputPin,
    In2: embedded_hal::digital::OutputPin,
    In2: embedded_hal::digital::OutputPin,
    In2: embedded_hal::digital::OutputPin,
{
    in1: In1,
    in2: In2,
    in3: In3,
    in4: In4,
    waveform_index: usize,
}

// Copied this waveform from
// <https://github.com/arduino-libraries/Stepper/blob/master/src/Stepper.h>
const WAVEFORM: [u8; 4] = [0b0101, 0b0110, 0b1010, 0b1001];

impl<In1, In2, In3, In4> Stepper<In1, In2, In3, In4>
where
    In1: embedded_hal::digital::OutputPin,
    In2: embedded_hal::digital::OutputPin,
    In3: embedded_hal::digital::OutputPin,
    In4: embedded_hal::digital::OutputPin,
{
    pub fn new(in1: In1, in2: In2, in3: In3, in4: In4) -> Stepper<In1, In2, In3, In4> {
        let mut s = Stepper {
            in1,
            in2,
            in3,
            in4,
            waveform_index: 0,
        };
        s.set_pins();
        return s;
    }

    fn set_pins(&mut self) {
        let pins = WAVEFORM[self.waveform_index];
        self.in1.set_state(((pins & 0x1) != 0).into()).unwrap();
        self.in2.set_state(((pins & 0x2) != 0).into()).unwrap();
        self.in3.set_state(((pins & 0x4) != 0).into()).unwrap();
        self.in4.set_state(((pins & 0x8) != 0).into()).unwrap();
    }

    pub fn step(&mut self, dir: bool) {
        let delta: isize = match dir {
            true => 1,
            false => -1,
        };
        self.waveform_index = self.waveform_index.wrapping_add_signed(delta) % WAVEFORM.len();
        self.set_pins();
    }
}
