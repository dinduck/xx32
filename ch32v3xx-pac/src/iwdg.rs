#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Key register (IWDG_CTLR)"]
    pub ctlr: CTLR,
    #[doc = "0x04 - Prescaler register (IWDG_PSCR)"]
    pub pscr: PSCR,
    #[doc = "0x08 - Reload register (IWDG_RLDR)"]
    pub rldr: RLDR,
    #[doc = "0x0c - Status register (IWDG_SR)"]
    pub statr: STATR,
}
#[doc = "CTLR (w) register accessor: Key register (IWDG_CTLR)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlr`]
module"]
pub type CTLR = crate::Reg<ctlr::CTLR_SPEC>;
#[doc = "Key register (IWDG_CTLR)"]
pub mod ctlr;
#[doc = "PSCR (rw) register accessor: Prescaler register (IWDG_PSCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscr`]
module"]
pub type PSCR = crate::Reg<pscr::PSCR_SPEC>;
#[doc = "Prescaler register (IWDG_PSCR)"]
pub mod pscr;
#[doc = "RLDR (rw) register accessor: Reload register (IWDG_RLDR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rldr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rldr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rldr`]
module"]
pub type RLDR = crate::Reg<rldr::RLDR_SPEC>;
#[doc = "Reload register (IWDG_RLDR)"]
pub mod rldr;
#[doc = "STATR (r) register accessor: Status register (IWDG_SR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statr`]
module"]
pub type STATR = crate::Reg<statr::STATR_SPEC>;
#[doc = "Status register (IWDG_SR)"]
pub mod statr;
