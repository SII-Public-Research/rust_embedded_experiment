// On va faire un exemple d'interruption sur la broche 13 du GPIO C

// c'est une interuption externe (utilisation du registre EXTI3)
// registre IMR (validation de l'it externe des ports 13)
// registre RTSR et FTSA (leve une it sur front descendant)

// ETAPE 1 : configuration des horloges à utiliser (RCC)
// ETAPE 2 : configuraiton du GPIO à utiliser (input et afio)
// ETAPE 3 : configuration du type d'EXTI (interuption + front descendant)

// PENDING REGISTER à mettre à 1 pour reset l'interuption

#![no_main]
#![no_std]

use panic_semihosting as _;
use cortex_m_semihosting::hprintln;

use cortex_m_rt::entry;
use stm32f1::stm32f103;

// Gestion des interruptions
use stm32f1::stm32f103::interrupt;


#[interrupt]
fn EXTI15_10() {
	static mut COUNT: u32 = 0;
    *COUNT += 1;

    // un petit printf ?? OUI
    hprintln!("{un petit front descendant vient de pointer !}").unwrap();
    hprintln!("c'est la {}ieme fois", COUNT).unwrap();

    unsafe {
        // on met à 1 le bit 13 du registre pr pour valider l'interuption 
        *(0x40010414 as *mut u32) |= 1 << 13;
    }

}

#[entry]
fn main() -> ! {

    let dp = stm32f103::Peripherals::take().unwrap();

    /****************************************************************************************/
    /*****************              ACTIVATION DE L'INTERUPTION         *********************/
    /****************************************************************************************/

    unsafe {
        stm32f103::NVIC::unmask(stm32f103::Interrupt::EXTI15_10);
    }

    /****************************************************************************************/
    /*****************              ACTIVATION DES HORLOGES          ************************/
    /****************************************************************************************/

    let rcc = &dp.RCC;

    // allume le GPIOC
    rcc.apb2enr.modify(|_, w| w.iopcen().set_bit());
    // allume les fonctions alternatives 
    rcc.apb2enr.modify(|_, w| w.afioen().set_bit());


    /****************************************************************************************/
    /*****************              INITIALISATION DES GPIOS         ************************/
    /****************************************************************************************/
    
    let gpioc = &dp.GPIOC;

    // configure le pin en input
    gpioc.crh.modify(|_, w| w.mode13().input());
    // configure le mode input en open_drain
    gpioc.crh.modify(|_, w| w.cnf13().open_drain());

    /****************************************************************************************/
    /*****************              INITIALISATION DE L'INTERUPTION         *****************/
    /****************************************************************************************/
    
    let afio = &dp.AFIO;

    unsafe {
    	// selectionne le GPIOC pin 13 en source
    	afio.exticr4.write(|w| w.exti13().bits(0x02));
    }

    let exti = &dp.EXTI;

    // active les interuptions sur les pin 13
    exti.imr.write(|w| w.mr13().set_bit());
    // leve une interuption sur les fronts descendants
    exti.ftsr.write(|w| w.tr13().set_bit());


    hprintln!("{config termine !}").unwrap();


    loop {}
        
}
