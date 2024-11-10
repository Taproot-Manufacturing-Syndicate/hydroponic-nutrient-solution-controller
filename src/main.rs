#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

mod stepper;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut pump0 = stepper::Stepper::new(
        pins.gpio0.into_push_pull_output(), // IN1
        pins.gpio1.into_push_pull_output(), // IN3
        pins.gpio2.into_push_pull_output(), // IN2
        pins.gpio3.into_push_pull_output(), // IN4
    );

    let mut pump1 = stepper::Stepper::new(
        pins.gpio4.into_push_pull_output(), // IN1
        pins.gpio5.into_push_pull_output(), // IN3
        pins.gpio6.into_push_pull_output(), // IN2
        pins.gpio7.into_push_pull_output(), // IN4
    );

    loop {
        // 3_000 µs/step gives enough torque that two motors together
        // can pump water.
        const DELAY_US: u32 = 3_000;
        const _STEPS_PER_REVOLUTION: u32 = 2048;
        pump0.step(true);
        pump1.step(false);
        delay.delay_us(DELAY_US);
    }
}
