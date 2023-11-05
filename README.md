
# :video_game: flipdot-games

Implementation of some game(s) for playing on a classic flipdot display! When [CRF](https://chalmersrobotics.se) recevied these in 2019 I was very excited to use them for something and started investigating. That resulted in the creation of [ChalmersRobotics/mqtt-flipdot-driver](https://github.com/ChalmersRobotics/mqtt-flipdot-driver) which used the built-in controller to put simple text and numbers on the display.

After my growing interest in [Rust](https://www.rust-lang.org/) and a few fascinating YouTube videos (especially [this](https://www.youtube.com/watch?v=8DvH6FiS3sg) by [@bitlunislab](https://www.youtube.com/@bitlunislab)). I came up with the idea of coding some games :video_game:

The following game(s) are currently implemented / work in progress:

- [x] Snake (quite feature complete, only missing random starting positions on the pico!)
- [ ] Tetris (wouldn't this be cool!? Well, work-in-progress... feel free to contribute!)


## :hammer: Development

To ease development, the project is separated into three crates:

- [`common`](common/): has all the game logic and defines abstractions (`Traits`) that the frontends need to implement.
- [`cli`](cli/): a command-line version where you can use `wasd` and `space` to control the game in your terminal! Perfect for quick iteration during development :fire:
- [`pico-firmware`](pico-firmware/): contains firmware for a Raspberry Pico to run the game on the real flipdot display! Since this is Rust, this usually "just works" after using the `cli` for development! Hardware schematics is work-in-progress!

<!-- The hardware (with schematics and pictures) for this is available in the folder [`pico-hardware`](pico-hardware). -->


### Run the CLI

To run the CLI, simply move into the folder and execute `cargo`:
```bash
cd cli
cargo run
```

### Flashing the Pico

Flashing the firmware to the Pico is a bit more involved. First you need to the right tools and add another rust target:

1. Add the `thumbv6m` target using [`rustup`](https://rustup.rs/).
    ```bash
    rustup target install thumbv6m-none-eabi
    ```

2. Install [`flip-link`](https://github.com/knurling-rs/flip-link) to provide zero-cost stack overflow protection.
    ```bash
    cargo install flip-link
    ```

3. Install [`probe-rs`]() used for uploading the firmware to the device. Previously used [`probe-run`](https://github.com/knurling-rs/probe-run) but as of October 2023 it has been [deprecated](https://ferrous-systems.com/blog/probe-run-deprecation/) in favor of `probe-rs`.
    ```bash
    cargo install probe-rs --features cli
    ```

You also need a compatible debug probe connected to the target Pico to flash.
You can use a second [Pico as a CMSIS-DAP debug probe](pico-firmware/debug_probes.md#raspberry-pi-pico). Details on other supported debug probes can be found in [pico-firmware/debug_probes.md](pico-firmware/debug_probes.md).

After all requirements are installed (and you see your debug probe listed when running `probe-rs list`), you should be able to simply execute
```bash
cd pico-firmware
cargo run --release
```
to compile and upload the firmware to the device! If you encounter any difficulties (perhaps I missed a step :confused:), please open an issue :triangular_flag_on_post:

## :books: Resources

Some resources that I found useful 

- [ChalmersRobotics/mqtt-flipdot-driver](https://github.com/ChalmersRobotics/mqtt-flipdot-driver) my previous work on interfacing with the flipdot displays.

- [openspaceaarhus/flipdot hardware schematic](https://github.com/openspaceaarhus/flipdot/blob/master/flipper/master_setup.pdf) - the connector on our display has the same pinout and functionality!
