// Ajout de l'utilisation d'un interuption 
//  - LED : GPIOA pin 5
//  - TIM2

// REMARQUE 1 : l'interruption est gérée par une balise par cargo : #[interrupt]. 
// Cette balise est redéfinie par la crate de gestion des périphériques PAC. La gestion de 
// l'interruption est donc gérée par un niveau d'abstraction plus élevé même si la 
// configuration reste bas niveau. 

// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

// makes `panic!` print messages to the host stderr using semihosting
// extern crate panic_semihosting;

use cortex_m_rt::entry;
use panic_halt as _;

// the attribute comes from the device crate not from cortex-m-rt
use stm32f1::stm32f103::interrupt;

// use core::ptr; // pour passer en volatile 

/*********************************************************************************************/
/**********            CLIGNOTEMENT DE LA LED AVEC UNE INTERUPTION               *************/
/*********************************************************************************************/

#[interrupt]
fn TIM2() {
    unsafe {
        // on remet la valeur du bit UIF à 0
        *(0x40000010 as *mut u32) &= !(0x01 << 0);
        // on inverse la valeur de la led avec une opération XOR
        *(0x4001080C as *mut u32) ^= 0x01 << 5;
    }
}

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
        *(RCC_APB2ENR as *mut u32) = *(RCC_APB2ENR as *mut u32) | (1 << 2);
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

    // Ttimer = Thorloge * (PSC + 1) * (ARR + 1)

    // alimentation des periphériques TIMER2 (RCC_APB1ENR)
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
    //const TIM2_SR: u32 = TIM2 + 0x10;
    //      - PSC :  0x28   prescaler
    const TIM2_PSC: u32 = TIM2 + 0x28;
    //      - ARR :  0x2C   autoreload
    const TIM2_ARR: u32 = TIM2 + 0x2C;  
    //      - DIER :  0x0C   interuption
    const TIM2_DIER: u32 = TIM2 + 0x0C;

    // Paramétrage du TIMER 2 (1 seconde)
    unsafe {
        *(TIM2_ARR as *mut u32) = 999;
        *(TIM2_PSC as *mut u32) = 7199;
        *(TIM2_DIER as *mut u32) |= 1 << 0; // active l'interuption (Bit UIE)
        *(TIM2_CR1 as *mut u32) |= 1 << 0; // demarre le timer (bit CEN)
    }

    /****************************************************************************************/
    /*****************              GESTION DE L'INTERUPTION         ************************/
    /****************************************************************************************/

    // L'interuption sur le timer 2 est identifiée par le numero 28 (p197-199)

    // adresse NVIC : 0xE000E100 
    const NVIC: u32 = 0xE000E100;

    // (offsets des registres)
    //      - ISER[0] :  0x00   set-enable register
    const NVIC_ISER: u32 = NVIC + 0x00;

    unsafe {
        *(NVIC_ISER as *mut u32) |= 1 << 28; // active l'interuption (numero 28)
    }

    loop {}
        
}

