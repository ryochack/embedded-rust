use volatile_register::RW;

pub const RCC_BASE: u32 = 0x4002_3800;

pub const RCC_AHB1ENR_GPIOAEN: u32 = 0x0000_0001;

#[repr(C)]
pub struct RegisterMap {
    pub cr: RW<u32>,
    pub pllcfgr: RW<u32>,
    pub cfgr: RW<u32>,
    pub cir: RW<u32>,
    pub ahb1rstr: RW<u32>,
    pub ahb2rstr: RW<u32>,
    reserved0: [u32; 2],
    pub apb1rstr: RW<u32>,
    pub apb2rstr: RW<u32>,
    reserved1: [u32; 2],
    pub ahb1enr: RW<u32>,
    pub ahb2enr: RW<u32>,
    reserved2: [u32; 2],
    pub apb1enr: RW<u32>,
    pub apb2enr: RW<u32>,
    reserved3: [u32; 2],
    pub ahb1lpenr: RW<u32>,
    pub ahb2lpenr: RW<u32>,
    reserved4: [u32; 2],
    pub apb1lpenr: RW<u32>,
    pub apb2lpenr: RW<u32>,
    reserved5: [u32; 2],
    pub bdcr: RW<u32>,
    pub csr: RW<u32>,
    reserved6: [u32; 2],
    pub sscgr: RW<u32>,
    pub plli2scfgr: RW<u32>,
    reserved7: u32,
    pub dckcfgr: RW<u32>,
}
