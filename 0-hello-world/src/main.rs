// Programme d'exemple pour faire du semi-hosting

// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

// dev profile: easier to debug panics; can put a breakpoint on `rust_begin_unwind`

use panic_semihosting as _;



// makes `panic!` print messages to the host stderr using semihosting
// extern crate panic_semihosting;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
// use core::ptr; // pour passer en volatile 


// the program entry point
#[entry]
fn main() -> ! {

	hprintln!("Hello, world!").unwrap();

	// Get access to the device specific peripherals from the peripheral access crate
	// let dp = pac::Peripherals::take().unwrap();

    loop {}
    	
}
