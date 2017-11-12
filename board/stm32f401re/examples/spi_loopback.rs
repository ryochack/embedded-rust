#![no_std]

extern crate stm32f401re;
extern crate cortex_m;

use stm32f401re::peripheral::{rcc, gpio, flash, pwr, spi};
use cortex_m::peripheral;
use cortex_m::peripheral::{NVIC, SCB, SYST};

enum Irq {
    MemoryManagement = -12,
    BusFault = -11,
    UsageFault = -10,
    SVCall = -5,
    DebugMonitor = -4,
    PendSV = -2,
    SysTick = -1,
}

fn nvic_get_priority_grouping() -> u32 {
    const PRIGROUP_MASK: u32 = 7 << 8;
    (unsafe { (*SCB.get()).aircr.read() } & PRIGROUP_MASK) >> 8
}

fn nvic_encode_priority(priority_group: u32, preempt_priority: u32, sub_priority: u32) -> u32 {
    let nvic_prio_bits: u32 = 4;
    let pg: u32 = priority_group & 0x07;
    let ppb: u32 = if (7 - pg) > nvic_prio_bits {
        nvic_prio_bits
    } else {
        pg + nvic_prio_bits
    };
    let spb: u32 = if (pg + nvic_prio_bits) < 7 {
        0
    } else {
        pg - 7 + nvic_prio_bits
    };
    (preempt_priority & ((1 << ppb) - 1)) << spb | (sub_priority & ((1 << spb) - 1))
}

fn nvic_set_priority(irqn: i32, priority: u32) {
    let nvic_prio_bits: u32 = 4;

    if irqn < 0 {
        let systick_irqn: u32 = irqn as u32;
        unsafe {
            (*SCB.get()).shpr[((systick_irqn & 0x0F) - 4) as usize]
                .write(((priority << (8 - nvic_prio_bits)) & 0xFFu32) as u8);
        }
    } else {
        unsafe {
            (*NVIC.get()).ipr[irqn as usize].write(
                (priority << (8 - nvic_prio_bits) & (0xFFu32)) as
                    u8,
            );
        }
    }
}

fn ll_init() {
    /* Set Priority Grouping */
    let nvic_prioritygroup_0: u32 = 0x00000007;
    unsafe {
        (*SCB.get()).aircr.modify(|v| {
            (v & !((0xFFFFu32 << 16) | (7u32 << 8))) | (0x5FAu32 << 16) |
                (nvic_prioritygroup_0 << 8)
        });
    }

    nvic_set_priority(
        Irq::MemoryManagement as i32,
        nvic_encode_priority(nvic_get_priority_grouping(), 0, 0),
    );
    nvic_set_priority(
        Irq::BusFault as i32,
        nvic_encode_priority(nvic_get_priority_grouping(), 0, 0),
    );
    nvic_set_priority(
        Irq::UsageFault as i32,
        nvic_encode_priority(nvic_get_priority_grouping(), 0, 0),
    );
    nvic_set_priority(
        Irq::SVCall as i32,
        nvic_encode_priority(nvic_get_priority_grouping(), 0, 0),
    );
    nvic_set_priority(
        Irq::DebugMonitor as i32,
        nvic_encode_priority(nvic_get_priority_grouping(), 0, 0),
    );
    nvic_set_priority(
        Irq::PendSV as i32,
        nvic_encode_priority(nvic_get_priority_grouping(), 0, 0),
    );
    nvic_set_priority(
        Irq::SysTick as i32,
        nvic_encode_priority(nvic_get_priority_grouping(), 0, 0),
    );
}

