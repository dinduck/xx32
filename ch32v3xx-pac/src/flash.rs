#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Flash key register"]
    pub keyr: KEYR,
    #[doc = "0x08 - Flash option key register"]
    pub obkeyr: OBKEYR,
    #[doc = "0x0c - Status register"]
    pub statr: STATR,
    #[doc = "0x10 - Control register"]
    pub ctlr: CTLR,
    #[doc = "0x14 - Flash address register"]
    pub addr: ADDR,
    _reserved5: [u8; 0x04],
    #[doc = "0x1c - Option byte register"]
    pub obr: OBR,
    #[doc = "0x20 - Write protection register"]
    pub wpr: WPR,
    #[doc = "0x24 - Mode select register"]
    pub modekeyr: MODEKEYR,
}
#[doc = "KEYR (w) register accessor: Flash key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr`]
module"]
pub type KEYR = crate::Reg<keyr::KEYR_SPEC>;
#[doc = "Flash key register"]
pub mod keyr;
#[doc = "OBKEYR (w) register accessor: Flash option key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obkeyr`]
module"]
pub type OBKEYR = crate::Reg<obkeyr::OBKEYR_SPEC>;
#[doc = "Flash option key register"]
pub mod obkeyr;
#[doc = "STATR (rw) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`statr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statr`]
module"]
pub type STATR = crate::Reg<statr::STATR_SPEC>;
#[doc = "Status register"]
pub mod statr;
#[doc = "CTLR (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlr`]
module"]
pub type CTLR = crate::Reg<ctlr::CTLR_SPEC>;
#[doc = "Control register"]
pub mod ctlr;
#[doc = "ADDR (w) register accessor: Flash address register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Flash address register"]
pub mod addr;
#[doc = "OBR (r) register accessor: Option byte register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obr`]
module"]
pub type OBR = crate::Reg<obr::OBR_SPEC>;
#[doc = "Option byte register"]
pub mod obr;
#[doc = "WPR (r) register accessor: Write protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpr`]
module"]
pub type WPR = crate::Reg<wpr::WPR_SPEC>;
#[doc = "Write protection register"]
pub mod wpr;
#[doc = "MODEKEYR (w) register accessor: Mode select register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modekeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modekeyr`]
module"]
pub type MODEKEYR = crate::Reg<modekeyr::MODEKEYR_SPEC>;
#[doc = "Mode select register"]
pub mod modekeyr;
