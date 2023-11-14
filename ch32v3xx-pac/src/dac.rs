#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register (DAC_CR)"]
    pub ctlr: CTLR,
    #[doc = "0x04 - DAC software trigger register (DAC_SWTRIGR)"]
    pub swtr: SWTR,
    #[doc = "0x08 - DAC channel1 12-bit right-aligned data holding register(DAC_DHR12R1)"]
    pub r12bdhr1: R12BDHR1,
    #[doc = "0x0c - DAC channel1 12-bit left aligned data holding register (DAC_DHR12L1)"]
    pub l12bdhr1: L12BDHR1,
    #[doc = "0x10 - DAC channel1 8-bit right aligned data holding register (DAC_DHR8R1)"]
    pub r8bdhr1: R8BDHR1,
    #[doc = "0x14 - DAC channel2 12-bit right aligned data holding register (DAC_DHR12R2)"]
    pub r12bdhr2: R12BDHR2,
    #[doc = "0x18 - DAC channel2 12-bit left aligned data holding register (DAC_DHR12L2)"]
    pub l12bdhr2: L12BDHR2,
    #[doc = "0x1c - DAC channel2 8-bit right-aligned data holding register (DAC_DHR8R2)"]
    pub r8bdhr2: R8BDHR2,
    #[doc = "0x20 - Dual DAC 12-bit right-aligned data holding register (DAC_DHR12RD), Bits 31:28 Reserved, Bits 15:12 Reserved"]
    pub rd12bdhr: RD12BDHR,
    #[doc = "0x24 - DUAL DAC 12-bit left aligned data holding register (DAC_DHR12LD), Bits 19:16 Reserved, Bits 3:0 Reserved"]
    pub ld12bdhr: LD12BDHR,
    #[doc = "0x28 - DUAL DAC 8-bit right aligned data holding register (DAC_DHR8RD), Bits 31:16 Reserved"]
    pub rd8bdhr: RD8BDHR,
    #[doc = "0x2c - DAC channel1 data output register (DAC_DOR1)"]
    pub dor1: DOR1,
    #[doc = "0x30 - DAC channel2 data output register (DAC_DOR2)"]
    pub dor2: DOR2,
}
#[doc = "CTLR (rw) register accessor: Control register (DAC_CR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlr`]
module"]
pub type CTLR = crate::Reg<ctlr::CTLR_SPEC>;
#[doc = "Control register (DAC_CR)"]
pub mod ctlr;
#[doc = "SWTR (w) register accessor: DAC software trigger register (DAC_SWTRIGR)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swtr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swtr`]
module"]
pub type SWTR = crate::Reg<swtr::SWTR_SPEC>;
#[doc = "DAC software trigger register (DAC_SWTRIGR)"]
pub mod swtr;
#[doc = "R12BDHR1 (rw) register accessor: DAC channel1 12-bit right-aligned data holding register(DAC_DHR12R1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r12bdhr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r12bdhr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r12bdhr1`]
module"]
pub type R12BDHR1 = crate::Reg<r12bdhr1::R12BDHR1_SPEC>;
#[doc = "DAC channel1 12-bit right-aligned data holding register(DAC_DHR12R1)"]
pub mod r12bdhr1;
#[doc = "L12BDHR1 (rw) register accessor: DAC channel1 12-bit left aligned data holding register (DAC_DHR12L1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l12bdhr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l12bdhr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l12bdhr1`]
module"]
pub type L12BDHR1 = crate::Reg<l12bdhr1::L12BDHR1_SPEC>;
#[doc = "DAC channel1 12-bit left aligned data holding register (DAC_DHR12L1)"]
pub mod l12bdhr1;
#[doc = "R8BDHR1 (rw) register accessor: DAC channel1 8-bit right aligned data holding register (DAC_DHR8R1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8bdhr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8bdhr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8bdhr1`]
module"]
pub type R8BDHR1 = crate::Reg<r8bdhr1::R8BDHR1_SPEC>;
#[doc = "DAC channel1 8-bit right aligned data holding register (DAC_DHR8R1)"]
pub mod r8bdhr1;
#[doc = "R12BDHR2 (rw) register accessor: DAC channel2 12-bit right aligned data holding register (DAC_DHR12R2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r12bdhr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r12bdhr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r12bdhr2`]
module"]
pub type R12BDHR2 = crate::Reg<r12bdhr2::R12BDHR2_SPEC>;
#[doc = "DAC channel2 12-bit right aligned data holding register (DAC_DHR12R2)"]
pub mod r12bdhr2;
#[doc = "L12BDHR2 (rw) register accessor: DAC channel2 12-bit left aligned data holding register (DAC_DHR12L2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l12bdhr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l12bdhr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l12bdhr2`]
module"]
pub type L12BDHR2 = crate::Reg<l12bdhr2::L12BDHR2_SPEC>;
#[doc = "DAC channel2 12-bit left aligned data holding register (DAC_DHR12L2)"]
pub mod l12bdhr2;
#[doc = "R8BDHR2 (rw) register accessor: DAC channel2 8-bit right-aligned data holding register (DAC_DHR8R2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8bdhr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8bdhr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8bdhr2`]
module"]
pub type R8BDHR2 = crate::Reg<r8bdhr2::R8BDHR2_SPEC>;
#[doc = "DAC channel2 8-bit right-aligned data holding register (DAC_DHR8R2)"]
pub mod r8bdhr2;
#[doc = "RD12BDHR (rw) register accessor: Dual DAC 12-bit right-aligned data holding register (DAC_DHR12RD), Bits 31:28 Reserved, Bits 15:12 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd12bdhr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd12bdhr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd12bdhr`]
module"]
pub type RD12BDHR = crate::Reg<rd12bdhr::RD12BDHR_SPEC>;
#[doc = "Dual DAC 12-bit right-aligned data holding register (DAC_DHR12RD), Bits 31:28 Reserved, Bits 15:12 Reserved"]
pub mod rd12bdhr;
#[doc = "LD12BDHR (rw) register accessor: DUAL DAC 12-bit left aligned data holding register (DAC_DHR12LD), Bits 19:16 Reserved, Bits 3:0 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ld12bdhr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ld12bdhr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ld12bdhr`]
module"]
pub type LD12BDHR = crate::Reg<ld12bdhr::LD12BDHR_SPEC>;
#[doc = "DUAL DAC 12-bit left aligned data holding register (DAC_DHR12LD), Bits 19:16 Reserved, Bits 3:0 Reserved"]
pub mod ld12bdhr;
#[doc = "RD8BDHR (rw) register accessor: DUAL DAC 8-bit right aligned data holding register (DAC_DHR8RD), Bits 31:16 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd8bdhr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd8bdhr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd8bdhr`]
module"]
pub type RD8BDHR = crate::Reg<rd8bdhr::RD8BDHR_SPEC>;
#[doc = "DUAL DAC 8-bit right aligned data holding register (DAC_DHR8RD), Bits 31:16 Reserved"]
pub mod rd8bdhr;
#[doc = "DOR1 (r) register accessor: DAC channel1 data output register (DAC_DOR1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dor1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dor1`]
module"]
pub type DOR1 = crate::Reg<dor1::DOR1_SPEC>;
#[doc = "DAC channel1 data output register (DAC_DOR1)"]
pub mod dor1;
#[doc = "DOR2 (r) register accessor: DAC channel2 data output register (DAC_DOR2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dor2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dor2`]
module"]
pub type DOR2 = crate::Reg<dor2::DOR2_SPEC>;
#[doc = "DAC channel2 data output register (DAC_DOR2)"]
pub mod dor2;
