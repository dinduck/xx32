#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register (DMA_INTFR)"]
    pub intfr: INTFR,
    #[doc = "0x04 - DMA interrupt flag clear register (DMA_INTFCR)"]
    pub intfcr: INTFCR,
    #[doc = "0x08 - DMA channel configuration register (DMA_CFGR)"]
    pub cfgr1: CFGR1,
    #[doc = "0x0c - DMA channel 1 number of data register"]
    pub cntr1: CNTR1,
    #[doc = "0x10 - DMA channel 1 peripheral address register"]
    pub paddr1: PADDR1,
    #[doc = "0x14 - DMA channel 1 memory address register"]
    pub maddr1: MADDR1,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - DMA channel configuration register (DMA_CFGR)"]
    pub cfgr2: CFGR2,
    #[doc = "0x20 - DMA channel 2 number of data register"]
    pub cntr2: CNTR2,
    #[doc = "0x24 - DMA channel 2 peripheral address register"]
    pub paddr2: PADDR2,
    #[doc = "0x28 - DMA channel 2 memory address register"]
    pub maddr2: MADDR2,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - DMA channel configuration register (DMA_CFGR)"]
    pub cfgr3: CFGR3,
    #[doc = "0x34 - DMA channel 3 number of data register"]
    pub cntr3: CNTR3,
    #[doc = "0x38 - DMA channel 3 peripheral address register"]
    pub paddr3: PADDR3,
    #[doc = "0x3c - DMA channel 3 memory address register"]
    pub maddr3: MADDR3,
    _reserved14: [u8; 0x04],
    #[doc = "0x44 - DMA channel configuration register (DMA_CFGR)"]
    pub cfgr4: CFGR4,
    #[doc = "0x48 - DMA channel 4 number of data register"]
    pub cntr4: CNTR4,
    #[doc = "0x4c - DMA channel 4 peripheral address register"]
    pub paddr4: PADDR4,
    #[doc = "0x50 - DMA channel 4 memory address register"]
    pub maddr4: MADDR4,
    _reserved18: [u8; 0x04],
    #[doc = "0x58 - DMA channel configuration register (DMA_CFGR)"]
    pub cfgr5: CFGR5,
    #[doc = "0x5c - DMA channel 5 number of data register"]
    pub cntr5: CNTR5,
    #[doc = "0x60 - DMA channel 5 peripheral address register"]
    pub paddr5: PADDR5,
    #[doc = "0x64 - DMA channel 5 memory address register"]
    pub maddr5: MADDR5,
    _reserved22: [u8; 0x04],
    #[doc = "0x6c - DMA channel configuration register (DMA_CFGR)"]
    pub cfgr6: CFGR6,
    #[doc = "0x70 - DMA channel 6 number of data register"]
    pub cntr6: CNTR6,
    #[doc = "0x74 - DMA channel 6 peripheral address register"]
    pub paddr6: PADDR6,
    #[doc = "0x78 - DMA channel 6 memory address register"]
    pub maddr6: MADDR6,
    _reserved26: [u8; 0x04],
    #[doc = "0x80 - DMA channel configuration register (DMA_CFGR)"]
    pub cfgr7: CFGR7,
    #[doc = "0x84 - DMA channel 7 number of data register"]
    pub cntr7: CNTR7,
    #[doc = "0x88 - DMA channel 7 peripheral address register"]
    pub paddr7: PADDR7,
    #[doc = "0x8c - DMA channel 7 memory address register"]
    pub maddr7: MADDR7,
    #[doc = "0x90 - DMA channel configuration register (DMA_CFGR) used in ch32v30x_D8/D8C"]
    pub cfgr8: CFGR8,
    #[doc = "0x94 - DMA channel 8 number of data register used in ch32v30x_D8/D8C"]
    pub cntr8: CNTR8,
    #[doc = "0x98 - DMA channel 8 peripheral address register used in ch32v30x_D8/D8C"]
    pub paddr8: PADDR8,
    #[doc = "0x9c - DMA channel 8 memory address register used in ch32v30x_D8/D8C"]
    pub maddr8: MADDR8,
    #[doc = "0xa0 - DMA channel configuration register (DMA_CFGR) used in ch32v30x_D8/D8C"]
    pub cfgr9: CFGR9,
    #[doc = "0xa4 - DMA channel 9 number of data register used in ch32v30x_D8/D8C"]
    pub cntr9: CNTR9,
    #[doc = "0xa8 - DMA channel 7 peripheral address register used in ch32v30x_D8/D8C"]
    pub paddr9: PADDR9,
    #[doc = "0xac - DMA channel 9 memory address register used in ch32v30x_D8/D8C"]
    pub maddr9: MADDR9,
    #[doc = "0xb0 - DMA channel configuration register (DMA_CFGR) used in ch32v30x_D8/D8C"]
    pub cfgr10: CFGR10,
    #[doc = "0xb4 - DMA channel 10 number of data register used in ch32v30x_D8/D8C"]
    pub cntr10: CNTR10,
    #[doc = "0xb8 - DMA channel 10 peripheral address register used in ch32v30x_D8/D8C"]
    pub paddr10: PADDR10,
    #[doc = "0xbc - DMA channel 10 memory address register used in ch32v30x_D8/D8C"]
    pub maddr10: MADDR10,
    #[doc = "0xc0 - DMA channel configuration register (DMA_CFGR) used in ch32v30x_D8/D8C"]
    pub cfgr11: CFGR11,
    #[doc = "0xc4 - DMA channel 11 number of data register used in ch32v30x_D8/D8C"]
    pub cntr11: CNTR11,
    #[doc = "0xc8 - DMA channel 11 peripheral address register used in ch32v30x_D8/D8C"]
    pub paddr11: PADDR11,
    #[doc = "0xcc - DMA channel 11 memory address register used in ch32v30x_D8/D8C"]
    pub maddr11: MADDR11,
    #[doc = "0xd0 - DMA2 EXTEN interrupt status register (DMA_INTFR)used in ch32v30x_D8/D8C"]
    pub exten_intfr: EXTEN_INTFR,
    #[doc = "0xd4 - DMA2 EXTEN interrupt flag clear register (DMA_INTFCR)used in ch32v30x_D8/D8C"]
    pub exten_intfcr: EXTEN_INTFCR,
}
#[doc = "INTFR (r) register accessor: DMA interrupt status register (DMA_INTFR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intfr`]
module"]
pub type INTFR = crate::Reg<intfr::INTFR_SPEC>;
#[doc = "DMA interrupt status register (DMA_INTFR)"]
pub mod intfr;
#[doc = "INTFCR (w) register accessor: DMA interrupt flag clear register (DMA_INTFCR)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intfcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intfcr`]
module"]
pub type INTFCR = crate::Reg<intfcr::INTFCR_SPEC>;
#[doc = "DMA interrupt flag clear register (DMA_INTFCR)"]
pub mod intfcr;
#[doc = "CFGR1 (rw) register accessor: DMA channel configuration register (DMA_CFGR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr1`]
module"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
#[doc = "DMA channel configuration register (DMA_CFGR)"]
pub mod cfgr1;
#[doc = "CNTR1 (rw) register accessor: DMA channel 1 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr1`]
module"]
pub type CNTR1 = crate::Reg<cntr1::CNTR1_SPEC>;
#[doc = "DMA channel 1 number of data register"]
pub mod cntr1;
#[doc = "PADDR1 (rw) register accessor: DMA channel 1 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`paddr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`paddr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paddr1`]
module"]
pub type PADDR1 = crate::Reg<paddr1::PADDR1_SPEC>;
#[doc = "DMA channel 1 peripheral address register"]
pub mod paddr1;
#[doc = "MADDR1 (rw) register accessor: DMA channel 1 memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maddr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maddr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maddr1`]
module"]
pub type MADDR1 = crate::Reg<maddr1::MADDR1_SPEC>;
#[doc = "DMA channel 1 memory address register"]
pub mod maddr1;
#[doc = "CFGR2 (rw) register accessor: DMA channel configuration register (DMA_CFGR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`]
module"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "DMA channel configuration register (DMA_CFGR)"]
pub mod cfgr2;
#[doc = "CNTR2 (rw) register accessor: DMA channel 2 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr2`]
module"]
pub type CNTR2 = crate::Reg<cntr2::CNTR2_SPEC>;
#[doc = "DMA channel 2 number of data register"]
pub mod cntr2;
#[doc = "PADDR2 (rw) register accessor: DMA channel 2 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`paddr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`paddr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paddr2`]
module"]
pub type PADDR2 = crate::Reg<paddr2::PADDR2_SPEC>;
#[doc = "DMA channel 2 peripheral address register"]
pub mod paddr2;
#[doc = "MADDR2 (rw) register accessor: DMA channel 2 memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maddr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maddr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maddr2`]
module"]
pub type MADDR2 = crate::Reg<maddr2::MADDR2_SPEC>;
#[doc = "DMA channel 2 memory address register"]
pub mod maddr2;
#[doc = "CFGR3 (rw) register accessor: DMA channel configuration register (DMA_CFGR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr3`]
module"]
pub type CFGR3 = crate::Reg<cfgr3::CFGR3_SPEC>;
#[doc = "DMA channel configuration register (DMA_CFGR)"]
pub mod cfgr3;
#[doc = "CNTR3 (rw) register accessor: DMA channel 3 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr3`]
module"]
pub type CNTR3 = crate::Reg<cntr3::CNTR3_SPEC>;
#[doc = "DMA channel 3 number of data register"]
pub mod cntr3;
#[doc = "PADDR3 (rw) register accessor: DMA channel 3 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`paddr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`paddr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paddr3`]
module"]
pub type PADDR3 = crate::Reg<paddr3::PADDR3_SPEC>;
#[doc = "DMA channel 3 peripheral address register"]
pub mod paddr3;
#[doc = "MADDR3 (rw) register accessor: DMA channel 3 memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maddr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maddr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maddr3`]
module"]
pub type MADDR3 = crate::Reg<maddr3::MADDR3_SPEC>;
#[doc = "DMA channel 3 memory address register"]
pub mod maddr3;
#[doc = "CFGR4 (rw) register accessor: DMA channel configuration register (DMA_CFGR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr4`]
module"]
pub type CFGR4 = crate::Reg<cfgr4::CFGR4_SPEC>;
#[doc = "DMA channel configuration register (DMA_CFGR)"]
pub mod cfgr4;
#[doc = "CNTR4 (rw) register accessor: DMA channel 4 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr4`]
module"]
pub type CNTR4 = crate::Reg<cntr4::CNTR4_SPEC>;
#[doc = "DMA channel 4 number of data register"]
pub mod cntr4;
#[doc = "PADDR4 (rw) register accessor: DMA channel 4 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`paddr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`paddr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paddr4`]
module"]
pub type PADDR4 = crate::Reg<paddr4::PADDR4_SPEC>;
#[doc = "DMA channel 4 peripheral address register"]
pub mod paddr4;
#[doc = "MADDR4 (rw) register accessor: DMA channel 4 memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maddr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maddr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maddr4`]
module"]
pub type MADDR4 = crate::Reg<maddr4::MADDR4_SPEC>;
#[doc = "DMA channel 4 memory address register"]
pub mod maddr4;
#[doc = "CFGR5 (rw) register accessor: DMA channel configuration register (DMA_CFGR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr5`]
module"]
pub type CFGR5 = crate::Reg<cfgr5::CFGR5_SPEC>;
#[doc = "DMA channel configuration register (DMA_CFGR)"]
pub mod cfgr5;
#[doc = "CNTR5 (rw) register accessor: DMA channel 5 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr5`]
module"]
pub type CNTR5 = crate::Reg<cntr5::CNTR5_SPEC>;
#[doc = "DMA channel 5 number of data register"]
pub mod cntr5;
#[doc = "PADDR5 (rw) register accessor: DMA channel 5 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`paddr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`paddr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paddr5`]
module"]
pub type PADDR5 = crate::Reg<paddr5::PADDR5_SPEC>;
#[doc = "DMA channel 5 peripheral address register"]
pub mod paddr5;
#[doc = "MADDR5 (rw) register accessor: DMA channel 5 memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maddr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maddr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maddr5`]
module"]
pub type MADDR5 = crate::Reg<maddr5::MADDR5_SPEC>;
#[doc = "DMA channel 5 memory address register"]
pub mod maddr5;
#[doc = "CFGR6 (rw) register accessor: DMA channel configuration register (DMA_CFGR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr6`]
module"]
pub type CFGR6 = crate::Reg<cfgr6::CFGR6_SPEC>;
#[doc = "DMA channel configuration register (DMA_CFGR)"]
pub mod cfgr6;
#[doc = "CNTR6 (rw) register accessor: DMA channel 6 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr6`]
module"]
pub type CNTR6 = crate::Reg<cntr6::CNTR6_SPEC>;
#[doc = "DMA channel 6 number of data register"]
pub mod cntr6;
#[doc = "PADDR6 (rw) register accessor: DMA channel 6 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`paddr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`paddr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paddr6`]
module"]
pub type PADDR6 = crate::Reg<paddr6::PADDR6_SPEC>;
#[doc = "DMA channel 6 peripheral address register"]
pub mod paddr6;
#[doc = "MADDR6 (rw) register accessor: DMA channel 6 memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maddr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maddr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maddr6`]
module"]
pub type MADDR6 = crate::Reg<maddr6::MADDR6_SPEC>;
#[doc = "DMA channel 6 memory address register"]
pub mod maddr6;
#[doc = "CFGR7 (rw) register accessor: DMA channel configuration register (DMA_CFGR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr7`]
module"]
pub type CFGR7 = crate::Reg<cfgr7::CFGR7_SPEC>;
#[doc = "DMA channel configuration register (DMA_CFGR)"]
pub mod cfgr7;
#[doc = "CNTR7 (rw) register accessor: DMA channel 7 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr7`]
module"]
pub type CNTR7 = crate::Reg<cntr7::CNTR7_SPEC>;
#[doc = "DMA channel 7 number of data register"]
pub mod cntr7;
#[doc = "PADDR7 (rw) register accessor: DMA channel 7 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`paddr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`paddr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paddr7`]
module"]
pub type PADDR7 = crate::Reg<paddr7::PADDR7_SPEC>;
#[doc = "DMA channel 7 peripheral address register"]
pub mod paddr7;
#[doc = "MADDR7 (rw) register accessor: DMA channel 7 memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maddr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maddr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maddr7`]
module"]
pub type MADDR7 = crate::Reg<maddr7::MADDR7_SPEC>;
#[doc = "DMA channel 7 memory address register"]
pub mod maddr7;
#[doc = "CFGR8 (rw) register accessor: DMA channel configuration register (DMA_CFGR) used in ch32v30x_D8/D8C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr8`]
module"]
pub type CFGR8 = crate::Reg<cfgr8::CFGR8_SPEC>;
#[doc = "DMA channel configuration register (DMA_CFGR) used in ch32v30x_D8/D8C"]
pub mod cfgr8;
#[doc = "CNTR8 (rw) register accessor: DMA channel 8 number of data register used in ch32v30x_D8/D8C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr8`]
module"]
pub type CNTR8 = crate::Reg<cntr8::CNTR8_SPEC>;
#[doc = "DMA channel 8 number of data register used in ch32v30x_D8/D8C"]
pub mod cntr8;
#[doc = "PADDR8 (rw) register accessor: DMA channel 8 peripheral address register used in ch32v30x_D8/D8C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`paddr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`paddr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paddr8`]
module"]
pub type PADDR8 = crate::Reg<paddr8::PADDR8_SPEC>;
#[doc = "DMA channel 8 peripheral address register used in ch32v30x_D8/D8C"]
pub mod paddr8;
#[doc = "MADDR8 (rw) register accessor: DMA channel 8 memory address register used in ch32v30x_D8/D8C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maddr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maddr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maddr8`]
module"]
pub type MADDR8 = crate::Reg<maddr8::MADDR8_SPEC>;
#[doc = "DMA channel 8 memory address register used in ch32v30x_D8/D8C"]
pub mod maddr8;
#[doc = "CFGR9 (rw) register accessor: DMA channel configuration register (DMA_CFGR) used in ch32v30x_D8/D8C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr9`]
module"]
pub type CFGR9 = crate::Reg<cfgr9::CFGR9_SPEC>;
#[doc = "DMA channel configuration register (DMA_CFGR) used in ch32v30x_D8/D8C"]
pub mod cfgr9;
#[doc = "CNTR9 (rw) register accessor: DMA channel 9 number of data register used in ch32v30x_D8/D8C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr9`]
module"]
pub type CNTR9 = crate::Reg<cntr9::CNTR9_SPEC>;
#[doc = "DMA channel 9 number of data register used in ch32v30x_D8/D8C"]
pub mod cntr9;
#[doc = "PADDR9 (rw) register accessor: DMA channel 7 peripheral address register used in ch32v30x_D8/D8C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`paddr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`paddr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paddr9`]
module"]
pub type PADDR9 = crate::Reg<paddr9::PADDR9_SPEC>;
#[doc = "DMA channel 7 peripheral address register used in ch32v30x_D8/D8C"]
pub mod paddr9;
#[doc = "MADDR9 (rw) register accessor: DMA channel 9 memory address register used in ch32v30x_D8/D8C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maddr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maddr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maddr9`]
module"]
pub type MADDR9 = crate::Reg<maddr9::MADDR9_SPEC>;
#[doc = "DMA channel 9 memory address register used in ch32v30x_D8/D8C"]
pub mod maddr9;
#[doc = "CFGR10 (rw) register accessor: DMA channel configuration register (DMA_CFGR) used in ch32v30x_D8/D8C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr10`]
module"]
pub type CFGR10 = crate::Reg<cfgr10::CFGR10_SPEC>;
#[doc = "DMA channel configuration register (DMA_CFGR) used in ch32v30x_D8/D8C"]
pub mod cfgr10;
#[doc = "CNTR10 (rw) register accessor: DMA channel 10 number of data register used in ch32v30x_D8/D8C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr10`]
module"]
pub type CNTR10 = crate::Reg<cntr10::CNTR10_SPEC>;
#[doc = "DMA channel 10 number of data register used in ch32v30x_D8/D8C"]
pub mod cntr10;
#[doc = "PADDR10 (rw) register accessor: DMA channel 10 peripheral address register used in ch32v30x_D8/D8C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`paddr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`paddr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paddr10`]
module"]
pub type PADDR10 = crate::Reg<paddr10::PADDR10_SPEC>;
#[doc = "DMA channel 10 peripheral address register used in ch32v30x_D8/D8C"]
pub mod paddr10;
#[doc = "MADDR10 (rw) register accessor: DMA channel 10 memory address register used in ch32v30x_D8/D8C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maddr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maddr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maddr10`]
module"]
pub type MADDR10 = crate::Reg<maddr10::MADDR10_SPEC>;
#[doc = "DMA channel 10 memory address register used in ch32v30x_D8/D8C"]
pub mod maddr10;
#[doc = "CFGR11 (rw) register accessor: DMA channel configuration register (DMA_CFGR) used in ch32v30x_D8/D8C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr11`]
module"]
pub type CFGR11 = crate::Reg<cfgr11::CFGR11_SPEC>;
#[doc = "DMA channel configuration register (DMA_CFGR) used in ch32v30x_D8/D8C"]
pub mod cfgr11;
#[doc = "CNTR11 (rw) register accessor: DMA channel 11 number of data register used in ch32v30x_D8/D8C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr11`]
module"]
pub type CNTR11 = crate::Reg<cntr11::CNTR11_SPEC>;
#[doc = "DMA channel 11 number of data register used in ch32v30x_D8/D8C"]
pub mod cntr11;
#[doc = "PADDR11 (rw) register accessor: DMA channel 11 peripheral address register used in ch32v30x_D8/D8C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`paddr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`paddr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paddr11`]
module"]
pub type PADDR11 = crate::Reg<paddr11::PADDR11_SPEC>;
#[doc = "DMA channel 11 peripheral address register used in ch32v30x_D8/D8C"]
pub mod paddr11;
#[doc = "MADDR11 (rw) register accessor: DMA channel 11 memory address register used in ch32v30x_D8/D8C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maddr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maddr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maddr11`]
module"]
pub type MADDR11 = crate::Reg<maddr11::MADDR11_SPEC>;
#[doc = "DMA channel 11 memory address register used in ch32v30x_D8/D8C"]
pub mod maddr11;
#[doc = "EXTEN_INTFR (r) register accessor: DMA2 EXTEN interrupt status register (DMA_INTFR)used in ch32v30x_D8/D8C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exten_intfr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exten_intfr`]
module"]
pub type EXTEN_INTFR = crate::Reg<exten_intfr::EXTEN_INTFR_SPEC>;
#[doc = "DMA2 EXTEN interrupt status register (DMA_INTFR)used in ch32v30x_D8/D8C"]
pub mod exten_intfr;
#[doc = "EXTEN_INTFCR (rw) register accessor: DMA2 EXTEN interrupt flag clear register (DMA_INTFCR)used in ch32v30x_D8/D8C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exten_intfcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exten_intfcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exten_intfcr`]
module"]
pub type EXTEN_INTFCR = crate::Reg<exten_intfcr::EXTEN_INTFCR_SPEC>;
#[doc = "DMA2 EXTEN interrupt flag clear register (DMA_INTFCR)used in ch32v30x_D8/D8C"]
pub mod exten_intfcr;
