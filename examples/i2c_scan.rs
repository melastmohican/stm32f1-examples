#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use stm32f1xx_hal::{
    gpio::GpioExt,
    i2c::{BlockingI2c, DutyCycle, Mode},
    pac,
    prelude::*,
};

#[entry]
fn main() -> ! {
    info!("Starting I2C scan...");

    // Get access to device peripherals
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();

    // Configure clocks
    let clocks = rcc.cfgr.freeze(&mut dp.FLASH.constrain().acr);

    // Configure GPIOB for I2C1
    let mut afio = dp.AFIO.constrain();
    let mut gpiob = dp.GPIOB.split();
    let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
    let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

    // Initialize I2C1 in blocking mode
    info!("Initialize I2C1 in blocking mode.");
    let mut i2c = dp
        .I2C1
        //.remap(&mut afio.mapr) // add this if want to use PB8, PB9 instead
        .blocking_i2c(
            (scl, sda),
            Mode::Fast {
                frequency: 400.kHz(),
                duty_cycle: DutyCycle::Ratio16to9,
            },
            &clocks,
            1000,
            10,
            1000,
            1000,
        );

    // Scan I2C addresses
    for addr in 0x01..=0x7F {
        //info!("Scanning 0x{:02X}", addr);
        let mut buf = [0u8; 1];
        match i2c.read(addr, &mut buf) {
            Ok(_) => info!("Device found at 0x{:02X}", addr),
            Err(_) => {} // No response, ignore
        }
    }

    info!("I2C scan complete.");

    loop {}
}
