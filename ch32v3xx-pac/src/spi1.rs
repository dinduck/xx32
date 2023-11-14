#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub ctlr1: CTLR1,
    #[doc = "0x04 - control register 2"]
    pub ctlr2: CTLR2,
    #[doc = "0x08 - status register"]
    pub statr: STATR,
    #[doc = "0x0c - data register"]
    pub datar: DATAR,
    #[doc = "0x10 - CRCR polynomial register"]
    pub crcr: CRCR,
    #[doc = "0x14 - RX CRC register"]
    pub rcrcr: RCRCR,
    #[doc = "0x18 - TX CRC register"]
    pub tcrcr: TCRCR,
    #[doc = "0x1c - SPI_I2S configure register"]
    pub spi_i2s_cfgr: SPI_I2S_CFGR,
    _reserved8: [u8; 0x04],
    #[doc = "0x24 - high speed control register"]
    pub hscr: HSCR,
}
#[doc = "CTLR1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlr1`]
module"]
pub type CTLR1 = crate::Reg<ctlr1::CTLR1_SPEC>;
#[doc = "control register 1"]
pub mod ctlr1;
#[doc = "CTLR2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlr2`]
module"]
pub type CTLR2 = crate::Reg<ctlr2::CTLR2_SPEC>;
#[doc = "control register 2"]
pub mod ctlr2;
#[doc = "STATR (rw) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`statr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statr`]
module"]
pub type STATR = crate::Reg<statr::STATR_SPEC>;
#[doc = "status register"]
pub mod statr;
#[doc = "DATAR (rw) register accessor: data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar`]
module"]
pub type DATAR = crate::Reg<datar::DATAR_SPEC>;
#[doc = "data register"]
pub mod datar;
#[doc = "CRCR (rw) register accessor: CRCR polynomial register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcr`]
module"]
pub type CRCR = crate::Reg<crcr::CRCR_SPEC>;
#[doc = "CRCR polynomial register"]
pub mod crcr;
#[doc = "RCRCR (r) register accessor: RX CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcrcr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcrcr`]
module"]
pub type RCRCR = crate::Reg<rcrcr::RCRCR_SPEC>;
#[doc = "RX CRC register"]
pub mod rcrcr;
#[doc = "TCRCR (r) register accessor: TX CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcrcr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcrcr`]
module"]
pub type TCRCR = crate::Reg<tcrcr::TCRCR_SPEC>;
#[doc = "TX CRC register"]
pub mod tcrcr;
#[doc = "SPI_I2S_CFGR (rw) register accessor: SPI_I2S configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_i2s_cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_i2s_cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_i2s_cfgr`]
module"]
pub type SPI_I2S_CFGR = crate::Reg<spi_i2s_cfgr::SPI_I2S_CFGR_SPEC>;
#[doc = "SPI_I2S configure register"]
pub mod spi_i2s_cfgr;
#[doc = "HSCR (rw) register accessor: high speed control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hscr`]
module"]
pub type HSCR = crate::Reg<hscr::HSCR_SPEC>;
#[doc = "high speed control register"]
pub mod hscr;
