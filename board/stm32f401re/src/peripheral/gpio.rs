use volatile_register::{RO, WO, RW};

pub const GPIOA_BASE: u32 = 0x4002_0000;
pub const GPIOB_BASE: u32 = 0x4002_0400;
pub const GPIOC_BASE: u32 = 0x4002_0800;
pub const GPIOD_BASE: u32 = 0x4002_0C00;
pub const GPIOE_BASE: u32 = 0x4002_1000;
pub const GPIOH_BASE: u32 = 0x4002_1C00;

#[allow(dead_code)]
pub enum Moder {
    Input = 0b00,
    Output = 0b01,
    Alternate = 0b10,
    Analog = 0b11,
}

#[allow(dead_code)]
pub enum Typer {
    PushPull = 0b0,
    OpenDrain = 0b1,
}

#[allow(dead_code)]
pub enum Ospeedr {
    Low = 0b00,
    Medium = 0b01,
    Fast = 0b10,
    High = 0b11,
}

#[allow(dead_code)]
pub enum Pupdr {
    NoPuPd = 0b00,
    PullUp = 0b01,
    PuuDown = 0b10,
}

#[repr(C)]
pub struct RegisterMap {
    pub moder: RW<u32>,
    pub otyper: RW<u32>,
    pub ospeedr: RW<u32>,
    pub pupdr: RW<u32>,
    pub idr: RO<u32>,
    pub odr: RW<u32>,
    pub bsrr: WO<u32>,
    pub lckr: RW<u32>,
    pub afrl: RW<u32>,
    pub afrh: RW<u32>,
}
