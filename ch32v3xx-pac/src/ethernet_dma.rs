#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet DMA bus mode register"]
    pub dmabmr: DMABMR,
    #[doc = "0x04 - Ethernet DMA transmit poll demand register"]
    pub dmatpdr: DMATPDR,
    #[doc = "0x08 - EHERNET DMA receive poll demand register"]
    pub dmarpdr: DMARPDR,
    #[doc = "0x0c - Ethernet DMA receive descriptor list address register"]
    pub dmardlar: DMARDLAR,
    #[doc = "0x10 - Ethernet DMA transmit descriptor list address register"]
    pub dmatdlar: DMATDLAR,
    #[doc = "0x14 - Ethernet DMA status register"]
    pub dmasr: DMASR,
    #[doc = "0x18 - Ethernet DMA operation mode register"]
    pub dmaomr: DMAOMR,
    #[doc = "0x1c - Ethernet DMA interrupt enable register"]
    pub dmaier: DMAIER,
    #[doc = "0x20 - Ethernet DMA missed frame and buffer overflow counter register"]
    pub dmamfbocr: DMAMFBOCR,
    _reserved9: [u8; 0x24],
    #[doc = "0x48 - Ethernet DMA current host transmit descriptor register"]
    pub dmachtdr: DMACHTDR,
    #[doc = "0x4c - Ethernet DMA current host receive descriptor register"]
    pub dmachrdr: DMACHRDR,
    #[doc = "0x50 - Ethernet DMA current host transmit buffer address register"]
    pub dmachtbar: DMACHTBAR,
    #[doc = "0x54 - Ethernet DMA current host receive buffer address register"]
    pub dmachrbar: DMACHRBAR,
}
#[doc = "DMABMR (rw) register accessor: Ethernet DMA bus mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmabmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmabmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmabmr`]
module"]
pub type DMABMR = crate::Reg<dmabmr::DMABMR_SPEC>;
#[doc = "Ethernet DMA bus mode register"]
pub mod dmabmr;
#[doc = "DMATPDR (rw) register accessor: Ethernet DMA transmit poll demand register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatpdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatpdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatpdr`]
module"]
pub type DMATPDR = crate::Reg<dmatpdr::DMATPDR_SPEC>;
#[doc = "Ethernet DMA transmit poll demand register"]
pub mod dmatpdr;
#[doc = "DMARPDR (rw) register accessor: EHERNET DMA receive poll demand register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmarpdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmarpdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmarpdr`]
module"]
pub type DMARPDR = crate::Reg<dmarpdr::DMARPDR_SPEC>;
#[doc = "EHERNET DMA receive poll demand register"]
pub mod dmarpdr;
#[doc = "DMARDLAR (rw) register accessor: Ethernet DMA receive descriptor list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmardlar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmardlar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmardlar`]
module"]
pub type DMARDLAR = crate::Reg<dmardlar::DMARDLAR_SPEC>;
#[doc = "Ethernet DMA receive descriptor list address register"]
pub mod dmardlar;
#[doc = "DMATDLAR (rw) register accessor: Ethernet DMA transmit descriptor list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatdlar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatdlar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatdlar`]
module"]
pub type DMATDLAR = crate::Reg<dmatdlar::DMATDLAR_SPEC>;
#[doc = "Ethernet DMA transmit descriptor list address register"]
pub mod dmatdlar;
#[doc = "DMASR (rw) register accessor: Ethernet DMA status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmasr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmasr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasr`]
module"]
pub type DMASR = crate::Reg<dmasr::DMASR_SPEC>;
#[doc = "Ethernet DMA status register"]
pub mod dmasr;
#[doc = "DMAOMR (rw) register accessor: Ethernet DMA operation mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaomr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaomr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaomr`]
module"]
pub type DMAOMR = crate::Reg<dmaomr::DMAOMR_SPEC>;
#[doc = "Ethernet DMA operation mode register"]
pub mod dmaomr;
#[doc = "DMAIER (rw) register accessor: Ethernet DMA interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaier`]
module"]
pub type DMAIER = crate::Reg<dmaier::DMAIER_SPEC>;
#[doc = "Ethernet DMA interrupt enable register"]
pub mod dmaier;
#[doc = "DMAMFBOCR (r) register accessor: Ethernet DMA missed frame and buffer overflow counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamfbocr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamfbocr`]
module"]
pub type DMAMFBOCR = crate::Reg<dmamfbocr::DMAMFBOCR_SPEC>;
#[doc = "Ethernet DMA missed frame and buffer overflow counter register"]
pub mod dmamfbocr;
#[doc = "DMACHTDR (r) register accessor: Ethernet DMA current host transmit descriptor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmachtdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmachtdr`]
module"]
pub type DMACHTDR = crate::Reg<dmachtdr::DMACHTDR_SPEC>;
#[doc = "Ethernet DMA current host transmit descriptor register"]
pub mod dmachtdr;
#[doc = "DMACHRDR (r) register accessor: Ethernet DMA current host receive descriptor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmachrdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmachrdr`]
module"]
pub type DMACHRDR = crate::Reg<dmachrdr::DMACHRDR_SPEC>;
#[doc = "Ethernet DMA current host receive descriptor register"]
pub mod dmachrdr;
#[doc = "DMACHTBAR (r) register accessor: Ethernet DMA current host transmit buffer address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmachtbar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmachtbar`]
module"]
pub type DMACHTBAR = crate::Reg<dmachtbar::DMACHTBAR_SPEC>;
#[doc = "Ethernet DMA current host transmit buffer address register"]
pub mod dmachtbar;
#[doc = "DMACHRBAR (r) register accessor: Ethernet DMA current host receive buffer address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmachrbar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmachrbar`]
module"]
pub type DMACHRBAR = crate::Reg<dmachrbar::DMACHRBAR_SPEC>;
#[doc = "Ethernet DMA current host receive buffer address register"]
pub mod dmachrbar;
