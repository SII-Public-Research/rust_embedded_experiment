// Jeux utilisant les GPIOS, timers et interruption
// 	- LED : GPIOA pin 5
// 	- BUTTON : GPIOC pin 13

// DESCRITPION DU JEU : le joueur va devoir appuyer sur un bouton quand une LED s'allume 
// et avant qu'elle ne s'éteigne
// le moment auquel s'allume la LED est aléatoire.
// Le joueur aura seulement 300ms pour réagir, le LED s'éteignant ensuite.
// Si le joueur gagne, alors la LED se met à clignoter 4 fois par secondes


// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]


use panic_semihosting as _;

use cortex_m_rt::entry;

use stm32f1::stm32f103::interrupt;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use stm32f1::stm32f103;


// déclaration des mutex vides dans des sections critiques 
static MU: Mutex<RefCell<Option<stm32f103::Peripherals>>> = Mutex::new(RefCell::new(None));

// le code en dessous pose problème au niveau du borrowing de la variable dp :
//static MU_TIM2: Mutex<RefCell<Option<stm32f103::TIM2>>> = Mutex::new(RefCell::new(None));
//static MU_GPIOA: Mutex<RefCell<Option<stm32f103::GPIOA>>> = Mutex::new(RefCell::new(None));


/*********************************************************************************************/
/**********                       GESTION DES INTERUPTIONS                       *************/
/*********************************************************************************************/


#[interrupt]
fn TIM2() {
	// le timer 2 gère la durée d'allumage de la LED
	// cette durée est toujour la meme. Donc la configuration est faite dans la fonction main
	// ici, on va 
	//	0- stopper TIM2
	// 	1- stop la led 
	//	2- configurer le temps du timer 3 (temps aléatoire) via le registre ARR
	//	3- demarrer TIM3

	cortex_m::interrupt::free(|cs| {
		let peripherals = MU.borrow(cs).borrow();

		// on etteind la LED
		peripherals.as_ref().unwrap().GPIOA.odr.write(|w| w.odr5().low());

		// arrete TIM2 (bit CEN)
    	peripherals.as_ref().unwrap().TIM2.cr1.write(|w| w.cen().clear_bit());
    	// on remet la valeur du bit UIF à 0
        peripherals.as_ref().unwrap().TIM2.sr.write(|w| w.uif().clear_bit());

        // on configure et demarre TIM3 (bit CEN)
        peripherals.as_ref().unwrap().TIM3.arr.write(|w| w.arr().bits(rand()));
		peripherals.as_ref().unwrap().TIM3.cr1.write(|w| w.cen().set_bit());
	});

}
#[interrupt]
fn TIM3() {
	// le timer 3 gère la durée d'extinction de la LED. 

	// ici, on va :
	//	0- stopper TIM3
	// 	1- allume la led
	//	2- démarrer TIM2

	cortex_m::interrupt::free(|cs| {
		let peripherals = MU.borrow(cs).borrow();

		// on allume la LED
		peripherals.as_ref().unwrap().GPIOA.odr.write(|w| w.odr5().high());

		// arrete TIM3 (bit CEN)
    	peripherals.as_ref().unwrap().TIM3.cr1.write(|w| w.cen().clear_bit());
    	// on remet la valeur du bit UIF à 0
        peripherals.as_ref().unwrap().TIM3.sr.write(|w| w.uif().clear_bit());

        // on demarre TIM2 (bit CEN)
		peripherals.as_ref().unwrap().TIM2.cr1.write(|w| w.cen().set_bit());
	});
}


// the program entry point
#[entry]
fn main() -> ! {

	let dp = stm32f103::Peripherals::take().unwrap();


	/****************************************************************************************/
    /*****************              ACTIVATION DES INTERUPTIONS         *********************/
    /****************************************************************************************/


	unsafe {
		stm32f103::NVIC::unmask(stm32f103::Interrupt::TIM2);
		stm32f103::NVIC::unmask(stm32f103::Interrupt::TIM3);
	}


	/****************************************************************************************/
    /*****************              ACTIVATION DES HORLOGES          ************************/
    /****************************************************************************************/


	let rcc = &dp.RCC;

	// enable the GPIOA peripheral
	rcc.apb2enr.modify(|_, w| w.iopaen().set_bit());
	// allume l'horloge correspondant au timer 2
    rcc.apb1enr.modify(|_, w| w.tim2en().set_bit());
    // allume l'horloge correspondant au timer 3
    rcc.apb1enr.modify(|_, w| w.tim3en().set_bit());


	/****************************************************************************************/
    /*****************              INITIALISATION DES GPIOS         ************************/
    /****************************************************************************************/
    

	let gpioa = &dp.GPIOA;

	// configure the pin as output
	gpioa.crl.modify(|_, w| w.mode5().output());
	// configure mode output push pull
	gpioa.crl.modify(|_, w| w.cnf5().push_pull());
	// allume la LED
	//gpioa.odr.modify(|_, w| w.odr5().set_bit());

    
	/****************************************************************************************/
    /*****************          INITIALISATION DES TIMERS          **************************/
    /****************************************************************************************/


    // Ttimer = Thorloge * (PSC + 1) * (ARR + 1)
    // On veux que la led change d'etat tout les 300 micro-secondes
    // Thorloge = 8kHz
    
    let tim2 = &dp.TIM2;    

    // configuration du pre scaler
    tim2.psc.write(|w| w.psc().bits(7999));
    // autoreload
    tim2.arr.write(|w| w.arr().bits(299));

    let tim3 = &dp.TIM3;

    // configuration du pre scaler
    tim3.psc.write(|w| w.psc().bits(7999));
    // autoreload
    tim3.arr.write(|w| w.arr().bits(rand()));


	// on met tous les peripheriques dans le mutex (move)
    cortex_m::interrupt::free(|cs| MU.borrow(cs).replace(Some(dp)));

    
    // on active les interuptions et on demarre TIM3 
    cortex_m::interrupt::free(|cs| {
    	let peripherals = MU.borrow(cs).borrow();
    	// active l'interuption de TIM2 (Bit UIE)
    	peripherals.as_ref().unwrap().TIM2.dier.write(|w| w.uie().set_bit());
    	// active l'interuption de TIM3 (Bit UIE)
    	peripherals.as_ref().unwrap().TIM3.dier.write(|w| w.uie().set_bit());
    	// demarre le timer de TIM3 (bit CEN)
    	peripherals.as_ref().unwrap().TIM3.cr1.write(|w| w.cen().set_bit());
	});


    loop {}
    	
}

fn rand() -> u16 {
	unsafe {
		static mut RANDOMSEED: u32 = 0;
		RANDOMSEED = (RANDOMSEED * 9301 + 49297) % 233280;
		800 + (RANDOMSEED % 1000) as u16
	}
}
