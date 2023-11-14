#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 1"]
    pub ctlr1: CTLR1,
    #[doc = "0x04 - Control register 2"]
    pub ctlr2: CTLR2,
    #[doc = "0x08 - Own address register 1"]
    pub oaddr1: OADDR1,
    #[doc = "0x0c - Own address register 2"]
    pub oaddr2: OADDR2,
    #[doc = "0x10 - Data register"]
    pub datar: DATAR,
    #[doc = "0x14 - Status register 1"]
    pub star1: STAR1,
    #[doc = "0x18 - Status register 2"]
    pub star2: STAR2,
    #[doc = "0x1c - Clock control register"]
    pub ckcfgr: CKCFGR,
    #[doc = "0x20 - Raise time register"]
    pub rtr: RTR,
}
#[doc = "CTLR1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlr1`]
module"]
pub type CTLR1 = crate::Reg<ctlr1::CTLR1_SPEC>;
#[doc = "Control register 1"]
pub mod ctlr1;
#[doc = "CTLR2 (rw) register accessor: Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlr2`]
module"]
pub type CTLR2 = crate::Reg<ctlr2::CTLR2_SPEC>;
#[doc = "Control register 2"]
pub mod ctlr2;
#[doc = "OADDR1 (rw) register accessor: Own address register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oaddr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oaddr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oaddr1`]
module"]
pub type OADDR1 = crate::Reg<oaddr1::OADDR1_SPEC>;
#[doc = "Own address register 1"]
pub mod oaddr1;
#[doc = "OADDR2 (rw) register accessor: Own address register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oaddr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oaddr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oaddr2`]
module"]
pub type OADDR2 = crate::Reg<oaddr2::OADDR2_SPEC>;
#[doc = "Own address register 2"]
pub mod oaddr2;
#[doc = "DATAR (rw) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar`]
module"]
pub type DATAR = crate::Reg<datar::DATAR_SPEC>;
#[doc = "Data register"]
pub mod datar;
#[doc = "STAR1 (rw) register accessor: Status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`star1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`star1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@star1`]
module"]
pub type STAR1 = crate::Reg<star1::STAR1_SPEC>;
#[doc = "Status register 1"]
pub mod star1;
#[doc = "STAR2 (r) register accessor: Status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`star2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@star2`]
module"]
pub type STAR2 = crate::Reg<star2::STAR2_SPEC>;
#[doc = "Status register 2"]
pub mod star2;
#[doc = "CKCFGR (rw) register accessor: Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckcfgr`]
module"]
pub type CKCFGR = crate::Reg<ckcfgr::CKCFGR_SPEC>;
#[doc = "Clock control register"]
pub mod ckcfgr;
#[doc = "RTR (rw) register accessor: Raise time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtr`]
module"]
pub type RTR = crate::Reg<rtr::RTR_SPEC>;
#[doc = "Raise time register"]
pub mod rtr;
