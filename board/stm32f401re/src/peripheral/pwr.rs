use volatile_register::RW;

pub const PWR_BASE: u32 = 0x4000_7000;

#[repr(C)]
pub struct RegisterMap {
    pub cr: RW<u32>,
    pub csr: RW<u32>,
}

pub mod cr {
    /// Regulator voltage scalling output selection
    pub const VOS_MASK: u32 = 0b11 << 14;
    pub enum Vos {
        Scale3Mode = 0b01 << 14,
        Scale2Mode = 0b10 << 14,
    }
    pub enum Adcdc1 {
        NoEffect = 0b0 << 13,
        Effect = 0b1 << 13,
    }
    /// Main regulator Low Voltage in Deep Sleep
    pub const MRLVDS: u32 = 0b1 << 11;
    /// Low-power regulator Low Voltage in Deep Sleep
    pub const LPLVDS: u32 = 0b1 << 10;
    /// Flash power-down in Stop mode
    pub const FPDS: u32 = 0b1 << 9;
    /// Disable backup domain write protection
    pub const DBP: u32 = 0b1 << 8;
    /// PVD level selection
    #[allow(non_camel_case_types)]
    pub enum Pls {
        _2_2V = 0b000 << 5,
        _2_3V = 0b001 << 5,
        _2_4V = 0b010 << 5,
        _2_5V = 0b011 << 5,
        _2_6V = 0b100 << 5,
        _2_7V = 0b101 << 5,
        _2_8V = 0b110 << 5,
        _2_9V = 0b111 << 5,
    }
    /// Power voltage detector enable
    pub enum Pvde {
        Disable = 0b0 << 4,
        Enable = 0b1 << 4,
    }
    /// Clear standby flag
    pub enum Csbf {
        NoEffect = 0b0 << 3,
        Effect = 0b1 << 3,
    }
    /// Clear wakeup flag
    pub enum Cwuf {
        NoEffect = 0b0 << 2,
        Effect = 0b1 << 2,
    }
    /// Power-down deepsleep
    pub enum Pdds {
        StopMode = 0b0 << 1, // Enter Stop mode when the CPU enters deepsleep.
        StandbyMode = 0b1 << 1, // Enter Standby mode when the CPU enters deepsleep.
    }
    /// Low-power deepsleep
    pub const LPDS: u32 = 0b1 << 0;
}

pub mod csr {
    /// Regulator voltage scalling output selection ready bit
    pub enum Vosrdy {
        NotReady = 0b0 << 14,
        Ready = 0b1 << 14,
    }
    /// Backup regulator enable
    pub enum Bre {
        Disable = 0b0 << 9,
        Enable = 0b1 << 9,
    }
    /// Enable WKUP pin
    pub enum Ewup {
        Disable = 0b0 << 8, // WKUP pin is used for GPIO.
        Enable = 0b1 << 8, // WKUP pin is used for wakeup from Standby mode.
    }
    /// Backup regulator ready
    pub const BRR: u32 = 0b1 << 3;
    /// PVD output
    pub const PVD: u32 = 0b1 << 2;
    /// Standby flag
    pub const SBF: u32 = 0b1 << 1;
    /// Wakeup flag
    pub const WUF: u32 = 0b1 << 0;
}
