use volatile_register::RW;

pub const EXTI_BASE: u32 = 0x4001_3C00;

#[repr(C)]
pub struct RegisterMap {
    pub imr: RW<u32>,
    pub emr: RW<u32>,
    pub rtsr: RW<u32>,
    pub ftsr: RW<u32>,
    pub swier: RW<u32>,
    pub pr: RW<u32>,
}

pub mod imr {
    /// Interrupt mask on line x
    pub enum Mrx {
        Mask = 0b0,
        NotMask = 0b1,
    }
}

pub mod emr {
    /// Event mask on line x
    pub enum Mrx {
        Mask = 0b0,
        NotMask = 0b1,
    }
}

pub mod rtsr {
    /// Rising trigger event configuration bit of line x
    pub enum Trx {
        Disable = 0b0, // Rising trigger disabled for input line
        Enable = 0b1, // Rising trigger enabled for input line
    }
}

pub mod ftsr {
    /// Falling trigger event configuration bit of line x
    pub enum Trx {
        Disable = 0b0, // Falling trigger disabled for input line
        Enable = 0b1, // Falling trigger enabled for input line
    }
}

pub mod swier {
    /// Software interrupt on line x
    pub enum Swierx {
        NotAllow = 0b0,
        Allow = 0b1,
    }
}

pub mod pr {
    /// Pending bit
    pub enum Prx {
        NotRequested = 0b0,
        Requested = 0b1,
    }
}
