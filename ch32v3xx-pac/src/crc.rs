#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data register"]
    pub datar: DATAR,
    #[doc = "0x04 - Independent Data register"]
    pub idatar: IDATAR,
    #[doc = "0x08 - Control register"]
    pub ctlr: CTLR,
}
#[doc = "DATAR (rw) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar`]
module"]
pub type DATAR = crate::Reg<datar::DATAR_SPEC>;
#[doc = "Data register"]
pub mod datar;
#[doc = "IDATAR (rw) register accessor: Independent Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idatar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idatar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idatar`]
module"]
pub type IDATAR = crate::Reg<idatar::IDATAR_SPEC>;
#[doc = "Independent Data register"]
pub mod idatar;
#[doc = "CTLR (w) register accessor: Control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlr`]
module"]
pub type CTLR = crate::Reg<ctlr::CTLR_SPEC>;
#[doc = "Control register"]
pub mod ctlr;
