#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAN Master control register"]
    pub ctlr: CTLR,
    #[doc = "0x04 - CAN master status register"]
    pub statr: STATR,
    #[doc = "0x08 - CAN transmit status register"]
    pub tstatr: TSTATR,
    #[doc = "0x0c - CAN receive FIFO 0 register"]
    pub rfifo0: RFIFO0,
    #[doc = "0x10 - CAN receive FIFO 1 register"]
    pub rfifo1: RFIFO1,
    #[doc = "0x14 - CAN interrupt enable register"]
    pub intenr: INTENR,
    #[doc = "0x18 - CAN error status register"]
    pub errsr: ERRSR,
    #[doc = "0x1c - CAN bit timing register"]
    pub btimr: BTIMR,
    _reserved8: [u8; 0x0160],
    #[doc = "0x180 - CAN TX mailbox identifier register"]
    pub txmir0: TXMIR0,
    #[doc = "0x184 - CAN mailbox data length control and time stamp register"]
    pub txmdtr0: TXMDTR0,
    #[doc = "0x188 - CAN mailbox data low register"]
    pub txmdlr0: TXMDLR0,
    #[doc = "0x18c - CAN mailbox data high register"]
    pub txmdhr0: TXMDHR0,
    #[doc = "0x190 - CAN TX mailbox identifier register"]
    pub txmir1: TXMIR1,
    #[doc = "0x194 - CAN mailbox data length control and time stamp register"]
    pub txmdtr1: TXMDTR1,
    #[doc = "0x198 - CAN mailbox data low register"]
    pub txmdlr1: TXMDLR1,
    #[doc = "0x19c - CAN mailbox data high register"]
    pub txmdhr1: TXMDHR1,
    #[doc = "0x1a0 - CAN TX mailbox identifier register"]
    pub txmir2: TXMIR2,
    #[doc = "0x1a4 - CAN mailbox data length control and time stamp register"]
    pub txmdtr2: TXMDTR2,
    #[doc = "0x1a8 - CAN mailbox data low register"]
    pub txmdlr2: TXMDLR2,
    #[doc = "0x1ac - CAN mailbox data high register"]
    pub txmdhr2: TXMDHR2,
    #[doc = "0x1b0 - CAN receive FIFO mailbox identifier register"]
    pub rxmir0: RXMIR0,
    #[doc = "0x1b4 - CAN receive FIFO mailbox data length control and time stamp register"]
    pub rxmdtr0: RXMDTR0,
    #[doc = "0x1b8 - CAN receive FIFO mailbox data low register"]
    pub rxmdlr0: RXMDLR0,
    #[doc = "0x1bc - CAN receive FIFO mailbox data high register"]
    pub rxmdhr0: RXMDHR0,
    #[doc = "0x1c0 - CAN receive FIFO mailbox identifier register"]
    pub rxmir1: RXMIR1,
    #[doc = "0x1c4 - CAN receive FIFO mailbox data length control and time stamp register"]
    pub rxmdtr1: RXMDTR1,
    #[doc = "0x1c8 - CAN receive FIFO mailbox data low register"]
    pub rxmdlr1: RXMDLR1,
    #[doc = "0x1cc - CAN receive FIFO mailbox data high register"]
    pub rxmdhr1: RXMDHR1,
    _reserved28: [u8; 0x30],
    #[doc = "0x200 - CAN filter master register"]
    pub fctlr: FCTLR,
    #[doc = "0x204 - CAN filter mode register"]
    pub fmcfgr: FMCFGR,
    _reserved30: [u8; 0x04],
    #[doc = "0x20c - CAN filter scale register"]
    pub fscfgr: FSCFGR,
    _reserved31: [u8; 0x04],
    #[doc = "0x214 - CAN filter FIFO assignment register"]
    pub fafifor: FAFIFOR,
    _reserved32: [u8; 0x04],
    #[doc = "0x21c - CAN filter activation register"]
    pub fwr: FWR,
    _reserved33: [u8; 0x20],
    #[doc = "0x240 - Filter bank 0 register 1"]
    pub f0r1: F0R1,
    #[doc = "0x244 - Filter bank 0 register 2"]
    pub f0r2: F0R2,
    #[doc = "0x248 - Filter bank 1 register 1"]
    pub f1r1: F1R1,
    #[doc = "0x24c - Filter bank 1 register 2"]
    pub f1r2: F1R2,
    #[doc = "0x250 - Filter bank 2 register 1"]
    pub f2r1: F2R1,
    #[doc = "0x254 - Filter bank 2 register 2"]
    pub f2r2: F2R2,
    #[doc = "0x258 - Filter bank 3 register 1"]
    pub f3r1: F3R1,
    #[doc = "0x25c - Filter bank 3 register 2"]
    pub f3r2: F3R2,
    #[doc = "0x260 - Filter bank 4 register 1"]
    pub f4r1: F4R1,
    #[doc = "0x264 - Filter bank 4 register 2"]
    pub f4r2: F4R2,
    #[doc = "0x268 - Filter bank 5 register 1"]
    pub f5r1: F5R1,
    #[doc = "0x26c - Filter bank 5 register 2"]
    pub f5r2: F5R2,
    #[doc = "0x270 - Filter bank 6 register 1"]
    pub f6r1: F6R1,
    #[doc = "0x274 - Filter bank 6 register 2"]
    pub f6r2: F6R2,
    #[doc = "0x278 - Filter bank 7 register 1"]
    pub f7r1: F7R1,
    #[doc = "0x27c - Filter bank 7 register 2"]
    pub f7r2: F7R2,
    #[doc = "0x280 - Filter bank 8 register 1"]
    pub f8r1: F8R1,
    #[doc = "0x284 - Filter bank 8 register 2"]
    pub f8r2: F8R2,
    #[doc = "0x288 - Filter bank 9 register 1"]
    pub f9r1: F9R1,
    #[doc = "0x28c - Filter bank 9 register 2"]
    pub f9r2: F9R2,
    #[doc = "0x290 - Filter bank 10 register 1"]
    pub f10r1: F10R1,
    #[doc = "0x294 - Filter bank 10 register 2"]
    pub f10r2: F10R2,
    #[doc = "0x298 - Filter bank 11 register 1"]
    pub f11r1: F11R1,
    #[doc = "0x29c - Filter bank 11 register 2"]
    pub f11r2: F11R2,
    #[doc = "0x2a0 - Filter bank 4 register 1"]
    pub f12r1: F12R1,
    #[doc = "0x2a4 - Filter bank 12 register 2"]
    pub f12r2: F12R2,
    #[doc = "0x2a8 - Filter bank 13 register 1"]
    pub f13r1: F13R1,
    #[doc = "0x2ac - Filter bank 13 register 2"]
    pub f13r2: F13R2,
    #[doc = "0x2b0 - Filter bank 14 register 1"]
    pub f14r1: F14R1,
    #[doc = "0x2b4 - Filter bank 14 register 2"]
    pub f14r2: F14R2,
    #[doc = "0x2b8 - Filter bank 15 register 1"]
    pub f15r1: F15R1,
    #[doc = "0x2bc - Filter bank 15 register 2"]
    pub f15r2: F15R2,
    #[doc = "0x2c0 - Filter bank 16 register 1"]
    pub f16r1: F16R1,
    #[doc = "0x2c4 - Filter bank 16 register 2"]
    pub f16r2: F16R2,
    #[doc = "0x2c8 - Filter bank 17 register 1"]
    pub f17r1: F17R1,
    #[doc = "0x2cc - Filter bank 17 register 2"]
    pub f17r2: F17R2,
    #[doc = "0x2d0 - Filter bank 18 register 1"]
    pub f18r1: F18R1,
    #[doc = "0x2d4 - Filter bank 18 register 2"]
    pub f18r2: F18R2,
    #[doc = "0x2d8 - Filter bank 19 register 1"]
    pub f19r1: F19R1,
    #[doc = "0x2dc - Filter bank 19 register 2"]
    pub f19r2: F19R2,
    #[doc = "0x2e0 - Filter bank 20 register 1"]
    pub f20r1: F20R1,
    #[doc = "0x2e4 - Filter bank 20 register 2"]
    pub f20r2: F20R2,
    #[doc = "0x2e8 - Filter bank 21 register 1"]
    pub f21r1: F21R1,
    #[doc = "0x2ec - Filter bank 21 register 2"]
    pub f21r2: F21R2,
    #[doc = "0x2f0 - Filter bank 22 register 1"]
    pub f22r1: F22R1,
    #[doc = "0x2f4 - Filter bank 22 register 2"]
    pub f22r2: F22R2,
    #[doc = "0x2f8 - Filter bank 23 register 1"]
    pub f23r1: F23R1,
    #[doc = "0x2fc - Filter bank 23 register 2"]
    pub f23r2: F23R2,
    #[doc = "0x300 - Filter bank 24 register 1"]
    pub f24r1: F24R1,
    #[doc = "0x304 - Filter bank 24 register 2"]
    pub f24r2: F24R2,
    #[doc = "0x308 - Filter bank 25 register 1"]
    pub f25r1: F25R1,
    #[doc = "0x30c - Filter bank 25 register 2"]
    pub f25r2: F25R2,
    #[doc = "0x310 - Filter bank 26 register 1"]
    pub f26r1: F26R1,
    #[doc = "0x314 - Filter bank 26 register 2"]
    pub f26r2: F26R2,
    #[doc = "0x318 - Filter bank 27 register 1"]
    pub f27r1: F27R1,
    #[doc = "0x31c - Filter bank 27 register 2"]
    pub f27r2: F27R2,
}
#[doc = "CTLR (rw) register accessor: CAN Master control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlr`]
module"]
pub type CTLR = crate::Reg<ctlr::CTLR_SPEC>;
#[doc = "CAN Master control register"]
pub mod ctlr;
#[doc = "STATR (rw) register accessor: CAN master status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`statr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statr`]
module"]
pub type STATR = crate::Reg<statr::STATR_SPEC>;
#[doc = "CAN master status register"]
pub mod statr;
#[doc = "TSTATR (rw) register accessor: CAN transmit status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tstatr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tstatr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstatr`]
module"]
pub type TSTATR = crate::Reg<tstatr::TSTATR_SPEC>;
#[doc = "CAN transmit status register"]
pub mod tstatr;
#[doc = "RFIFO0 (rw) register accessor: CAN receive FIFO 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifo0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfifo0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfifo0`]
module"]
pub type RFIFO0 = crate::Reg<rfifo0::RFIFO0_SPEC>;
#[doc = "CAN receive FIFO 0 register"]
pub mod rfifo0;
#[doc = "RFIFO1 (rw) register accessor: CAN receive FIFO 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifo1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfifo1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfifo1`]
module"]
pub type RFIFO1 = crate::Reg<rfifo1::RFIFO1_SPEC>;
#[doc = "CAN receive FIFO 1 register"]
pub mod rfifo1;
#[doc = "INTENR (rw) register accessor: CAN interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenr`]
module"]
pub type INTENR = crate::Reg<intenr::INTENR_SPEC>;
#[doc = "CAN interrupt enable register"]
pub mod intenr;
#[doc = "ERRSR (rw) register accessor: CAN error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`errsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errsr`]
module"]
pub type ERRSR = crate::Reg<errsr::ERRSR_SPEC>;
#[doc = "CAN error status register"]
pub mod errsr;
#[doc = "BTIMR (rw) register accessor: CAN bit timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btimr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btimr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btimr`]
module"]
pub type BTIMR = crate::Reg<btimr::BTIMR_SPEC>;
#[doc = "CAN bit timing register"]
pub mod btimr;
#[doc = "TXMIR0 (rw) register accessor: CAN TX mailbox identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmir0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmir0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmir0`]
module"]
pub type TXMIR0 = crate::Reg<txmir0::TXMIR0_SPEC>;
#[doc = "CAN TX mailbox identifier register"]
pub mod txmir0;
#[doc = "TXMDTR0 (rw) register accessor: CAN mailbox data length control and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmdtr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmdtr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmdtr0`]
module"]
pub type TXMDTR0 = crate::Reg<txmdtr0::TXMDTR0_SPEC>;
#[doc = "CAN mailbox data length control and time stamp register"]
pub mod txmdtr0;
#[doc = "TXMDLR0 (rw) register accessor: CAN mailbox data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmdlr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmdlr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmdlr0`]
module"]
pub type TXMDLR0 = crate::Reg<txmdlr0::TXMDLR0_SPEC>;
#[doc = "CAN mailbox data low register"]
pub mod txmdlr0;
#[doc = "TXMDHR0 (rw) register accessor: CAN mailbox data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmdhr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmdhr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmdhr0`]
module"]
pub type TXMDHR0 = crate::Reg<txmdhr0::TXMDHR0_SPEC>;
#[doc = "CAN mailbox data high register"]
pub mod txmdhr0;
#[doc = "TXMIR1 (rw) register accessor: CAN TX mailbox identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmir1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmir1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmir1`]
module"]
pub type TXMIR1 = crate::Reg<txmir1::TXMIR1_SPEC>;
#[doc = "CAN TX mailbox identifier register"]
pub mod txmir1;
#[doc = "TXMDTR1 (rw) register accessor: CAN mailbox data length control and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmdtr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmdtr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmdtr1`]
module"]
pub type TXMDTR1 = crate::Reg<txmdtr1::TXMDTR1_SPEC>;
#[doc = "CAN mailbox data length control and time stamp register"]
pub mod txmdtr1;
#[doc = "TXMDLR1 (rw) register accessor: CAN mailbox data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmdlr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmdlr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmdlr1`]
module"]
pub type TXMDLR1 = crate::Reg<txmdlr1::TXMDLR1_SPEC>;
#[doc = "CAN mailbox data low register"]
pub mod txmdlr1;
#[doc = "TXMDHR1 (rw) register accessor: CAN mailbox data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmdhr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmdhr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmdhr1`]
module"]
pub type TXMDHR1 = crate::Reg<txmdhr1::TXMDHR1_SPEC>;
#[doc = "CAN mailbox data high register"]
pub mod txmdhr1;
#[doc = "TXMIR2 (rw) register accessor: CAN TX mailbox identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmir2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmir2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmir2`]
module"]
pub type TXMIR2 = crate::Reg<txmir2::TXMIR2_SPEC>;
#[doc = "CAN TX mailbox identifier register"]
pub mod txmir2;
#[doc = "TXMDTR2 (rw) register accessor: CAN mailbox data length control and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmdtr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmdtr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmdtr2`]
module"]
pub type TXMDTR2 = crate::Reg<txmdtr2::TXMDTR2_SPEC>;
#[doc = "CAN mailbox data length control and time stamp register"]
pub mod txmdtr2;
#[doc = "TXMDLR2 (rw) register accessor: CAN mailbox data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmdlr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmdlr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmdlr2`]
module"]
pub type TXMDLR2 = crate::Reg<txmdlr2::TXMDLR2_SPEC>;
#[doc = "CAN mailbox data low register"]
pub mod txmdlr2;
#[doc = "TXMDHR2 (rw) register accessor: CAN mailbox data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmdhr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmdhr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmdhr2`]
module"]
pub type TXMDHR2 = crate::Reg<txmdhr2::TXMDHR2_SPEC>;
#[doc = "CAN mailbox data high register"]
pub mod txmdhr2;
#[doc = "RXMIR0 (r) register accessor: CAN receive FIFO mailbox identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmir0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmir0`]
module"]
pub type RXMIR0 = crate::Reg<rxmir0::RXMIR0_SPEC>;
#[doc = "CAN receive FIFO mailbox identifier register"]
pub mod rxmir0;
#[doc = "RXMDTR0 (r) register accessor: CAN receive FIFO mailbox data length control and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmdtr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmdtr0`]
module"]
pub type RXMDTR0 = crate::Reg<rxmdtr0::RXMDTR0_SPEC>;
#[doc = "CAN receive FIFO mailbox data length control and time stamp register"]
pub mod rxmdtr0;
#[doc = "RXMDLR0 (r) register accessor: CAN receive FIFO mailbox data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmdlr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmdlr0`]
module"]
pub type RXMDLR0 = crate::Reg<rxmdlr0::RXMDLR0_SPEC>;
#[doc = "CAN receive FIFO mailbox data low register"]
pub mod rxmdlr0;
#[doc = "RXMDHR0 (r) register accessor: CAN receive FIFO mailbox data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmdhr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmdhr0`]
module"]
pub type RXMDHR0 = crate::Reg<rxmdhr0::RXMDHR0_SPEC>;
#[doc = "CAN receive FIFO mailbox data high register"]
pub mod rxmdhr0;
#[doc = "RXMIR1 (r) register accessor: CAN receive FIFO mailbox identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmir1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmir1`]
module"]
pub type RXMIR1 = crate::Reg<rxmir1::RXMIR1_SPEC>;
#[doc = "CAN receive FIFO mailbox identifier register"]
pub mod rxmir1;
#[doc = "RXMDTR1 (r) register accessor: CAN receive FIFO mailbox data length control and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmdtr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmdtr1`]
module"]
pub type RXMDTR1 = crate::Reg<rxmdtr1::RXMDTR1_SPEC>;
#[doc = "CAN receive FIFO mailbox data length control and time stamp register"]
pub mod rxmdtr1;
#[doc = "RXMDLR1 (r) register accessor: CAN receive FIFO mailbox data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmdlr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmdlr1`]
module"]
pub type RXMDLR1 = crate::Reg<rxmdlr1::RXMDLR1_SPEC>;
#[doc = "CAN receive FIFO mailbox data low register"]
pub mod rxmdlr1;
#[doc = "RXMDHR1 (r) register accessor: CAN receive FIFO mailbox data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmdhr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmdhr1`]
module"]
pub type RXMDHR1 = crate::Reg<rxmdhr1::RXMDHR1_SPEC>;
#[doc = "CAN receive FIFO mailbox data high register"]
pub mod rxmdhr1;
#[doc = "FCTLR (rw) register accessor: CAN filter master register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctlr`]
module"]
pub type FCTLR = crate::Reg<fctlr::FCTLR_SPEC>;
#[doc = "CAN filter master register"]
pub mod fctlr;
#[doc = "FMCFGR (rw) register accessor: CAN filter mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmcfgr`]
module"]
pub type FMCFGR = crate::Reg<fmcfgr::FMCFGR_SPEC>;
#[doc = "CAN filter mode register"]
pub mod fmcfgr;
#[doc = "FSCFGR (rw) register accessor: CAN filter scale register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fscfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fscfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fscfgr`]
module"]
pub type FSCFGR = crate::Reg<fscfgr::FSCFGR_SPEC>;
#[doc = "CAN filter scale register"]
pub mod fscfgr;
#[doc = "FAFIFOR (rw) register accessor: CAN filter FIFO assignment register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fafifor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fafifor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fafifor`]
module"]
pub type FAFIFOR = crate::Reg<fafifor::FAFIFOR_SPEC>;
#[doc = "CAN filter FIFO assignment register"]
pub mod fafifor;
#[doc = "FWR (rw) register accessor: CAN filter activation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwr`]
module"]
pub type FWR = crate::Reg<fwr::FWR_SPEC>;
#[doc = "CAN filter activation register"]
pub mod fwr;
#[doc = "F0R1 (rw) register accessor: Filter bank 0 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f0r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f0r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f0r1`]
module"]
pub type F0R1 = crate::Reg<f0r1::F0R1_SPEC>;
#[doc = "Filter bank 0 register 1"]
pub mod f0r1;
#[doc = "F0R2 (rw) register accessor: Filter bank 0 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f0r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f0r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f0r2`]
module"]
pub type F0R2 = crate::Reg<f0r2::F0R2_SPEC>;
#[doc = "Filter bank 0 register 2"]
pub mod f0r2;
#[doc = "F1R1 (rw) register accessor: Filter bank 1 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f1r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f1r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f1r1`]
module"]
pub type F1R1 = crate::Reg<f1r1::F1R1_SPEC>;
#[doc = "Filter bank 1 register 1"]
pub mod f1r1;
#[doc = "F1R2 (rw) register accessor: Filter bank 1 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f1r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f1r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f1r2`]
module"]
pub type F1R2 = crate::Reg<f1r2::F1R2_SPEC>;
#[doc = "Filter bank 1 register 2"]
pub mod f1r2;
#[doc = "F2R1 (rw) register accessor: Filter bank 2 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f2r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f2r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f2r1`]
module"]
pub type F2R1 = crate::Reg<f2r1::F2R1_SPEC>;
#[doc = "Filter bank 2 register 1"]
pub mod f2r1;
#[doc = "F2R2 (rw) register accessor: Filter bank 2 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f2r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f2r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f2r2`]
module"]
pub type F2R2 = crate::Reg<f2r2::F2R2_SPEC>;
#[doc = "Filter bank 2 register 2"]
pub mod f2r2;
#[doc = "F3R1 (rw) register accessor: Filter bank 3 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f3r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f3r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f3r1`]
module"]
pub type F3R1 = crate::Reg<f3r1::F3R1_SPEC>;
#[doc = "Filter bank 3 register 1"]
pub mod f3r1;
#[doc = "F3R2 (rw) register accessor: Filter bank 3 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f3r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f3r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f3r2`]
module"]
pub type F3R2 = crate::Reg<f3r2::F3R2_SPEC>;
#[doc = "Filter bank 3 register 2"]
pub mod f3r2;
#[doc = "F4R1 (rw) register accessor: Filter bank 4 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f4r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f4r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f4r1`]
module"]
pub type F4R1 = crate::Reg<f4r1::F4R1_SPEC>;
#[doc = "Filter bank 4 register 1"]
pub mod f4r1;
#[doc = "F4R2 (rw) register accessor: Filter bank 4 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f4r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f4r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f4r2`]
module"]
pub type F4R2 = crate::Reg<f4r2::F4R2_SPEC>;
#[doc = "Filter bank 4 register 2"]
pub mod f4r2;
#[doc = "F5R1 (rw) register accessor: Filter bank 5 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f5r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f5r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f5r1`]
module"]
pub type F5R1 = crate::Reg<f5r1::F5R1_SPEC>;
#[doc = "Filter bank 5 register 1"]
pub mod f5r1;
#[doc = "F5R2 (rw) register accessor: Filter bank 5 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f5r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f5r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f5r2`]
module"]
pub type F5R2 = crate::Reg<f5r2::F5R2_SPEC>;
#[doc = "Filter bank 5 register 2"]
pub mod f5r2;
#[doc = "F6R1 (rw) register accessor: Filter bank 6 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f6r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f6r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f6r1`]
module"]
pub type F6R1 = crate::Reg<f6r1::F6R1_SPEC>;
#[doc = "Filter bank 6 register 1"]
pub mod f6r1;
#[doc = "F6R2 (rw) register accessor: Filter bank 6 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f6r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f6r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f6r2`]
module"]
pub type F6R2 = crate::Reg<f6r2::F6R2_SPEC>;
#[doc = "Filter bank 6 register 2"]
pub mod f6r2;
#[doc = "F7R1 (rw) register accessor: Filter bank 7 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f7r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f7r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f7r1`]
module"]
pub type F7R1 = crate::Reg<f7r1::F7R1_SPEC>;
#[doc = "Filter bank 7 register 1"]
pub mod f7r1;
#[doc = "F7R2 (rw) register accessor: Filter bank 7 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f7r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f7r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f7r2`]
module"]
pub type F7R2 = crate::Reg<f7r2::F7R2_SPEC>;
#[doc = "Filter bank 7 register 2"]
pub mod f7r2;
#[doc = "F8R1 (rw) register accessor: Filter bank 8 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f8r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f8r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f8r1`]
module"]
pub type F8R1 = crate::Reg<f8r1::F8R1_SPEC>;
#[doc = "Filter bank 8 register 1"]
pub mod f8r1;
#[doc = "F8R2 (rw) register accessor: Filter bank 8 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f8r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f8r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f8r2`]
module"]
pub type F8R2 = crate::Reg<f8r2::F8R2_SPEC>;
#[doc = "Filter bank 8 register 2"]
pub mod f8r2;
#[doc = "F9R1 (rw) register accessor: Filter bank 9 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f9r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f9r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f9r1`]
module"]
pub type F9R1 = crate::Reg<f9r1::F9R1_SPEC>;
#[doc = "Filter bank 9 register 1"]
pub mod f9r1;
#[doc = "F9R2 (rw) register accessor: Filter bank 9 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f9r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f9r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f9r2`]
module"]
pub type F9R2 = crate::Reg<f9r2::F9R2_SPEC>;
#[doc = "Filter bank 9 register 2"]
pub mod f9r2;
#[doc = "F10R1 (rw) register accessor: Filter bank 10 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f10r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f10r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f10r1`]
module"]
pub type F10R1 = crate::Reg<f10r1::F10R1_SPEC>;
#[doc = "Filter bank 10 register 1"]
pub mod f10r1;
#[doc = "F10R2 (rw) register accessor: Filter bank 10 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f10r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f10r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f10r2`]
module"]
pub type F10R2 = crate::Reg<f10r2::F10R2_SPEC>;
#[doc = "Filter bank 10 register 2"]
pub mod f10r2;
#[doc = "F11R1 (rw) register accessor: Filter bank 11 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f11r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f11r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f11r1`]
module"]
pub type F11R1 = crate::Reg<f11r1::F11R1_SPEC>;
#[doc = "Filter bank 11 register 1"]
pub mod f11r1;
#[doc = "F11R2 (rw) register accessor: Filter bank 11 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f11r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f11r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f11r2`]
module"]
pub type F11R2 = crate::Reg<f11r2::F11R2_SPEC>;
#[doc = "Filter bank 11 register 2"]
pub mod f11r2;
#[doc = "F12R1 (rw) register accessor: Filter bank 4 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f12r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f12r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f12r1`]
module"]
pub type F12R1 = crate::Reg<f12r1::F12R1_SPEC>;
#[doc = "Filter bank 4 register 1"]
pub mod f12r1;
#[doc = "F12R2 (rw) register accessor: Filter bank 12 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f12r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f12r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f12r2`]
module"]
pub type F12R2 = crate::Reg<f12r2::F12R2_SPEC>;
#[doc = "Filter bank 12 register 2"]
pub mod f12r2;
#[doc = "F13R1 (rw) register accessor: Filter bank 13 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f13r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f13r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f13r1`]
module"]
pub type F13R1 = crate::Reg<f13r1::F13R1_SPEC>;
#[doc = "Filter bank 13 register 1"]
pub mod f13r1;
#[doc = "F13R2 (rw) register accessor: Filter bank 13 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f13r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f13r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f13r2`]
module"]
pub type F13R2 = crate::Reg<f13r2::F13R2_SPEC>;
#[doc = "Filter bank 13 register 2"]
pub mod f13r2;
#[doc = "F14R1 (rw) register accessor: Filter bank 14 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f14r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f14r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f14r1`]
module"]
pub type F14R1 = crate::Reg<f14r1::F14R1_SPEC>;
#[doc = "Filter bank 14 register 1"]
pub mod f14r1;
#[doc = "F14R2 (rw) register accessor: Filter bank 14 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f14r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f14r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f14r2`]
module"]
pub type F14R2 = crate::Reg<f14r2::F14R2_SPEC>;
#[doc = "Filter bank 14 register 2"]
pub mod f14r2;
#[doc = "F15R1 (rw) register accessor: Filter bank 15 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f15r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f15r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f15r1`]
module"]
pub type F15R1 = crate::Reg<f15r1::F15R1_SPEC>;
#[doc = "Filter bank 15 register 1"]
pub mod f15r1;
#[doc = "F15R2 (rw) register accessor: Filter bank 15 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f15r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f15r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f15r2`]
module"]
pub type F15R2 = crate::Reg<f15r2::F15R2_SPEC>;
#[doc = "Filter bank 15 register 2"]
pub mod f15r2;
#[doc = "F16R1 (rw) register accessor: Filter bank 16 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f16r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f16r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f16r1`]
module"]
pub type F16R1 = crate::Reg<f16r1::F16R1_SPEC>;
#[doc = "Filter bank 16 register 1"]
pub mod f16r1;
#[doc = "F16R2 (rw) register accessor: Filter bank 16 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f16r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f16r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f16r2`]
module"]
pub type F16R2 = crate::Reg<f16r2::F16R2_SPEC>;
#[doc = "Filter bank 16 register 2"]
pub mod f16r2;
#[doc = "F17R1 (rw) register accessor: Filter bank 17 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f17r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f17r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f17r1`]
module"]
pub type F17R1 = crate::Reg<f17r1::F17R1_SPEC>;
#[doc = "Filter bank 17 register 1"]
pub mod f17r1;
#[doc = "F17R2 (rw) register accessor: Filter bank 17 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f17r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f17r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f17r2`]
module"]
pub type F17R2 = crate::Reg<f17r2::F17R2_SPEC>;
#[doc = "Filter bank 17 register 2"]
pub mod f17r2;
#[doc = "F18R1 (rw) register accessor: Filter bank 18 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f18r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f18r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f18r1`]
module"]
pub type F18R1 = crate::Reg<f18r1::F18R1_SPEC>;
#[doc = "Filter bank 18 register 1"]
pub mod f18r1;
#[doc = "F18R2 (rw) register accessor: Filter bank 18 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f18r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f18r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f18r2`]
module"]
pub type F18R2 = crate::Reg<f18r2::F18R2_SPEC>;
#[doc = "Filter bank 18 register 2"]
pub mod f18r2;
#[doc = "F19R1 (rw) register accessor: Filter bank 19 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f19r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f19r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f19r1`]
module"]
pub type F19R1 = crate::Reg<f19r1::F19R1_SPEC>;
#[doc = "Filter bank 19 register 1"]
pub mod f19r1;
#[doc = "F19R2 (rw) register accessor: Filter bank 19 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f19r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f19r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f19r2`]
module"]
pub type F19R2 = crate::Reg<f19r2::F19R2_SPEC>;
#[doc = "Filter bank 19 register 2"]
pub mod f19r2;
#[doc = "F20R1 (rw) register accessor: Filter bank 20 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f20r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f20r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f20r1`]
module"]
pub type F20R1 = crate::Reg<f20r1::F20R1_SPEC>;
#[doc = "Filter bank 20 register 1"]
pub mod f20r1;
#[doc = "F20R2 (rw) register accessor: Filter bank 20 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f20r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f20r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f20r2`]
module"]
pub type F20R2 = crate::Reg<f20r2::F20R2_SPEC>;
#[doc = "Filter bank 20 register 2"]
pub mod f20r2;
#[doc = "F21R1 (rw) register accessor: Filter bank 21 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f21r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f21r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f21r1`]
module"]
pub type F21R1 = crate::Reg<f21r1::F21R1_SPEC>;
#[doc = "Filter bank 21 register 1"]
pub mod f21r1;
#[doc = "F21R2 (rw) register accessor: Filter bank 21 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f21r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f21r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f21r2`]
module"]
pub type F21R2 = crate::Reg<f21r2::F21R2_SPEC>;
#[doc = "Filter bank 21 register 2"]
pub mod f21r2;
#[doc = "F22R1 (rw) register accessor: Filter bank 22 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f22r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f22r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f22r1`]
module"]
pub type F22R1 = crate::Reg<f22r1::F22R1_SPEC>;
#[doc = "Filter bank 22 register 1"]
pub mod f22r1;
#[doc = "F22R2 (rw) register accessor: Filter bank 22 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f22r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f22r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f22r2`]
module"]
pub type F22R2 = crate::Reg<f22r2::F22R2_SPEC>;
#[doc = "Filter bank 22 register 2"]
pub mod f22r2;
#[doc = "F23R1 (rw) register accessor: Filter bank 23 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f23r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f23r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f23r1`]
module"]
pub type F23R1 = crate::Reg<f23r1::F23R1_SPEC>;
#[doc = "Filter bank 23 register 1"]
pub mod f23r1;
#[doc = "F23R2 (rw) register accessor: Filter bank 23 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f23r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f23r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f23r2`]
module"]
pub type F23R2 = crate::Reg<f23r2::F23R2_SPEC>;
#[doc = "Filter bank 23 register 2"]
pub mod f23r2;
#[doc = "F24R1 (rw) register accessor: Filter bank 24 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f24r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f24r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f24r1`]
module"]
pub type F24R1 = crate::Reg<f24r1::F24R1_SPEC>;
#[doc = "Filter bank 24 register 1"]
pub mod f24r1;
#[doc = "F24R2 (rw) register accessor: Filter bank 24 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f24r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f24r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f24r2`]
module"]
pub type F24R2 = crate::Reg<f24r2::F24R2_SPEC>;
#[doc = "Filter bank 24 register 2"]
pub mod f24r2;
#[doc = "F25R1 (rw) register accessor: Filter bank 25 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f25r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f25r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f25r1`]
module"]
pub type F25R1 = crate::Reg<f25r1::F25R1_SPEC>;
#[doc = "Filter bank 25 register 1"]
pub mod f25r1;
#[doc = "F25R2 (rw) register accessor: Filter bank 25 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f25r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f25r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f25r2`]
module"]
pub type F25R2 = crate::Reg<f25r2::F25R2_SPEC>;
#[doc = "Filter bank 25 register 2"]
pub mod f25r2;
#[doc = "F26R1 (rw) register accessor: Filter bank 26 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f26r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f26r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f26r1`]
module"]
pub type F26R1 = crate::Reg<f26r1::F26R1_SPEC>;
#[doc = "Filter bank 26 register 1"]
pub mod f26r1;
#[doc = "F26R2 (rw) register accessor: Filter bank 26 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f26r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f26r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f26r2`]
module"]
pub type F26R2 = crate::Reg<f26r2::F26R2_SPEC>;
#[doc = "Filter bank 26 register 2"]
pub mod f26r2;
#[doc = "F27R1 (rw) register accessor: Filter bank 27 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f27r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f27r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f27r1`]
module"]
pub type F27R1 = crate::Reg<f27r1::F27R1_SPEC>;
#[doc = "Filter bank 27 register 1"]
pub mod f27r1;
#[doc = "F27R2 (rw) register accessor: Filter bank 27 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f27r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f27r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f27r2`]
module"]
pub type F27R2 = crate::Reg<f27r2::F27R2_SPEC>;
#[doc = "Filter bank 27 register 2"]
pub mod f27r2;
