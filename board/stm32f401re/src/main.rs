//! Prints "Hello, world!" on the OpenOCD console using semihosting
#![no_std]

extern crate stm32f401re;

use stm32f401re::peripheral::{rcc, gpio};

fn main() {
    let rcc = rcc::RCC_BASE as *const rcc::RegisterMap;
    let gpioa = gpio::GPIOA_BASE as *const gpio::RegisterMap;
    const PIN_A5: u32 = 1 << 5;

    unsafe {
        (*rcc).ahb1enr.write(
            (*rcc).ahb1enr.read() |
                rcc::RCC_AHB1ENR_GPIOAEN,
        );
        (*gpioa).moder.write(
            ((*gpioa).moder.read() & !((0b11 as u32) << (5 * 2))) |
                ((gpio::Moder::Output as u32) << (5 * 2)),
        );
        (*gpioa).otyper.write(
            ((*gpioa).otyper.read() & !((0b1 as u32) << 5)) |
                ((gpio::Typer::PushPull as u32) << 5),
        );
        (*gpioa).ospeedr.write(
            ((*gpioa).ospeedr.read() & !((0b11 as u32) << (5 * 2))) |
                ((gpio::Ospeedr::High as u32) << (5 * 2)),
        );
        (*gpioa).pupdr.write(
            ((*gpioa).pupdr.read() & !((0b11 as u32) << (5 * 2))) |
                (gpio::Pupdr::NoPuPd as u32) << (5 * 2),
        );
    }

    loop {
        unsafe {
            (*gpioa).odr.write((*gpioa).odr.read() ^ PIN_A5);
        }
        stm32f401re::delay(1000);
    }
}