fn system_clock_config() {
    let flash = flash::FLASH_BASE as *const flash::RegisterMap;
    let pwr = pwr::PWR_BASE as *const pwr::RegisterMap;
    let rcc = rcc::RCC_BASE as *const rcc::RegisterMap;

    unsafe {
        (*flash).acr.modify(
            |v| (v & !(flash::acr::LATENCY_MASK as u32)) | 2,
        );
        (*pwr).cr.modify(|v| {
            (v & !(pwr::cr::VOS_MASK)) | pwr::cr::Vos::Scale2Mode as u32
        });
        (*rcc).cr.modify(|v| {
            (v & !(rcc::cr::HSITRIM_MASK)) | (16 << rcc::cr::HSITRIM_SHIFT)
        });
        (*rcc).cr.modify(|v| (v | rcc::cr::Hsion::On as u32));
    }

    /* Wait till HSI is ready */
    loop {
        if (unsafe { (*rcc).cr.read() } & rcc::cr::Hsirdy::Ready as u32) ==
            rcc::cr::Hsirdy::Ready as u32
        {
            break;
        }
    }

    unsafe {
        (*rcc).pllcfgr.modify(|v| {
            (v & !(rcc::pllcfgr::PLLSRC_MASK | rcc::pllcfgr::PLLM_MASK | rcc::pllcfgr::PLLN_MASK)) |
                (rcc::pllcfgr::Pllsrc::HsiClock as u32 | 0x10u32 | 336 << rcc::pllcfgr::PLLN_SHIFT |
                     (rcc::pllcfgr::Pllp::_4 as u32))
        });
        (*rcc).cr.modify(|v| v | rcc::cr::PLLON_MASK);
    }

    /* Wait till PLL is ready */
    loop {
        if (unsafe { (*rcc).cr.read() } & rcc::cr::Pllrdy::Locked as u32) ==
            rcc::cr::Pllrdy::Locked as u32
        {
            break;
        }
    }

    unsafe {
        (*rcc).cfgr.modify(|v| {
            (v & !rcc::cfgr::HPRE_MASK) | rcc::cfgr::Hpre::Div1 as u32
        });
        (*rcc).cfgr.modify(|v| {
            (v & !rcc::cfgr::PPRE1_MASK) | rcc::cfgr::Ppre1::Div2 as u32
        });
        (*rcc).cfgr.modify(|v| {
            (v & !rcc::cfgr::PPRE2_MASK) | rcc::cfgr::Ppre2::Div1 as u32
        });
        (*rcc).cfgr.modify(|v| {
            (v & !rcc::cfgr::SW_MASK) | rcc::cfgr::Sw::Pll as u32
        });
    }

    /* Wait till System clock is ready */
    loop {
        let cfgr = unsafe { (*rcc).cfgr.read() };
        if cfgr & rcc::cfgr::SWS_MASK == rcc::cfgr::Sws::Pll as u32 {
            break;
        }
        /*
        if unsafe { (*rcc).cfgr.read() } & rcc::cfgr::SWS_MASK == rcc::cfgr::Sws::Pll as u32 {
            break;
        }
        */
    }

    /* Initialize 1ms ticks */
    let hclk_frequency: u32 = 84000000;
    let ticks: u32 = 1000;
    unsafe {
        (*SYST.get()).set_reload((hclk_frequency / ticks) - 1);
        (*SYST.get()).clear_current();
        (*SYST.get()).set_clock_source(peripheral::SystClkSource::Core);
        (*SYST.get()).enable_counter();
    }

    /* Set timer clock prescaler */
    unsafe {
        (*rcc).dckcfgr.modify(|v| {
            (v & !rcc::dckcfgr::TIMPRE_MASK) | rcc::dckcfgr::Timpre::X2 as u32
        });
    }

    /* SysTick_IRQn interrupt configuration */
    nvic_set_priority(
        Irq::SysTick as i32,
        nvic_encode_priority(nvic_get_priority_grouping(), 0, 0),
    );
}

fn gpio_init() {
    let rcc = rcc::RCC_BASE as *const rcc::RegisterMap;
    let gpioa = gpio::GPIOA_BASE as *const gpio::RegisterMap;
    //let gpiob = gpio::GPIOB_BASE as *const gpio::RegisterMap;
    //let gpioc = gpio::GPIOC_BASE as *const gpio::RegisterMap;
    //let gpioh = gpio::GPIOH_BASE as *const gpio::RegisterMap;

    unsafe {
        (*rcc).ahb1enr.modify(|v| {
            v |
                (rcc::ahb1enr::Gpioaen::Enable as u32 | rcc::ahb1enr::Gpioben::Enable as u32 |
                     rcc::ahb1enr::Gpiocen::Enable as u32 |
                     rcc::ahb1enr::Gpiohen::Enable as u32)
        });
    }

    /* GPIOA */
    const PIN_A5: u32 = 5;
    unsafe {
        (*gpioa).moder.modify(|v| {
            (v & !((0b11 as u32) << (PIN_A5 * 2))) |
                ((gpio::moder::Modery::Output as u32) << (PIN_A5 * 2))
        });
        (*gpioa).otyper.modify(|v| {
            (v & !((0b1 as u32) << PIN_A5)) | ((gpio::otyper::Oty::PushPull as u32) << PIN_A5)
        });
        (*gpioa).ospeedr.modify(|v| {
            (v & !((0b11 as u32) << (PIN_A5 * 2))) |
                ((gpio::ospeedr::Ospeedr::High as u32) << (PIN_A5 * 2))
        });
        (*gpioa).pupdr.modify(|v| {
            (v & !((0b11 as u32) << (PIN_A5 * 2))) |
                (gpio::pupdr::Pupdr::NoPuPd as u32) << (PIN_A5 * 2)
        });
    }
}

struct GpioSetup {
    mode: gpio::moder::Modery,
    speed: gpio::ospeedr::Ospeedr,
    otype: gpio::otyper::Oty,
    pupd: gpio::pupdr::Pupdr,
    af: gpio::afr::Afry,
}

