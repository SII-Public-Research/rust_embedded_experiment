//! Serial interface loopback test
//!
//! You have to short the TX and RX pins to make this program work

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_semihosting as _;

use cortex_m::asm;

use nb::block;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    serial::{Config, Serial},
};

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let p = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Prepare the alternate function I/O registers
    let mut afio = p.AFIO.constrain(&mut rcc.apb2);

    // Prepare the GPIOB peripheral
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);

    // USART1
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;

    // USART1
    // let tx = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    // let rx = gpiob.pb7;

    // USART2
    // let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    // let rx = gpioa.pa3;

    // USART3
    // Configure pb10 as a push_pull output, this will be the tx pin
    // let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    // Take ownership over pb11
    // let rx = gpiob.pb11;

    // Set up the usart device. Taks ownership over the USART register and tx/rx pins. The rest of
    // the registers are used to enable and configure the device.
    let mut serial = Serial::usart1(
        p.USART1,
        (tx, rx),
        &mut afio.mapr,
        Config::default().baudrate(9600.bps()),
        clocks,
        &mut rcc.apb2,
    );

    // Loopback test. Write `X` and wait until the write is successful.
    hprintln!("{On envoie une valeur}").unwrap();
    let sent = b'X';
    block!(serial.write(sent)).ok();

    hprintln!("{On recoit une valeur :}").unwrap();
    // Read the byte that was just sent. Blocks until the read is complete
    let received = block!(serial.read()).unwrap();

    hprintln!("{}", received).unwrap();

    hprintln!("{on est pas bloque :}").unwrap();



    loop {}
}