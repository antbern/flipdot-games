//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

mod driver;

use core::time::Duration;

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;
use common::{
    input::DebouncedInput, menu::GameMenu, snake::SnakeGame, tetris::TetrisGame, Game,
    RandomNumberSource,
};

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

use crate::driver::Display;

const ROWS: usize = 16;
const COLS: usize = 42;

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
    let driver_pins = driver::Pins {
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
            &mut pins.gpio9.into_push_pull_output(),  // 13
            &mut pins.gpio10.into_push_pull_output(), // 14
            &mut pins.gpio11.into_push_pull_output(), // 15
            &mut pins.gpio12.into_push_pull_output(), // 16
            &mut pins.gpio13.into_push_pull_output(), // 17
        ],
        col_high_low: &mut pins.gpio14.into_push_pull_output(),
        col_select: &mut [
            &mut pins.gpio16.into_push_pull_output(),
            &mut pins.gpio17.into_push_pull_output(),
        ],
    };

    let mut display = Display::<ROWS, COLS>::new(driver_pins);

    led_pin.set_high().unwrap();
    // do a deep screen refresh (to make sure all pixels are really off)
    display.clear();
    display.fill(true);
    display.refresh(&mut delay, true);
    delay.delay_ms(500);

    display.clear();
    display.refresh(&mut delay, true);
    delay.delay_ms(250);
    display.refresh(&mut delay, true);
    delay.delay_ms(250);

    led_pin.set_low().unwrap();

    let mut games = [
        &mut TetrisGame::<ROWS, COLS>::new() as &mut dyn Game<_, _, _>,
        &mut SnakeGame::<ROWS, COLS>::new() as &mut dyn Game<_, _, _>,
    ];

    let mut game = GameMenu::new(&mut games);

    let input_up = pins.gpio28.into_pull_up_input();
    let input_down = pins.gpio27.into_pull_up_input();
    let input_left = pins.gpio26.into_pull_up_input();
    let input_right = pins.gpio22.into_pull_up_input();
    let input_action = pins.gpio21.into_pull_up_input();

    let mut i_debounced = DebouncedInput::default();

    let mut rng = RandomSource::default();
    loop {
        // tick every 10ms
        delay.delay_ms(10);

        // refresh screen if game did an update
        let elapsed = Duration::from_millis(10);

        // read the input
        let i = common::input::BasicInput {
            left: input_left.is_low().unwrap(),
            right: input_right.is_low().unwrap(),
            up: input_up.is_low().unwrap(),
            down: input_down.is_low().unwrap(),
            action: input_action.is_low().unwrap(),
        };

        defmt::debug!(
            "left:{}, right:{}, up:{}, down:{}, action: {}",
            i.left,
            i.right,
            i.up,
            i.down,
            i.action
        );

        i_debounced.update(&i);

        // the display is already "double buffered" so this repeated calling should be fine!
        game.update(elapsed, &i_debounced, &mut display, &mut rng);
        display.refresh(&mut delay, false);
    }
}

#[derive(Default)]
struct RandomSource {
    count: u32,
}

impl RandomNumberSource for RandomSource {
    fn next_u32(&mut self) -> u32 {
        self.count += 1;
        self.count
    }
}
