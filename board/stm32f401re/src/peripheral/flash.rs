use volatile_register::{WO, RW};

pub const FLASH_BASE: u32 = 0x4002_3C00;

#[repr(C)]
pub struct RegisterMap {
    pub acr: RW<u32>,
    pub keyr: WO<u32>,
    pub optkeyr: WO<u32>,
    pub sr: RW<u32>,
    pub cr: RW<u32>,
    pub optcr: RW<u32>,
}

pub mod acr {
    /// Data cache reset
    pub enum Dcrst {
        NotReset = 0b0 << 12,
        Reset = 0b1 << 12,
    }
    /// Instruction cache reset
    pub enum Icrst {
        NotReset = 0b0 << 11,
        Reset = 0b1 << 11,
    }
    /// Data cache enable
    pub enum Dcen {
        Disable = 0b0 << 10,
        Enable = 0b1 << 10,
    }
    /// Instruction cache enable
    pub enum Icen {
        Disable = 0b0 << 9,
        Enable = 0b1 << 9,
    }
    /// Prefetch enable
    pub enum Prften {
        Disable = 0b0 << 8,
        Enable = 0b1 << 8,
    }
    /// Latency
    pub const LATENCY_MASK: u32 = 0xF;
}

pub mod sr {
    /// Busy
    pub const BSY: u32 = 0b1 << 16;
    /// Road Protection Error
    pub const RDERR: u32 = 0b1 << 8;
    /// Programming sequence error
    pub const PGSERR: u32 = 0b1 << 7;
    /// Programming parallelism error
    pub const PGPERR: u32 = 0b1 << 6;
    /// Programming alignment error
    pub const PGAERR: u32 = 0b1 << 5;
    /// Write protection error
    pub const WRPERR: u32 = 0b1 << 4;
    /// Operation error
    pub const OPERR: u32 = 0b1 << 1;
    /// End of operation
    pub const EOP: u32 = 0b1 << 0;
}

pub mod cr {
    /// Lock
    pub const LOCK: u32 = 0b1 << 31;
    /// Error interrupt enable
    pub enum Errie {
        Disable = 0b0 << 25,
        Enable = 0b1 << 25,
    }
    /// End of operation interrupt enable
    pub enum Eopie {
        Disable = 0b0 << 24,
        Enable = 0b1 << 24,
    }
    /// Start
    pub const START: u32 = 0b1 << 16;
    /// Program size
    pub enum Psize {
        X8 = 0b00 << 8,
        X16 = 0b01 << 8,
        X32 = 0b10 << 8,
        X64 = 0b11 << 8,
    }
    /// Sector number
    pub const SNB_MASK: u32 = 0xF << 3;
    /// Mass Erase
    pub const MER: u32 = 0b1 << 2;
    /// Sector Erase
    pub const SER: u32 = 0b1 << 1;
    /// Programming
    pub const PG: u32 = 0b1 << 0;
}

pub mod optcr {
    /// Selection of Protection Mode of nWPRi bits
    pub enum SPRMOD {
        Disable = 0b0 << 31,
        Enable = 0b1 << 31,
    }
    /// Not write protect
    pub const N_WRP_MASK: u32 = 0xFF << 16;
    /// Read protect
    pub enum Rdp {
        Level0 = 0xAA << 8,
        Level2 = 0xCC << 8,
        Level1 = 0xBB << 8, // others: level2
    }
    /// User option bytes
    pub const USER_N_RST_STDBY: u32 = 0b1 << 7;
    pub const USER_N_RST_STOP: u32 = 0b1 << 6;
    pub const USER_WDG_SW: u32 = 0b1 << 5;
    /// BOR reset level
    pub enum Bor {
        Level3 = 0b00 << 2,
        Level2 = 0b01 << 2,
        Level1 = 0b10 << 2,
        Off = 0b11 << 2,
    }
    /// Option start
    pub const OPTSTART: u32 = 0b1 << 1;
    /// Option lock
    pub const OPTLOCK: u32 = 0b1 << 0;
}
