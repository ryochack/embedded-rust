#![no_std]
#![feature(asm)]
#![feature(used)]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate volatile_register;

use cortex_m::asm;

pub mod peripheral;

pub fn delay(ticks: u32) {
    for _ in 1..ticks {
        unsafe { asm!("") }
    }
}

// As we are not using interrupts, we just register a dummy catch all handler
#[allow(dead_code)]
#[used]
#[link_section = ".vector_table.interrupts"]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
