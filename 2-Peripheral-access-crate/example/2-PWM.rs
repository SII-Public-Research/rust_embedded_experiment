// Programme d'exemple d'utilisation d'un TIMER pour sortir un signal PWM
// On va configurer le canal CH1 du TIM3, qui sort sur la boche PA.6

// A TESTER !!!!

// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]


use panic_semihosting as _;

use cortex_m_rt::entry;

use stm32f1::stm32f103;
use stm32f1::stm32f103::interrupt;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;


// the program entry point
#[entry]
fn main() -> ! {

	let dp = stm32f103::Peripherals::take().unwrap();


	/****************************************************************************************/
    /*****************              ACTIVATION DES HORLOGES          ************************/
    /****************************************************************************************/


	let rcc = &dp.RCC;

	// enable the GPIOA peripheral
	rcc.apb2enr.modify(|_, w| w.iopaen().set_bit());
    // allume l'horloge correspondant au timer 3
    rcc.apb1enr.modify(|_, w| w.tim3en().set_bit());


	/****************************************************************************************/
    /*****************              INITIALISATION DES GPIOS         ************************/
    /****************************************************************************************/
    

	let gpioa = &dp.GPIOA;

	// configurartion du pin 6 en mode alternate function, push-pull
	gpioa.crl.modify(|_, w| w.mode6().output2()); // output 2MHz
	// configure mode output push pull
	gpioa.crl.modify(|_, w| w.cnf6().alt_push_pull()); // push-pull


    /****************************************************************************************/
    /*****************             INITIALISATION DE TIM3          **************************/
    /****************************************************************************************/


    let tim3 = &dp.TIM3;
    // configuration du pre scaler
    tim3.psc.write(|w| w.psc().bits(0));
    // autoreload
    tim3.arr.write(|w| w.arr().bits(3599));

    // configuration du canal CH1 en mode PWM (p. 386) 
    // ecrire b110 dans le registre TIM3_CCMR1, bits OC1M
    tim3.ccmr1_output.write(|w| w.oc1m().pwm_mode1());
    // validation du canal de sortie 
    tim3.ccer.write(|w| w.cc1e().set_bit());
    //reglage de la duree de l'impulsion (20% -> 719) resolution du timer = ARR
    tim3.ccr.write(|w| w.ccr().bits(719);
    // demarre le timer
    tim3.cr1.write(|w| w.cen().set_bit());


    loop {}
    	
}
