// Ajout de l'utilisation du timer TIM2 (basic timer)
//  - LED : GPIOA pin 5


// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

// makes `panic!` print messages to the host stderr using semihosting
// extern crate panic_semihosting;

use cortex_m_rt::entry;
use panic_halt as _;
// use core::ptr; // pour passer en volatile 

// the program entry point
#[entry]
fn main() -> ! {

    /****************************************************************************************/
    /*****************              INITIALISATION DES GPIOS         ************************/
    /****************************************************************************************/

    // alimentation des periphériques GPIO (RCC_APB2ENR)
    // adresse RCC : 0x40021000
    // offset APB2ENR : 0x18
    // IOPAEN : Bit 2
    const RCC_APB2ENR: u32 = 0x40021018;
    unsafe {
        *(RCC_APB2ENR as *mut u32) = *(RCC_APB2ENR as *mut u32) | (1 << 2) | (1 << 4);
    }


    // l'adresse du GPIOA est : 0x40010800
    const GPIOA: u32 = 0x40010800;

    // (offsets des registres)
    //      - CRL :  0x00   configuration
    const GPIOA_CRL: u32 = GPIOA + 0x00;
    //      - CRH :  0x04   configuration
    //      - IDR :  0x08   lecture
    //      - ODR :  0x0C   ecriture
    const GPIOA_ODR: u32 = GPIOA + 0x0C;
    //      - BSRR : 0x10   action SET
    //      - BRR :  0x14   action RESET
    //      - LCKR : 0x18   

    unsafe {
        // registre CRL, configuration de la LED en output push-pull : b0001
        *(GPIOA_CRL as *mut u32) = 1 << 20; // (page 171 du manuel RM0008)
    }

    /****************************************************************************************/
    /*****************              INITIALISATION DU TIMER          ************************/
    /****************************************************************************************/


    // alimentation des periphériques TIM2 (RCC_APB1ENR)
    // adresse RCC : 0x40021000
    // offset APB1ENR : 0x1C
    // TIM2EN : Bit 0
    const RCC_APB1ENR: u32 = 0x4002101C;
    unsafe {
        *(RCC_APB1ENR as *mut u32) |= 1 << 0;
    }

    // l'adresse de TIM2 est : 0x4000 0000 
    const TIM2: u32 = 0x40000000;

    // (offsets des registres)
    //      - CR1 :  0x00   control register
    const TIM2_CR1: u32 = TIM2 + 0x00;
    //      - SR  :  0x10   Status register
    const TIM2_SR: u32 = TIM2 + 0x10;
    //      - PSC :  0x28   prescaler
    const TIM2_PSC: u32 = TIM2 + 0x28;
    //      - ARR :  0x2C   autoreload
    const TIM2_ARR: u32 = TIM2 + 0x2C;  

    // Paramétrage du TIMER 2 (10 seconde) REVOIR LE CALCUL POUR 1 SEC
    unsafe {
        *(TIM2_ARR as *mut u32) = 9999;
        *(TIM2_PSC as *mut u32) = 7199;
        *(TIM2_CR1 as *mut u32) |= 1 << 0; // demarre le timer
    }


    loop {

        unsafe {
            // on regarde si le timer a débordé (bit UIF)
            if (*(TIM2_SR as *mut u32) & 0x01 << 0) == 1 {
                // si oui, on remet la valeur du bit UIF à 0
                *(TIM2_SR as *mut u32) &= !(0x01 << 0);
                // on inverse la valeur de la led avec une opération XOR
                *(GPIOA_ODR as *mut u32) ^= 0x01 << 5;
            }
        }

    }
        
}
