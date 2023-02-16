// Programme de test pour voir si il est possible de faire de l'overflow sur un registre
// On va prendre 2 registres a cote :
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
    let mut RCC_APB2ENR = 0x40021018;
    unsafe {
        // lors d'un dereferencement, doit preciser le type et donc la taille de la variable
        //*RCC_APB2ENR |= 1 << 32; // integer ne peut pas etre deference
        //*(0x40021018 as *mut u32) |= 1 << 32; // ne fonctionne pas
        *(0x40021018 as *mut u64) |= 1 << 32; // ecriture dans un autre registre
    }


    loop {}
        
}
