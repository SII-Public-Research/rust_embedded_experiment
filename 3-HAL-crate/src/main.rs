
//#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_semihosting::hprintln;

use stm32f1::stm32f103;
use stm32f1::stm32f103::interrupt;


use cortex_m_rt::entry;
use stm32f1xx_hal::{
    pac,
};

use cortex_m::peripheral::DWT;

static mut tableau: [u32; 8] = [0; 8];
static mut COUNT: u32 = 0;


#[interrupt]
fn EXTI15_10() {

    static mut NEW_COUNT: u32 = 0;
    *NEW_COUNT = DWT::get_cycle_count();
    let temps: u32 = *NEW_COUNT / 8;

    unsafe {
        // on inverse la valeur de la led avec une opération XOR
        *(0x4001080C as *mut u32) ^= 0x01 << 5;
        // on met à 1 le bit 13 du registre pr pour valider l'interuption 
        *(0x40010414 as *mut u32) |= 1 << 13;
        // je remet a 0 le counter DWT
        *(0xE0001004 as *mut u32) = 0;

    }
    unsafe {
        if COUNT == 0 {
            COUNT += 1;
        } else if COUNT < 9 {
            tableau[(COUNT as usize) - 1] = temps;
            COUNT += 1;
        } else {
            hprintln!("frame : {:?} ", tableau).unwrap();
            COUNT = 0;
        }
    }
    
    //hprintln!("{un petit front descendant vient de pointer !}").unwrap();
    //hprintln!("temps entre 2 fronts descendants : {} µs", temps).unwrap();
}

#[entry]
fn main() -> ! {

    let dp = pac::Peripherals::take().unwrap();
    let mut cp = pac::CorePeripherals::take().unwrap();

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
    // enable the GPIOA peripheral
    rcc.apb2enr.modify(|_, w| w.iopaen().set_bit());


    /****************************************************************************************/
    /*****************              INITIALISATION DES GPIOS         ************************/
    /****************************************************************************************/
    
    let gpioc = &dp.GPIOC;

    // configure le pin en input
    gpioc.crh.modify(|_, w| w.mode13().input());
    // configure le mode input en open_drain
    gpioc.crh.modify(|_, w| w.cnf13().open_drain());

    let gpioa = &dp.GPIOA;

    // configure the pin as output
    gpioa.crl.modify(|_, w| w.mode5().output());
    // configure mode output push pull
    gpioa.crl.modify(|_, w| w.cnf5().push_pull());

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


    //hprintln!("{config termine !}").unwrap();

    cp.DWT.enable_cycle_counter();

    //let mut flash = dp.FLASH.constrain();
    //let mut rcc = dp.RCC.constrain();
    //let clocks = rcc.cfgr.freeze(&mut flash.acr);
    //let timer = MonoTimer::new(cp.DWT, clocks);
    // 0xE000_1000

    //let t = timer.now().elapsed();
    //hprintln!("nombre de ticks : {}", t).unwrap();
    //let freq = timer.frequency();
    //hprintln!("frequence : {:?} Hertz", freq.0).unwrap();
    loop {}
        
}
