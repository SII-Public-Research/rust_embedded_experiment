// Programme d'exemple d'utilisation d'un UART



// A FINIR !!!
// A TESTER !!!!

// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]


use panic_semihosting as _;

use cortex_m_rt::entry;

use stm32f1::stm32f103;
//use stm32f1::stm32f103::interrupt;
//use core::cell::RefCell;
//use cortex_m::interrupt::Mutex;



// the program entry point
#[entry]
fn main() -> ! {

	let dp = stm32f103::Peripherals::take().unwrap();


	/****************************************************************************************/
    /*****************              ACTIVATION DES HORLOGES          ************************/
    /****************************************************************************************/


	let rcc = &dp.RCC;

	// choix de l'horloge à 24MHz (HSI, horloge selectionnée de base à 8MHz)
	// activation de l'horloge HSI (bit HSION du registre RCC_CR)
	// le bit HSIRDY indique si l'horloge est prête 
	rcc.cr.write(|w| w.hsion().set_bit());
	while !rcc.cr.read().hsirdy().is_ready() {}
	// configuration du multiplexeur afin de selectionner HSI en horloge (PLLSRC registre RCC_CFGR)
	// (la frequence de l'horloge est divisée par 2)
	rcc.cfgr.write(|w| w.pllsrc().hsi_div2());
	// multiplication de la frequence x6 (PLLMUL registre RCC_CFGR)
	rcc.cfgr.write(|w| w.pllmul().mul6());
	// allume l'horloge pll
	rcc.cr.write(|w| w.pllon().set_bit());
	while !rcc.cr.read().pllrdy().is_ready() {}
	// on sélectionne l'horloge PLL dans le mutliplexeur
	rcc.cfgr.write(|w| w.sw().pll());


	// enable the GPIOA peripheral
	rcc.apb2enr.modify(|_, w| w.iopaen().set_bit());
	// activation de l'horloge USART1
	rcc.apb2enr.modify(|_, w| w.usart1en().set_bit());


    /****************************************************************************************/
    /*****************              INITIALISATION DES GPIOS         ************************/
    /****************************************************************************************/
 

    let gpioa = &dp.GPIOA;


    // configuration du pin 9 en mode alternate function, push-pull
	gpioa.crh.modify(|_, w| w.mode9().output2()); // output 2MHz
	// configure mode output push pull
	gpioa.crh.modify(|_, w| w.cnf9().alt_push_pull()); // push-pull


    //configure_usart_9600bps(&dp);
    //send(16, &dp);

    let usart1 = &dp.USART1;

	// Activation de l'USART (bit UE registre CR1)
	usart1.cr1.write(|w| w.ue().set_bit());
	// Choix d'une taille de 8 bits de données (bit M du registre CR1)
	usart1.cr1.write(|w| w.m().m8());
	// Choix d'un seul bit de stop (registre CR2)
	usart1.cr2.write(|w| w.stop().stop1());
	// fixe le baud rate à 9600bps (registre BRR)
	// USARTDIV = DIV_MANTISSA + (DIV_FRACTION / 16)
	// baud rate = sysclock / 16*USARTDIV
	// ici, sysclock = 24MHz
	usart1.brr.write(|w| w.div_mantissa().bits(156)); //468
	usart1.brr.write(|w| w.div_fraction().bits(25)); //75
	// Envoi de la premiere trame d'attente (bit TE du registre CR1)
	usart1.cr1.write(|w| w.te().set_bit());
	



    loop {
    	// ecriture des donnees a transmettre dans le registre DR et on attend que TC = 1
		usart1.dr.write(|w| w.dr().bits(u16::from(b'X')));
		while !usart1.sr.read().tc().bit() {}
    }

}


/*



/**********************************************************************************************/
/*****************              CONFIGURATION DE L'USART         ******************************/
/**********************************************************************************************/
//	- 1 bit de demarrage 
//	- 8 bits de donnees
//	- 1 bit de stop
// protocole décrit dans le manuel P792
fn configure_usart_9600bps(some_dp: &stm32f103::Peripherals){
	// validation horloge USART1
	some_dp.RCC.apb2enr.modify(|_, w| w.usart1en().set_bit());
	// Activation de l'USART (bit UE registre CR1)
	some_dp.USART1.cr1.write(|w| w.ue().set_bit());
	// Choix d'une taille de 8 bits de données (bit M du registre CR1)
	some_dp.USART1.cr1.write(|w| w.m().m8());
	// Choix d'un seul bit de stop (registre CR2)
	some_dp.USART1.cr2.write(|w| w.stop().stop1());
	// fixe le baud rate à 9600bps (registre BRR)
	// USARTDIV = DIV_MANTISSA + (DIV_FRACTION / 16)
	// baud rate = sysclock / 16*USARTDIV
	// ici, sysclock = 8MHz
	some_dp.USART1.brr.write(|w| w.div_mantissa().bits(052));
	some_dp.USART1.brr.write(|w| w.div_fraction().bits(08));
	// Envoi de la premiere trame d'attente (bit TE du registre CR1)
	some_dp.USART1.cr1.write(|w| w.te().set_bit());
}

// ecriture d'une donnee
fn send(data: u16, some_dp: &stm32f103::Peripherals){
	some_dp.USART1.dr.write(|w| w.dr().bits(data));
	while !some_dp.USART1.sr.read().tc().bit() {}
}


*/