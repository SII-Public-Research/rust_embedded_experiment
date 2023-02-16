// Utilisation des GPIOS, timers et interruption
//  - LED : GPIOA pin 5
//  - BUTTON : GPIOC pin 13
//  - TIM2

// Phase 1 : on va faire clignoter la LED toutes les secondes.

// REMARQUE 1 : Utilisation des mutex quasiment obligatoire pour pouvoir utiliser les 
// périphériques déclarés en global. Obligatoire pour rester dans du "safe RUST".
// Il est aussi possible de recreer des variables dans la fonction d'interuption mais cela
// a l'air moins propre.

// REMARQUE 2 : impossible de différencier les périphériques dans plusieurs MUTEX (en tout cas 
// je n'ai pas réussi) donc création d'un mutex comprenant tout les peripheriques !! 

// REMARQUE 3 : J'ai toujours du code unsafe en voulant utiliser une operation XOR !!!

// REMARQUE 4 : Encore du code unsafe sur l'activation de l'interuption 

// REMARQUE 5 : Attention à la difference entre les methodes (.) et les fonctions propres (::)
// (voir l'activation de l'interuption en debut de code).

// REMARQUE 6 : premiere utilisation de la crate cortex-m (Utilisation des Mutex)

// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]


use panic_semihosting as _;
use cortex_m_semihosting::hprintln;

use cortex_m_rt::entry;

use stm32f1::stm32f103;
use stm32f1::stm32f103::interrupt;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;



// déclaration des mutex vides dans des sections critiques 
static MU: Mutex<RefCell<Option<stm32f103::Peripherals>>> = Mutex::new(RefCell::new(None));

// le code en dessous pose problème au niveau du borrowing de la variable dp :
//static MU_TIM2: Mutex<RefCell<Option<stm32f103::TIM2>>> = Mutex::new(RefCell::new(None));
//static MU_GPIOA: Mutex<RefCell<Option<stm32f103::GPIOA>>> = Mutex::new(RefCell::new(None));


/*********************************************************************************************/
/**********            CLIGNOTEMENT DE LA LED AVEC UNE INTERUPTION               *************/
/*********************************************************************************************/


#[interrupt]
fn TIM2() {
    hprintln!("{on est dans l'interuption de TIM2 !}").unwrap();
    cortex_m::interrupt::free(|cs| {
        let peripherals = MU.borrow(cs).borrow();
        // on remet la valeur du bit UIF à 0
        peripherals.as_ref().unwrap().TIM2.sr.write(|w| w.uif().clear_bit());
        // on inverse la valeur de la led avec une opération XOR
        // OPERATION UNSAFE !!!!
        unsafe {
            peripherals.as_ref().unwrap().GPIOA.odr.write(|w| w
                .bits(peripherals.as_ref().unwrap().GPIOA.odr.read().bits() ^ 0x01 << 5));
        }
    });
}

// the program entry point
#[entry]
fn main() -> ! {

    let dp = stm32f103::Peripherals::take().unwrap();


    /****************************************************************************************/
    /*****************              ACTIVATION DE L'INTERUPTION         *********************/
    /****************************************************************************************/


    unsafe {
        stm32f103::NVIC::unmask(stm32f103::Interrupt::TIM2);
    }


    /****************************************************************************************/
    /*****************              ACTIVATION DES HORLOGES          ************************/
    /****************************************************************************************/


    let rcc = &dp.RCC;

    // enable the GPIOA peripheral
    rcc.apb2enr.modify(|_, w| w.iopaen().set_bit());
    // allume l'horloge correspondant au timer 2
    rcc.apb1enr.modify(|_, w| w.tim2en().set_bit());


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
    // On veux que la led change d'etat tout les 125 micro-secondes
    // Thorloge = 8kHz
    
    let tim2 = &dp.TIM2;    

    // autoreload
    tim2.arr.write(|w| w.arr().bits(999));
    // configuration du pre scaler
    tim2.psc.write(|w| w.psc().bits(7999));




    // on met tous les peripheriques dans le mutex (move)
    cortex_m::interrupt::free(|cs| MU.borrow(cs).replace(Some(dp)));

    
    cortex_m::interrupt::free(|cs| {
        let peripherals = MU.borrow(cs).borrow();
        // active l'interuption (Bit UIE)
        peripherals.as_ref().unwrap().TIM2.dier.write(|w| w.uie().set_bit());
        // demarre le timer (bit CEN)
        peripherals.as_ref().unwrap().TIM2.cr1.write(|w| w.cen().set_bit());
    });


    loop {}
        
}
