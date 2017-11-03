//! Prints "Hello, world!" on the OpenOCD console using semihosting

#![feature(used)]
#![no_std]
#![feature(asm)]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate volatile_register;

use cortex_m::asm;
use volatile_register::{RO, WO, RW};

const GPIOA_BASE: u32 = 0x4002_0000;
const RCC_BASE: u32 = 0x4002_3800;

const RCC_AHB1ENR_GPIOAEN: u32 = 0x0000_0001;

#[allow(dead_code)]
enum GpioModer {
    Input = 0b00,
    Output = 0b01,
    Alternate = 0b10,
    Analog = 0b11,
}

#[allow(dead_code)]
enum GpioTyper {
    PushPull = 0b0,
    OpenDrain = 0b1,
}

#[allow(dead_code)]
enum GpioOspeedr {
    LowSpeed = 0b00,
    MediumSpeed = 0b01,
    FastSpeed = 0b10,
    HighSpeed = 0b11,
}

#[allow(dead_code)]
enum GpioPupdr {
    NoPuPd = 0b00,
    PullUp = 0b01,
    PuuDown = 0b10,
}

#[repr(C)]
struct RCC {
    cr: RW<u32>,
    pllcfgr: RW<u32>,
    cfgr: RW<u32>,
    cir: RW<u32>,
    ahb1rstr: RW<u32>,
    ahb2rstr: RW<u32>,
    reserved0: [u32; 2],
    apb1rstr: RW<u32>,
    apb2rstr: RW<u32>,
    reserved1: [u32; 2],
    ahb1enr: RW<u32>,
    ahb2enr: RW<u32>,
    reserved2: [u32; 2],
    apb1enr: RW<u32>,
    apb2enr: RW<u32>,
    reserved3: [u32; 2],
    ahb1lpenr: RW<u32>,
    ahb2lpenr: RW<u32>,
    reserved4: [u32; 2],
    apb1lpenr: RW<u32>,
    apb2lpenr: RW<u32>,
    reserved5: [u32; 2],
    bdcr: RW<u32>,
    csr: RW<u32>,
    reserved6: [u32; 2],
    sscgr: RW<u32>,
    plli2scfgr: RW<u32>,
    reserved7: u32,
    dckcfgr: RW<u32>,
}

#[repr(C)]
struct GPIOA {
    moder: RW<u32>,
    otyper: RW<u32>,
    ospeedr: RW<u32>,
    pupdr: RW<u32>,
    idr: RO<u32>,
    odr: RW<u32>,
    bsrr: WO<u32>,
    lckr: RW<u32>,
    afrl: RW<u32>,
    afrh: RW<u32>,
}

fn delay() {
    for _ in 1..1000 {
        unsafe { asm!("") }
    }
}

fn main() {
    let rcc = RCC_BASE as *const RCC;
    let gpioa = GPIOA_BASE as *const GPIOA;
    const PIN_A5: u32 = 1 << 5;

    unsafe {
        (*rcc).ahb1enr.write(
            (*rcc).ahb1enr.read() | RCC_AHB1ENR_GPIOAEN,
        );
        (*gpioa).moder.write(
            ((*gpioa).moder.read() & !((0b11 as u32) << (5 * 2))) |
                ((GpioModer::Output as u32) << (5 * 2)),
        );
        (*gpioa).otyper.write(
            ((*gpioa).otyper.read() & !((0b1 as u32) << 5)) |
                ((GpioTyper::PushPull as u32) << 5),
        );
        (*gpioa).ospeedr.write(
            ((*gpioa).ospeedr.read() & !((0b11 as u32) << (5 * 2))) |
                ((GpioOspeedr::HighSpeed as u32) << (5 * 2)),
        );
        (*gpioa).pupdr.write(
            ((*gpioa).pupdr.read() & !((0b11 as u32) << (5 * 2))) |
                (GpioPupdr::NoPuPd as u32) << (5 * 2),
        );
    }

    loop {
        unsafe {
            (*gpioa).odr.write((*gpioa).odr.read() ^ PIN_A5);
        }
        delay();
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
