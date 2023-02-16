// Programme de test pour voir si il est possible de faire de l'overflow sur un registre
// On va prendre 2 registres à coté :
//  - RCC adresse : 0x40021000
//      - AHBENR adresse  : 0x14
//      - APB2ENR adresse : 0x18
//      - APB1ENR adresse : 0x1C


// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]


use cortex_m_rt::entry;
use panic_halt as _;


#[entry]
fn main() -> ! {


	// IOPAEN : Bit 2 du registre APB2ENR
	//const RCC_APB2ENR: u32 = 0x40021018;
	unsafe {
		*(RCC_APB2ENR as *mut u32) |= | (1 << 2);
	}



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
