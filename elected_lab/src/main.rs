#![no_std]
#![no_main]

/**** low-level imports *****/
use core::fmt::Write as SerialWrite;
use core::panic::PanicInfo;
use adafruit_feather_rp2040::hal::gpio::I2C;
use adafruit_feather_rp2040::pac::I2C1;
// use panic_halt as _;
use cortex_m::prelude::*;
use cortex_m_rt::entry;
use embedded_hal::blocking::i2c::{Read, Write, self};
use embedded_hal::{
    digital::v2::{InputPin, OutputPin},
    spi,
    timer::CountDown,
};
use embedded_time::rate::*;

/***** board-specific imports *****/
use adafruit_feather_rp2040::hal::{self, i2c};
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
use animations::{Pulse, Snake}; //TODO: add other 2

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
    // let mut i2c = I2C::i2c1(
    //     pac.I2C1,
    //     i2c_sda,
    //     i2c_scl,
    //     400000,
    //     &mut pac.RESETS,
    //     &pac.CLOCKS,
    // );

    // Initialize the LIS3DH accelerometer
    let mut lis3dh = Lis3dh::new_i2c(i2c, 0x18);

    // Set the accelerometer to a specific range and mode, e.g., ±2g and normal mode
    lis3dh.set_mode(lis3dh::Mode::Normal).unwrap();

    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    // Setup the Propmaker Power Enable pin
    let mut pwr_pin = pins.d10.into_push_pull_output();
    pwr_pin.set_high().unwrap();

    let mut delay_timer =
        cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Define modes TODO: add other 2
    let mut pulse = Pulse::new(RGB8::new(50, 0, 50));
    let mut snake = Snake::new(RGB8::new(255, 255, 255));

    let mut mode: u8 = 0; //TODO: will later be set by accel values
    let mut nticks: u8 = 5; // Loop delay is ms

    loop {
        if lis3dh.accel_status().unwrap() {
            let accel = lis3dh.accel_raw().unwrap();
            x = accel.x as i32;
            y = accel.y as i32;
            z = accel.z as i32;
        }
        if nticks > 4 {
            write!(usb, "X: {}, Y: {}, Z: {}\r\n", x, y, z).unwrap();
            write!(usb, "Updating display...\r\n").unwrap();
            nticks = 0;
            pulse.next(); //TODO: add other 2
            snake.next();

            let ds: [RGB8; animations::NUM_PX] = match mode {
                0 => pulse.to_list(),
                1 => snake.to_list(),
                // 2 => TODO:.to_list(),
                // 3 => TODO:.to_list(),
                _ => [RGB8::new(0, 0, 0); animations::NUM_PX],
            };

            neopixels.write(ds.iter().cloned()).unwrap();
        }
        nticks += 1;
        delay_timer.delay_ms(5 as u32);
    }

    /*
    // Old Loop Section
    let mut blinky_led_pin = pins.d13.into_push_pull_output();
    let delay: u32 = 500; // loop delay in ms
    let mut n: u32 = 0;
    loop {
        write!(usb, "starting loop number {:?}\r\n", n).unwrap();
        blinky_led_pin.set_low().unwrap();
        delay_timer.delay_ms(delay as u32);
        blinky_led_pin.set_high().unwrap();
        delay_timer.delay_ms(delay as u32);
        n = n + 1;
    }
    */
}
