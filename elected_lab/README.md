Embedded Rust Template Project for Feather RP2040
=================================================

This repo contains a simple blinky-led example in embedded rust for the Adafruit Feather RP2040. The template includes code that will configure the USB peripheral as a serial port to allow for printing of formatted strings via the `write!` macro. Additionally, panic messages are sent to the serial port, and will show up when properly connected to a utility such as minicom, nRF terminal, or putty.

### Acknowledgements
Most of this code is adapted from [this](https://github.com/eterevsky/rp2040-blink/blob/main/README.md) repository -- thanks!

This project contains code for an orientation based animated display in embedded rust for the Adafruit Feather RP2040. The code also includes serial printing of the relevant accelerometer values and each time the display is updated for debugging. To build the application run "cargo run" in this directory, then flash the elected_lab.uf2 at home\wsl\CEN3907\elected_lab\target\thumbv6m-none-eabi\debug to the board of USB