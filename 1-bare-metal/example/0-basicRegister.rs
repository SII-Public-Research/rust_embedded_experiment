// Programme d'exemple pour l'utilisation de registres du micro STM32F103
// 	- LED : GPIOA pin 5

// REMARQUE 1 : la manipulation des registres est considérée comme du code UNSAFE car il est
// possible de faire des dépassements de mémoire (exemple particulier pour les TIMERS qui 
// necessitent ce dépassement).

// REMARQUE 2 : LLVM optimise le code et supprime toutes les opérations sur les registres 
// sauf la dernière. Il faut donc passer en VOLATILE quand on fait plusieurs opérations sur
// un meme registre.

// REMARQUE 3 : attention à la portée des variables - ne déclarez pas vos variables dans une
// boucle unsafe.

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

	// alimentation du periphérique GPIOA (RCC_APB2ENR_IOPAEN)
	// adresse RCC : 0x40021000
	// offset APB2ENR : 0x18
	// IOPAEN : Bit 2
	// IOPCEN : Bit 4
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
    // const GPIOA_BSRR: u32 = GPIOA + 0x10;
    //      - BRR :  0x14   action RESET
    // const GPIOA_BRR: u32 = GPIOA + 0x14;
    //      - LCKR : 0x18   


    unsafe {
        // Configuration du port 5 :
        // registre CRL, configuration en input floating : b0100
        //*(GPIOA_CRL as *mut u32) = 1 << 22; // (page 171 du manuel RM0008)
        //*(GPIOA_CRL as *mut u32) = *(GPIOA_CRL as *mut u32) & !(0xF << 20); // Mise à 0 des bits b23 b22 b21 b20
        //*(GPIOA_CRL as *mut u32) = *(GPIOA_CRL as *mut u32) | (0x1 << 22); // Mise à 1 du bit b22
        // ptr::write_volatile(GPIOA_CRL as *mut u32, 1 << 22);

        // registre CRL, configuration de la LED en output push-pull : b0001
        *(GPIOA_CRL as *mut u32) = 1 << 20; // (page 171 du manuel RM0008)

        // lecture du registre IDR (bit 5) :
        // let value_idr = (*(GPIOA_IDR as *mut u32) & (0x1 << 5)) >> 5;

        // ecriture du registre ODR (bit 5) :
        *(GPIOA_ODR as *mut u32) = *(GPIOA_ODR as *mut u32) | (0x1 << 5);

        // ecriture du registre BSRR (bit 5) :
        // ptr::write_volatile(GPIOA_BSRR as *mut u32, 1 << 5);
    }

    loop {}
    	
}
