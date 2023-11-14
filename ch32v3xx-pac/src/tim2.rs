#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub ctlr1: CTLR1,
    #[doc = "0x04 - control register 2"]
    pub ctlr2: CTLR2,
    #[doc = "0x08 - slave mode control register"]
    pub smcfgr: SMCFGR,
    #[doc = "0x0c - DMA/Interrupt enable register"]
    pub dmaintenr: DMAINTENR,
    #[doc = "0x10 - status register"]
    pub intfr: INTFR,
    #[doc = "0x14 - event generation register"]
    pub swevgr: SWEVGR,
    _reserved_6_chctlr1: [u8; 0x04],
    _reserved_7_chctlr2: [u8; 0x04],
    #[doc = "0x20 - capture/compare enable register"]
    pub ccer: CCER,
    #[doc = "0x24 - counter"]
    pub cnt: CNT,
    #[doc = "0x28 - prescaler"]
    pub psc: PSC,
    #[doc = "0x2c - auto-reload register"]
    pub atrlr: ATRLR,
    _reserved12: [u8; 0x04],
    #[doc = "0x34 - capture/compare register 1"]
    pub ch1cvr: CH1CVR,
    #[doc = "0x38 - capture/compare register 2"]
    pub ch2cvr: CH2CVR,
    #[doc = "0x3c - capture/compare register 3"]
    pub ch3cvr: CH3CVR,
    #[doc = "0x40 - capture/compare register 4"]
    pub ch4cvr: CH4CVR,
    _reserved16: [u8; 0x04],
    #[doc = "0x48 - DMA control register"]
    pub dmacfgr: DMACFGR,
    #[doc = "0x4c - DMA address for full transfer"]
    pub dmaadr: DMAADR,
}
impl RegisterBlock {
    #[doc = "0x18 - capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub const fn chctlr1_input(&self) -> &CHCTLR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub const fn chctlr1_output(&self) -> &CHCTLR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x1c - capture/compare mode register 2 (input mode)"]
    #[inline(always)]
    pub const fn chctlr2_input(&self) -> &CHCTLR2_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - capture/compare mode register 2 (output mode)"]
    #[inline(always)]
    pub const fn chctlr2_output(&self) -> &CHCTLR2_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
}
#[doc = "CTLR1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlr1`]
module"]
pub type CTLR1 = crate::Reg<ctlr1::CTLR1_SPEC>;
#[doc = "control register 1"]
pub mod ctlr1;
#[doc = "CTLR2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlr2`]
module"]
pub type CTLR2 = crate::Reg<ctlr2::CTLR2_SPEC>;
#[doc = "control register 2"]
pub mod ctlr2;
#[doc = "SMCFGR (rw) register accessor: slave mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smcfgr`]
module"]
pub type SMCFGR = crate::Reg<smcfgr::SMCFGR_SPEC>;
#[doc = "slave mode control register"]
pub mod smcfgr;
#[doc = "DMAINTENR (rw) register accessor: DMA/Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaintenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaintenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaintenr`]
module"]
pub type DMAINTENR = crate::Reg<dmaintenr::DMAINTENR_SPEC>;
#[doc = "DMA/Interrupt enable register"]
pub mod dmaintenr;
#[doc = "INTFR (rw) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intfr`]
module"]
pub type INTFR = crate::Reg<intfr::INTFR_SPEC>;
#[doc = "status register"]
pub mod intfr;
#[doc = "SWEVGR (w) register accessor: event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swevgr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swevgr`]
module"]
pub type SWEVGR = crate::Reg<swevgr::SWEVGR_SPEC>;
#[doc = "event generation register"]
pub mod swevgr;
#[doc = "CHCTLR1_Output (rw) register accessor: capture/compare mode register 1 (output mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctlr1_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctlr1_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctlr1_output`]
module"]
pub type CHCTLR1_OUTPUT = crate::Reg<chctlr1_output::CHCTLR1_OUTPUT_SPEC>;
#[doc = "capture/compare mode register 1 (output mode)"]
pub mod chctlr1_output;
#[doc = "CHCTLR1_Input (rw) register accessor: capture/compare mode register 1 (input mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctlr1_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctlr1_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctlr1_input`]
module"]
pub type CHCTLR1_INPUT = crate::Reg<chctlr1_input::CHCTLR1_INPUT_SPEC>;
#[doc = "capture/compare mode register 1 (input mode)"]
pub mod chctlr1_input;
#[doc = "CHCTLR2_Output (rw) register accessor: capture/compare mode register 2 (output mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctlr2_output::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctlr2_output::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctlr2_output`]
module"]
pub type CHCTLR2_OUTPUT = crate::Reg<chctlr2_output::CHCTLR2_OUTPUT_SPEC>;
#[doc = "capture/compare mode register 2 (output mode)"]
pub mod chctlr2_output;
#[doc = "CHCTLR2_Input (rw) register accessor: capture/compare mode register 2 (input mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctlr2_input::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctlr2_input::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctlr2_input`]
module"]
pub type CHCTLR2_INPUT = crate::Reg<chctlr2_input::CHCTLR2_INPUT_SPEC>;
#[doc = "capture/compare mode register 2 (input mode)"]
pub mod chctlr2_input;
#[doc = "CCER (rw) register accessor: capture/compare enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccer`]
module"]
pub type CCER = crate::Reg<ccer::CCER_SPEC>;
#[doc = "capture/compare enable register"]
pub mod ccer;
#[doc = "CNT (rw) register accessor: counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "counter"]
pub mod cnt;
#[doc = "PSC (rw) register accessor: prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psc`]
module"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "prescaler"]
pub mod psc;
#[doc = "ATRLR (rw) register accessor: auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atrlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atrlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atrlr`]
module"]
pub type ATRLR = crate::Reg<atrlr::ATRLR_SPEC>;
#[doc = "auto-reload register"]
pub mod atrlr;
#[doc = "CH1CVR (rw) register accessor: capture/compare register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1cvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1cvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cvr`]
module"]
pub type CH1CVR = crate::Reg<ch1cvr::CH1CVR_SPEC>;
#[doc = "capture/compare register 1"]
pub mod ch1cvr;
#[doc = "CH2CVR (rw) register accessor: capture/compare register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2cvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2cvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cvr`]
module"]
pub type CH2CVR = crate::Reg<ch2cvr::CH2CVR_SPEC>;
#[doc = "capture/compare register 2"]
pub mod ch2cvr;
#[doc = "CH3CVR (rw) register accessor: capture/compare register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3cvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3cvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cvr`]
module"]
pub type CH3CVR = crate::Reg<ch3cvr::CH3CVR_SPEC>;
#[doc = "capture/compare register 3"]
pub mod ch3cvr;
#[doc = "CH4CVR (rw) register accessor: capture/compare register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4cvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4cvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cvr`]
module"]
pub type CH4CVR = crate::Reg<ch4cvr::CH4CVR_SPEC>;
#[doc = "capture/compare register 4"]
pub mod ch4cvr;
#[doc = "DMACFGR (rw) register accessor: DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacfgr`]
module"]
pub type DMACFGR = crate::Reg<dmacfgr::DMACFGR_SPEC>;
#[doc = "DMA control register"]
pub mod dmacfgr;
#[doc = "DMAADR (rw) register accessor: DMA address for full transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaadr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaadr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaadr`]
module"]
pub type DMAADR = crate::Reg<dmaadr::DMAADR_SPEC>;
#[doc = "DMA address for full transfer"]
pub mod dmaadr;
