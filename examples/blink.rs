#![no_main]
#![no_std]


#[macro_use] extern crate cortex_m_rt;
#[macro_use] extern crate nb;

extern crate dwm1001;
extern crate panic_semihosting;


use dwm1001::{
    nrf52_hal::{
        prelude::*,
        nrf52::Peripherals,
        timer::Timer,
    },
};


entry!(main);

fn main() -> ! {
    let p = Peripherals::take().unwrap();

    let mut p0_14 = p.P0
        .split()
        .p0_14
        .into_push_pull_output();

    let mut timer = p.TIMER0.constrain();

    loop {
        // Set P0.14 to LOW, thereby enabling the LED
        p0_14.set_low();

        delay(&mut timer, 20_000); // 20ms

        // Set P0.14 to HIGH, thereby disabling the LED
        p0_14.set_high();

        delay(&mut timer, 230_000); // 230ms
    }
}


fn delay<T>(timer: &mut Timer<T>, cycles: u32) where T: TimerExt {
    timer.start(cycles);
    block!(timer.wait());
}