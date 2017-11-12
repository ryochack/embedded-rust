use volatile_register::{RO, RW};

pub const SPI2_BASE: u32 = 0x4000_3800;
pub const SPI3_BASE: u32 = 0x4000_3C00;
pub const SPI1_BASE: u32 = 0x4001_3000;
pub const SPI4_BASE: u32 = 0x4001_3400;

#[repr(C)]
pub struct RegisterMap {
    pub cr1: RW<u32>,
    pub cr2: RW<u32>,
    pub sr: RW<u32>,
    pub dr: RW<u32>,
    pub crcpr: RW<u32>,
    pub rxcrcr: RO<u32>,
    pub txcrcr: RO<u32>,
    pub i2scfgr: RW<u32>,
    pub i2spr: RW<u32>,
}

pub mod cr1 {
    /// Bidirectional data mode enable
    pub enum Bidimode {
        Undir2Line = 0b0 << 15,
        Bidir1Line = 0b1 << 15,
    }
    /// Output enable in bidirectional mode
    pub enum Bidioe {
        Disable = 0b0 << 14,
        Enable = 0b1 << 14,
    }
    /// Hardware CRC calculation enable
    pub enum Crcen {
        Disable = 0b0 << 13,
        Enable = 0b1 << 13,
    }
    /// CRC transfer next
    pub enum Crcnext {
        NoCrcPhase = 0b0 << 12,
        CrcPhase = 0b1 << 12,
    }
    /// Data frame format
    pub enum Dff {
        Df8bit = 0b0 << 11,
        Df16bit = 0b1 << 11,
    }
    /// Receive only
    pub enum Rxonly {
        FullDuplex = 0b0 << 10,
        OutputDisable = 0b1 << 10,
    }
    /// Software slave management
    pub enum Ssm {
        Disable = 0b0 << 9,
        Enable = 0b1 << 9,
    }
    /// Internal slave select
    pub enum Ssi {
        Disable = 0b0 << 8,
        Enable = 0b1 << 8,
    }
    /// Frame format
    pub enum Lsbfirst {
        MsbFirst = 0b0 << 7,
        LsbFirst = 0b1 << 7,
    }
    /// SPI enable
    pub enum Spe {
        Disable = 0b0 << 6,
        Enable = 0b1 << 6,
    }
    /// Baud rate control (f_pclk/Br)
    pub enum Br {
        DIV2 = 0b000 << 3,
        DIV4 = 0b001 << 3,
        DIV8 = 0b010 << 3,
        DIV16 = 0b011 << 3,
        DIV32 = 0b100 << 3,
        DIV64 = 0b101 << 3,
        DIV128 = 0b110 << 3,
        DIV256 = 0b111 << 3,
    }
    /// Master selection
    pub enum Mstr {
        Slave = 0b0 << 2,
        Master = 0b1 << 2,
    }
    /// Clock polarity
    pub enum Cpol {
        Positive = 0b0 << 1, // CK to 0 when idle
        Negative = 0b1 << 1, // CK to 1 when idle
    }
    /// Clock phase
    pub enum Cpha {
        Raising = 0b0, // The first clock transition is the first data capture edge
        Falling = 0b1, // The second clock transition is the first data capture edge
    }
}

pub mod cr2 {
    /// Tx buffer empty interrupt enable
    pub enum Txeie {
        Disable = 0b0 << 7,
        Enable = 0b1 << 7,
    }
    /// Rx buffer not empty interrupt enable
    pub enum Rxneie {
        Disable = 0b0 << 6,
        Enable = 0b1 << 6,
    }
    /// Error interrutp enable
    pub enum Errie {
        Disable = 0b0 << 5,
        Enable = 0b1 << 5,
    }
    /// Frame format
    pub enum Frf {
        SpiMotorolaMode = 0b0 << 4,
        SpiTiMode = 0b1 << 4,
    }
    /// SS output enable
    pub enum Ssoe {
        Disable = 0b0 << 2,
        Enable = 0b1 << 2,
    }
    /// Tx buffer DMA enable
    pub enum Txdmaen {
        Disable = 0b0 << 1,
        Enable = 0b1 << 1,
    }
    /// Rx buffer DMA enable
    pub enum Rxdmaen {
        Disable = 0b0 << 0,
        Enable = 0b1 << 0,
    }
}

pub mod sr {
    /// Frame format error
    pub enum Fre {
        NoError = 0b0 << 8,
        Error = 0b1 << 8,
    }
    /// Busy flag
    pub enum Bsy {
        NotBusy = 0b0 << 7,
        Busy = 0b1 << 7,
    }
    /// Overrun flag
    pub enum Ovr {
        NoOverrun = 0b0 << 6,
        Overrun = 0b1 << 6,
    }
    /// Mode fault
    pub enum Modf {
        NoModeFault = 0b0 << 5,
        ModeFault = 0b1 << 5,
    }
    /// CRC error flag
    pub enum Crcerr {
        NoError = 0b0 << 4, // CRC value received matches the SPI_RXCRCR value
        Error = 0b1 << 4, // CRC value received does not match the SPI_RXCRCR value
    }
    /// Underrun flag
    pub enum Udr {
        NoUnderrun = 0b0 << 3,
        Underrun = 0b1 << 3,
    }
    /// Channel side
    pub enum Chside {
        Left = 0b0 << 2,
        Right = 0b1 << 2,
    }
    /// Transmit buffer empty
    pub enum Txe {
        NotEmpty = 0b0 << 1,
        Empty = 0b1 << 1,
    }
    /// Receive buffer not empty
    pub enum Rxne {
        Empty = 0b0 << 0,
        NotEmpty = 0b1 << 0,
    }
}

pub mod i2scfgr {
    /// I2S mode selection
    pub enum I2smod {
        SpiMode = 0b0 << 11,
        I2sMode = 0b1 << 11,
    }
    /// I2S Peripheral Enable
    pub enum I2se {
        Disable = 0b0 << 10,
        Enable = 0b1 << 10,
    }
    /// I2S configuration mode
    pub enum I2scfg {
        SlaveTransmit = 0b00 << 8,
        SlaveReceive = 0b01 << 8,
        MasterTransmit = 0b10 << 8,
        MasterReceive = 0b11 << 8,
    }
    /// PCM frame synchronization
    pub enum Pcmsync {
        Short = 0b0 << 7, // Short frame synchronization
        Long = 0b1 << 7, // Long frame synchronization
    }
    /// I2S standard selection
    pub enum I2sstd {
        I2sPhilips = 0b00 << 4,
        MsbJustified = 0b01 << 4,
        LsbJustified = 0b10 << 4,
        Pcm = 0b11 << 4,
    }
    /// Steady state clock polarity
    pub enum Ckpol {
        Low = 0b0 << 3, // I2S clock steady state is low level
        High = 0b1 << 3, // I2S clock steady state is high level
    }
    /// Data length to be transferred
    pub enum Datalen {
        _16bit = 0b00 << 1,
        _24bit = 0b01 << 1,
        _32bit = 0b10 << 1,
        NotAllow = 0b11 << 1,
    }
    /// Channel length (number of bits per audio channel)
    pub enum Chlen {
        _16bitWide = 0b0 << 0,
        _32bitWide = 0b1 << 0,
    }
}

pub mod i2spr {
    /// Master clock output enable
    pub enum Mckoe {
        Disable = 0b0 << 9,
        Enable = 0b1 << 9,
    }
    /// Odd factor for the prescaler
    pub enum Odd {
        I2sDivX2 = 0b0 << 8,
        I2sDivX2P1 = 0b1 << 8,
    }
    pub const I2S_DIV_MASK: u32 = 0xFF;
}
