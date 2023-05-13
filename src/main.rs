//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

mod driver;

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
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

use crate::driver::{Display, DriveDirection};

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

    // This is the correct pin on the Raspberry Pico board. On other boards, even if they have an
    // on-board LED, it might need to be changed.
    // Notably, on the Pico W, the LED is not connected to any of the RP2040 GPIOs but to the cyw43 module instead. If you have
    // a Pico W and want to toggle a LED with a simple GPIO output pin, you can connect an external
    // LED to one of the GPIO pins, and reference that pin here.
    let mut led_pin = pins.led.into_push_pull_output();

    // pins for the display driver
    let mut driver_pins = driver::Pins {
        row_address: [
            &mut pins.gpio2.into_push_pull_output(),
            &mut pins.gpio3.into_push_pull_output(),
            &mut pins.gpio4.into_push_pull_output(),
            &mut pins.gpio5.into_push_pull_output(),
            &mut pins.gpio6.into_push_pull_output(),
        ],
        row_high_en: &mut pins.gpio7.into_push_pull_output(),
        row_low_en: &mut pins.gpio8.into_push_pull_output(),
        col_address: [
            &mut pins.gpio9.into_push_pull_output(),
            &mut pins.gpio10.into_push_pull_output(),
            &mut pins.gpio11.into_push_pull_output(),
            &mut pins.gpio12.into_push_pull_output(),
            &mut pins.gpio13.into_push_pull_output(),
        ],
        col_high_low: &mut pins.gpio14.into_push_pull_output(),
        col_select: &mut [
            &mut pins.gpio16.into_push_pull_output(),
            &mut pins.gpio17.into_push_pull_output(),
        ],
    };

    let mut display = Display::<16, 42>::new(driver_pins);

    display.clear();
    display.refresh(&mut delay, true);

    // let range = 0..35;

    // for i in range.clone() {
    //     // debug!("Low: {}", i);
    //     driver_pins.drive_pixel(1, i, DriveDirection::Low, &mut delay, 2000);
    // }
    // delay.delay_ms(1000);

    let mut t = 0;

    loop {
        display.clear();

        for row in 0..10 {
            for i in t..t + 3 {
                display.set_pixel(row, (row + i) % 42, true);
            }
        }

        display.refresh(&mut delay, false);

        delay.delay_ms(50);
        t += 1;
    }
    // loop {
    //     info!("on!");
    //     led_pin.set_high().unwrap();

    //     // for i in range.clone() {
    //     //     debug!("High: {}", i);
    //     //     driver_pins.drive_pixel(1, i, DriveDirection::High, &mut delay, 2000);
    //     //     delay.delay_ms(100);
    //     // }
    //     display.set_pixel(1, 1, true);
    //     display.refresh(&mut delay, false);

    //     delay.delay_ms(1000);

    //     info!("off!");
    //     led_pin.set_low().unwrap();

    //     // for i in range.clone() {
    //     //     debug!("Low: {}", i);
    //     //     driver_pins.drive_pixel(1, i, DriveDirection::Low, &mut delay, 2000);
    //     //     delay.delay_ms(100);
    //     // }

    //     display.set_pixel(1, 1, false);
    //     display.refresh(&mut delay, false);

    //     delay.delay_ms(1000);
    // }
}
