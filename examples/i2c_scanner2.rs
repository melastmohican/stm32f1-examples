//! Example of using I2C.
//! Scans available I2C devices on bus and print the result.
//! https://github.com/stm32-rs/stm32f3xx-hal/blob/master/examples/i2c_scanner.rs

#![no_std]
#![no_main]

use core::{convert::TryInto, ops::Range};

use panic_semihosting as _;

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::{hprint, hprintln};

use stm32f1xx_hal::{self as hal, pac, prelude::*};
use stm32f1xx_hal::i2c::{BlockingI2c, DutyCycle, Error, Mode};

const VALID_ADDR_RANGE: Range<u8> = 0x08..0x7F;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split();

    // Configure I2C1
    let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
    let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

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

    hprintln!("Start i2c scanning...");
    hprintln!();

    for addr in 0x00_u8..0x7F {
        // Write the empty array and check the slave response.
        if VALID_ADDR_RANGE.contains(&addr) && i2c.write(addr, &[]).is_ok() {
            hprint!("{:02x}", addr);
        } else {
            hprint!("..");
        }
        if addr % 0x10 == 0x0F {
            hprintln!();
        } else {
            hprint!(" ");
        }
    }

    hprintln!();
    hprintln!("Done!");



    loop {
        asm::wfi();
    }
}