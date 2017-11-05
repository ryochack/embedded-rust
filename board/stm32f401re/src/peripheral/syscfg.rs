use volatile_register::RW;

pub const SYSCFG_BASE: u32 = 0x4001_3800;

#[repr(C)]
pub struct RegisterMap {
    pub memrmp: RW<u32>,
    pub pmc: RW<u32>,
    pub exticr1: RW<u32>,
    pub exticr2: RW<u32>,
    pub exticr3: RW<u32>,
    pub exticr4: RW<u32>,
    pub cmpcr: RW<u32>,
}

pub mod memrmp {
    /// Memory mapping selection
    pub enum MemMode {
        MainFlashMemory = 0b00 << 0, // Main Flash memory mapped at 0x0000_0000
        SystemFlashMemory = 0b01 << 0, // System Flash memory mapped at 0x0000_0000
        EmbeddedSram = 0b11 << 0, // Embedded SRAM mapped at 0x0000_0000
    }
}

pub mod pmc {
    pub enum Adc1Dc2 {
        NoEffect = 0x0 << 16,
        Effect = 0b1 << 16,
    }
}

pub mod exticrx {
    /// EXTI x configuration
    /// These bits are written by software to select the source input for the EXTIx external interrupt.
    pub enum Extix {
        PAxPin = 0b0000,
        PBxPin = 0b0001,
        PCxPin = 0b0010,
        PDxPin = 0b0011,
        PExPin = 0b0100,
        PHxPin = 0b0111,
    }
}

pub mod cmpcr {
    /// Compensation cell ready flag
    pub enum Ready {
        NotReady = 0b0 << 8,
        Ready = 0b1 << 8,
    }
    /// Compensation cell power-down
    pub enum CmpPd {
        PowerDownMode = 0b0 << 0,
        Enabled = 0b1 << 0,
    }
}
