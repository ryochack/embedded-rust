//! Prints "Hello, world!" on the OpenOCD console using semihosting
#![no_std]

extern crate stm32f401re;

use stm32f401re::peripheral::{rcc, gpio};

fn main() {
    let rcc = rcc::RCC_BASE as *const rcc::RegisterMap;
    let gpioa = gpio::GPIOA_BASE as *const gpio::RegisterMap;
    const PIN_A5: u32 = 1 << 5;

    unsafe {
        (*rcc).ahb1enr.modify(
            |v| v | rcc::ahb1enr::Gpioaen::Enable as u32,
        );
        (*gpioa).moder.modify(|v| {
            (v & !((0b11 as u32) << (5 * 2))) | ((gpio::moder::Modery::Output as u32) << (5 * 2))
        });
        (*gpioa).otyper.modify(|v| {
            (v & !((0b1 as u32) << 5)) | ((gpio::otyper::Oty::PushPull as u32) << 5)
        });
        (*gpioa).ospeedr.modify(|v| {
            (v & !((0b11 as u32) << (5 * 2))) | ((gpio::ospeedr::Ospeedr::High as u32) << (5 * 2))
        });
        (*gpioa).pupdr.modify(|v| {
            (v & !((0b11 as u32) << (5 * 2))) | (gpio::pupdr::Pupdr::NoPuPd as u32) << (5 * 2)
        });
    }

    loop {
        unsafe {
            (*gpioa).odr.modify(|v| v ^ PIN_A5);
        }
        stm32f401re::delay(1000);
    }
}
