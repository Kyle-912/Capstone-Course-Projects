#![no_std]
#![no_main]

/**** low-level imports *****/
use core::fmt::Write as SerialWrite;
use core::panic::PanicInfo;
// use panic_halt as _;
use cortex_m::prelude::*;
use cortex_m_rt::entry;
use embedded_hal::blocking::i2c::{Read, Write};
use embedded_hal::{
    digital::v2::{InputPin, OutputPin},
    spi,
    timer::CountDown,
};
// use embedded_time::rate::*;

/***** board-specific imports *****/
use adafruit_feather_rp2040::hal;
use adafruit_feather_rp2040::{
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        gpio::{FunctionI2C, FunctionSpi, FunctionUart},
        pac,
        pac::interrupt,
        pio::PIOExt,
        timer::Timer,
        uart,
        watchdog::Watchdog,
        Sio, I2C,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};

/**** imports for external devices *****/
use fugit::{ExtU32, RateExtU32};
use lis3dh::accelerometer::RawAccelerometer;
use lis3dh::{Lis3dh, Lis3dhI2C};
use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_pio::Ws2812;

// USB Device support
use usb_device::class_prelude::*;
// USB Communications Class Device support
mod usb_manager;
use usb_manager::UsbManager;
// Global USB objects & interrupt
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;
static mut USB_MANAGER: Option<UsbManager> = None;
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    match USB_MANAGER.as_mut() {
        Some(manager) => manager.interrupt(),
        None => (),
    };
}
#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    if let Some(usb) = unsafe { USB_MANAGER.as_mut() } {
        writeln!(usb, "{}", panic_info).ok();
    }
    loop {}
}

mod animations;
use animations::{Pulse, Snake, Strobe, Wave};

#[entry]
fn main() -> ! {
    // Grab the singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    // Init the watchdog timer, to pass into the clock init
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Setup USB
    let usb = unsafe {
        USB_BUS = Some(UsbBusAllocator::new(hal::usb::UsbBus::new(
            pac.USBCTRL_REGS,
            pac.USBCTRL_DPRAM,
            clocks.usb_clock,
            true,
            &mut pac.RESETS,
        )));
        USB_MANAGER = Some(UsbManager::new(USB_BUS.as_ref().unwrap()));
        // Enable the USB interrupt
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
        USB_MANAGER.as_mut().unwrap()
    };

    // Initialize the Single Cycle IO
    let sio = Sio::new(pac.SIO);
    // Initialize the pins to default state
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Initialize pio
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);
    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let mut neopixels = Ws2812::new(
        pins.d5.into_mode(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    // Initialize I2C
    let i2c_sda = pins.sda.into_mode();
    let i2c_scl = pins.scl.into_mode();
    let i2c = I2C::i2c1(
        pac.I2C1,
        i2c_sda,
        i2c_scl,
        400.kHz(),
        &mut pac.RESETS,
        &clocks.system_clock,
    );

    // Initialize the LIS3DH accelerometer
    let mut lis3dh = Lis3dh::new_i2c(i2c, lis3dh::SlaveAddr::Default).unwrap();
    lis3dh.set_mode(lis3dh::Mode::Normal).unwrap();
    let accel = lis3dh.accel_raw().unwrap();
    let mut x;
    let mut y;

    // Setup the Propmaker Power Enable pin
    let mut pwr_pin = pins.d10.into_push_pull_output();
    pwr_pin.set_high().unwrap();

    let mut delay_timer =
        cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Define modes
    let mut pulse = Pulse::new(RGB8::new(255, 0, 0));
    let mut snake = Snake::new(RGB8::new(0, 255, 0));
    let mut flash = Strobe::new(RGB8::new(255, 255, 255));
    let mut wave = Wave::new(RGB8::new(0, 0, 255));

    let mut mode: u8;
    let mut nticks: u8 = 5; // Loop delay is ms
    loop {
        if nticks > 4 {
            nticks = 0;

            x = accel.x as i32;
            y = accel.y as i32;

            if x.abs() > y.abs() {
                if x > 0 {
                    mode = 0; // +x
                } else {
                    mode = 2; // -x
                }
            } else {
                if y > 0 {
                    mode = 1; // +y
                } else {
                    mode = 3; // -y
                }
            }

            write!(usb, "X: {}, Y: {}\r\n", x, y).unwrap();
            write!(usb, "Updating display...\r\n").unwrap();

            pulse.next();
            snake.next();
            flash.next();
            wave.next();

            let ds: [RGB8; animations::NUM_PX] = match mode {
                0 => pulse.to_list(),
                1 => snake.to_list(),
                2 => flash.to_list(),
                3 => wave.to_list(),
                _ => [RGB8::new(0, 0, 0); animations::NUM_PX],
            };

            neopixels.write(ds.iter().cloned()).unwrap();
        }
        nticks += 1;
        delay_timer.delay_ms(5 as u32);
    }
}
