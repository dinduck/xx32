#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port configuration register low (GPIOn_CFGLR)"]
    pub cfglr: CFGLR,
    #[doc = "0x04 - Port configuration register high (GPIOn_CFGHR)"]
    pub cfghr: CFGHR,
    #[doc = "0x08 - Port input data register (GPIOn_INDR)"]
    pub indr: INDR,
    #[doc = "0x0c - Port output data register (GPIOn_OUTDR)"]
    pub outdr: OUTDR,
    #[doc = "0x10 - Port bit set/reset register (GPIOn_BSHR)"]
    pub bshr: BSHR,
    #[doc = "0x14 - Port bit reset register (GPIOn_BCR)"]
    pub bcr: BCR,
    #[doc = "0x18 - Port configuration lock register"]
    pub lckr: LCKR,
}
#[doc = "CFGLR (rw) register accessor: Port configuration register low (GPIOn_CFGLR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfglr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfglr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfglr`]
module"]
pub type CFGLR = crate::Reg<cfglr::CFGLR_SPEC>;
#[doc = "Port configuration register low (GPIOn_CFGLR)"]
pub mod cfglr;
#[doc = "CFGHR (rw) register accessor: Port configuration register high (GPIOn_CFGHR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfghr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfghr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfghr`]
module"]
pub type CFGHR = crate::Reg<cfghr::CFGHR_SPEC>;
#[doc = "Port configuration register high (GPIOn_CFGHR)"]
pub mod cfghr;
#[doc = "INDR (r) register accessor: Port input data register (GPIOn_INDR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indr`]
module"]
pub type INDR = crate::Reg<indr::INDR_SPEC>;
#[doc = "Port input data register (GPIOn_INDR)"]
pub mod indr;
#[doc = "OUTDR (rw) register accessor: Port output data register (GPIOn_OUTDR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outdr`]
module"]
pub type OUTDR = crate::Reg<outdr::OUTDR_SPEC>;
#[doc = "Port output data register (GPIOn_OUTDR)"]
pub mod outdr;
#[doc = "BSHR (w) register accessor: Port bit set/reset register (GPIOn_BSHR)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bshr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bshr`]
module"]
pub type BSHR = crate::Reg<bshr::BSHR_SPEC>;
#[doc = "Port bit set/reset register (GPIOn_BSHR)"]
pub mod bshr;
#[doc = "BCR (w) register accessor: Port bit reset register (GPIOn_BCR)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr`]
module"]
pub type BCR = crate::Reg<bcr::BCR_SPEC>;
#[doc = "Port bit reset register (GPIOn_BCR)"]
pub mod bcr;
#[doc = "LCKR (rw) register accessor: Port configuration lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lckr`]
module"]
pub type LCKR = crate::Reg<lckr::LCKR_SPEC>;
#[doc = "Port configuration lock register"]
pub mod lckr;
