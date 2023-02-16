// Programme d'exemple d'utilisation d'un ADC (ADC1, voie 8 -> broche PB0)
// Fonctionnement d'un ADC :
//  1- mise en service
//  2- demarrage : lancement de la conversion
//  3- drapeau signalant la fin de la conversion

// modes possibles :
//  1- single conversion 
//  2- mode continu (pour differer le traitement de la mesure, utilisation de DMA)

// A FINIR !!!
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

    // on va choisir d'utiliser une horloge systeme à 8MHz. 
    // Je pense que c'est d'ailleur la config de base
    rcc.cr.write(|w| w.hsion().set_bit());

    // enable the GPIOA peripheral
    rcc.apb2enr.modify(|_, w| w.iopben().set_bit());
    // allume l'horloge correspondant à l'ADC 1
    rcc.apb2enr.modify(|_, w| w.adc1en().set_bit());


    /****************************************************************************************/
    /*****************              INITIALISATION DES GPIOS         ************************/
    /****************************************************************************************/
    

    let gpiob = &dp.GPIOB;

    // configurartion du pin 0 en mode input
    gpiob.crl.modify(|_, w| w.mode0().input());
    // configure mode analog input
    gpiob.crl.modify(|_, w| w.cnf0().push_pull());


    /****************************************************************************************/
    /*****************             INITIALISATION DE L'ADC          *************************/
    /****************************************************************************************/

    let adc1 = &dp.ADC1;

    // on active l'ADC bit ADON du registre CR2
    adc1.cr2.write(|w| w.adon().set_bit());
    // on choisit le nombre de voies à utiliser (ici 0 pour une voie) SQR1_L
    adc1.sqr1.write(|w| w.l().bits(0));
    // on définit la voie à convertir (voie 8) SQR3_8 (premiere convertion)
    adc.sqr3.write(|w| w.sq1().bits(8));
    // lancement de la calibration de l'ADC CR2_CAL (et attendre qu'il repasse à 0)
    adc.cr2.write(|w| w.cal().start());
    while(adc.cr2.read().cal().is_not_complete()) {};

    // pour lancer une calibration,  il faut repasser le bit ADON à 1
    adc1.cr2.write(|w| w.adon().set_bit()) {};
    // on attend la fin de la conversion (bit EOC du registre SR)
    while(adc.sr.read().eoc().is_not_complete());
    // on recupère la valeur de la conversion dans le registre DR
    let temp = adc.dr.read().data();


    loop {}
        
}
