#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register (WWDG_CR)"]
    pub ctlr: CTLR,
    #[doc = "0x04 - Configuration register (WWDG_CFR)"]
    pub cfgr: CFGR,
    #[doc = "0x08 - Status register (WWDG_SR)"]
    pub statr: STATR,
}
#[doc = "CTLR (rw) register accessor: Control register (WWDG_CR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlr`]
module"]
pub type CTLR = crate::Reg<ctlr::CTLR_SPEC>;
#[doc = "Control register (WWDG_CR)"]
pub mod ctlr;
#[doc = "CFGR (rw) register accessor: Configuration register (WWDG_CFR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`]
module"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "Configuration register (WWDG_CFR)"]
pub mod cfgr;
#[doc = "STATR (rw) register accessor: Status register (WWDG_SR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`statr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statr`]
module"]
pub type STATR = crate::Reg<statr::STATR_SPEC>;
#[doc = "Status register (WWDG_SR)"]
pub mod statr;
