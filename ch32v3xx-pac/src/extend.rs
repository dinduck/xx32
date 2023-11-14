#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EXTEND register"]
    pub extend_ctr: EXTEND_CTR,
}
#[doc = "EXTEND_CTR (rw) register accessor: EXTEND register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extend_ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extend_ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extend_ctr`]
module"]
pub type EXTEND_CTR = crate::Reg<extend_ctr::EXTEND_CTR_SPEC>;
#[doc = "EXTEND register"]
pub mod extend_ctr;
