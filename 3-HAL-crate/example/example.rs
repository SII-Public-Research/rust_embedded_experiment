#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_rt::entry;

use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::i2c::I2c;

#[entry]
fn main() -> ! {

     let dp = pac::Peripherals::take().unwrap();
     let mut flash = dp.FLASH.constrain();
     let mut rcc = dp.RCC.constrain();
     let clocks = rcc.cfgr.hclk(8.mhz()).freeze(&mut flash.acr);


     loop {}
}