fn gpio_setup(gpio_base: u32, pin: u32, set: &GpioSetup) {
    let gpio = gpio_base as *const gpio::RegisterMap;

    unsafe {
        (*gpio).moder.modify(|v| {
            (v & !((0b11 as u32) << (pin * 2))) | ((set.mode as u32) << (pin * 2))
        });
        (*gpio).ospeedr.modify(|v| {
            (v & !((0b11 as u32) << (pin * 2))) | ((set.speed as u32) << (pin * 2))
        });
        (*gpio).pupdr.modify(|v| {
            (v & !((0b11 as u32) << (pin * 2))) | (set.pupd as u32) << (pin * 2)
        });
        (*gpio).otyper.modify(|v| {
            (v & !((0b1 as u32) << pin)) | ((set.otype as u32) << pin)
        });
    }

    if pin < 8 {
        unsafe {
            (*gpio).afr[0].modify(|v| {
                (v & !((0b1111 as u32) << (pin * 4))) | ((set.af as u32) << (pin * 4))
            })
        }
    } else {
        unsafe {
            (*gpio).afr[1].modify(|v| {
                (v & !((0b1111 as u32) << ((pin - 8) * 4))) | ((set.af as u32) << ((pin - 8) * 4))
            })
        }
    }
}

fn spi_init() {
    let rcc = rcc::RCC_BASE as *const rcc::RegisterMap;
    let spi = spi::SPI3_BASE as *const spi::RegisterMap;

    unsafe {
        (*rcc).apb1enr.modify(
            |v| v | rcc::apb1enr::Spi3en::Enable as u32,
        );
    }

    // SPI3 GPIO Configuration
    //  PA4  ------> SPI3_NSS
    //  PC10 ------> SPI3_SCK
    //  PC11 ------> SPI3_MISO
    //  PC12 ------> SPI3_MOSI
    let set = GpioSetup {
        mode: gpio::moder::Modery::Alternate,
        speed: gpio::ospeedr::Ospeedr::High,
        otype: gpio::otyper::Oty::PushPull,
        pupd: gpio::pupdr::Pupdr::NoPuPd,
        af: gpio::afr::Afry::AF6,
    };
    gpio_setup(gpio::GPIOA_BASE, 4, &set);
    gpio_setup(gpio::GPIOC_BASE, 10, &set);
    gpio_setup(gpio::GPIOC_BASE, 11, &set);
    gpio_setup(gpio::GPIOC_BASE, 12, &set);

    /* SPI3 parameter configuration*/
    unsafe {
        (*spi).cr1.modify(|v| {
            (v & !(spi::cr1::Spe::Enable as u32)) |
                (spi::cr1::Rxonly::FullDuplex as u32 | spi::cr1::Mstr::Master as u32 |
                     spi::cr1::Ssi::Enable as u32 | spi::cr1::Dff::Df8bit as u32 |
                     spi::cr1::Cpol::Positive as u32 |
                     spi::cr1::Cpha::Raising as u32 | spi::cr1::Br::DIV32 as u32 |
                     spi::cr1::Lsbfirst::MsbFirst as u32 |
                     spi::cr1::Crcen::Disable as u32)
        });
        (*spi).cr2.modify(|v| v | spi::cr2::Ssoe::Enable as u32);
        // clear I2S mode and activate SPI mode
        (*spi).i2scfgr.modify(|v| {
            v & !(spi::i2scfgr::I2smod::I2sMode as u32)
        });
        // set SPI Motorola mode
        (*spi).cr2.modify(
            |v| v & !(spi::cr2::Frf::SpiTiMode as u32),
        );
    }
}

fn init() {
    ll_init();
    system_clock_config();
    gpio_init();
    spi_init();
}

fn spi_xfer(tx: &u8, rx: &mut u8) {
    let spi = spi::SPI3_BASE as *const spi::RegisterMap;
    // spi enable
    unsafe { (*spi).cr1.modify(|v| v | (spi::cr1::Spe::Enable as u32)) }

    while (unsafe { (*spi).sr.read() } & (spi::sr::Rxne::NotEmpty as u32)) ==
        spi::sr::Rxne::NotEmpty as u32
    {
        let _ = unsafe { (*spi).dr.read() };
    }

    unsafe { (*spi).dr.write(*tx as u32) }

    while (unsafe { (*spi).sr.read() } & (spi::sr::Bsy::Busy as u32)) == spi::sr::Bsy::Busy as u32 {
    }

    while (unsafe { (*spi).sr.read() } & (spi::sr::Txe::Empty as u32)) ==
        spi::sr::Txe::NotEmpty as u32
    {}

    while (unsafe { (*spi).sr.read() } & (spi::sr::Rxne::NotEmpty as u32)) !=
        spi::sr::Rxne::NotEmpty as u32
    {}

    *rx = unsafe { (*spi).dr.read() } as u8;

    // spi disable
    unsafe { (*spi).cr1.modify(|v| v & !(spi::cr1::Spe::Enable as u32)) };
}

fn main() {
    let gpioa = gpio::GPIOA_BASE as *const gpio::RegisterMap;
    const PIN_A5_ODR: u32 = 1 << 5;

    init();

    let mut tx: u8 = 0;
    let mut rx: u8 = 0;

    loop {
        spi_xfer(&tx, &mut rx);
        if tx == rx {
            unsafe { (*gpioa).odr.modify(|v| v | PIN_A5_ODR) }
        } else {
            unsafe { (*gpioa).odr.modify(|v| v & !PIN_A5_ODR) }
        }
        stm32f401re::delay(10000);
        tx = ((tx as u16 + 1) & 0xFF) as u8;
    }
}
