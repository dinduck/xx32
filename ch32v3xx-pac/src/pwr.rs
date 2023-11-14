#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power control register (PWR_CTRL)"]
    pub ctlr: CTLR,
    #[doc = "0x04 - Power control register (PWR_CSR)"]
    pub csr: CSR,
}
#[doc = "CTLR (rw) register accessor: Power control register (PWR_CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlr`]
module"]
pub type CTLR = crate::Reg<ctlr::CTLR_SPEC>;
#[doc = "Power control register (PWR_CTRL)"]
pub mod ctlr;
#[doc = "CSR (rw) register accessor: Power control register (PWR_CSR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Power control register (PWR_CSR)"]
pub mod csr;
