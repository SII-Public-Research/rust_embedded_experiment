// Programme d'exemple d'utilisation d'une crate PAC

// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

use stm32f1::stm32f103;

// dev profile: easier to debug panics; can put a breakpoint on `rust_begin_unwind`
use panic_semihosting as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
// use core::ptr; // pour passer en volatile 


// the program entry point
#[entry]
fn main() -> ! {

	hprintln!("Bonjour !").unwrap();

	let p = stm32f103::Peripherals::take().unwrap();

	let rcc = p.RCC;

	let gpioa = p.GPIOA;

	// enable the GPIOA peripheral
	rcc.apb2enr.modify(|_, w| w.iopaen().set_bit());

	// configure the pin as output
	gpioa.crl.modify(|_, w| w.mode5().output());
	// configure mode output push pull
	gpioa.crl.modify(|_, w| w.cnf5().push_pull());
	// allume la LED
	gpioa.odr.modify(|_, w| w.odr5().set_bit());


    loop {}
    	
}
