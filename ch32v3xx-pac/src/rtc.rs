#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC Control Register High"]
    pub ctlrh: CTLRH,
    #[doc = "0x04 - RTC Control Register Low"]
    pub ctlrl: CTLRL,
    #[doc = "0x08 - RTC Prescaler Load Register High"]
    pub pscrh: PSCRH,
    #[doc = "0x0c - RTC Prescaler Load Register Low"]
    pub pscrl: PSCRL,
    #[doc = "0x10 - RTC Prescaler Divider Register High"]
    pub divh: DIVH,
    #[doc = "0x14 - RTC Prescaler Divider Register Low"]
    pub divl: DIVL,
    #[doc = "0x18 - RTC Counter Register High"]
    pub cnth: CNTH,
    #[doc = "0x1c - RTC Counter Register Low"]
    pub cntl: CNTL,
    #[doc = "0x20 - RTC Alarm Register High"]
    pub alrmh: ALRMH,
    #[doc = "0x24 - RTC Alarm Register Low"]
    pub alrml: ALRML,
}
#[doc = "CTLRH (rw) register accessor: RTC Control Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlrh`]
module"]
pub type CTLRH = crate::Reg<ctlrh::CTLRH_SPEC>;
#[doc = "RTC Control Register High"]
pub mod ctlrh;
#[doc = "CTLRL (rw) register accessor: RTC Control Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlrl`]
module"]
pub type CTLRL = crate::Reg<ctlrl::CTLRL_SPEC>;
#[doc = "RTC Control Register Low"]
pub mod ctlrl;
#[doc = "PSCRH (w) register accessor: RTC Prescaler Load Register High\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pscrh::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscrh`]
module"]
pub type PSCRH = crate::Reg<pscrh::PSCRH_SPEC>;
#[doc = "RTC Prescaler Load Register High"]
pub mod pscrh;
#[doc = "PSCRL (w) register accessor: RTC Prescaler Load Register Low\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pscrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscrl`]
module"]
pub type PSCRL = crate::Reg<pscrl::PSCRL_SPEC>;
#[doc = "RTC Prescaler Load Register Low"]
pub mod pscrl;
#[doc = "DIVH (r) register accessor: RTC Prescaler Divider Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divh::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divh`]
module"]
pub type DIVH = crate::Reg<divh::DIVH_SPEC>;
#[doc = "RTC Prescaler Divider Register High"]
pub mod divh;
#[doc = "DIVL (r) register accessor: RTC Prescaler Divider Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divl`]
module"]
pub type DIVL = crate::Reg<divl::DIVL_SPEC>;
#[doc = "RTC Prescaler Divider Register Low"]
pub mod divl;
#[doc = "CNTH (rw) register accessor: RTC Counter Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnth`]
module"]
pub type CNTH = crate::Reg<cnth::CNTH_SPEC>;
#[doc = "RTC Counter Register High"]
pub mod cnth;
#[doc = "CNTL (rw) register accessor: RTC Counter Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntl`]
module"]
pub type CNTL = crate::Reg<cntl::CNTL_SPEC>;
#[doc = "RTC Counter Register Low"]
pub mod cntl;
#[doc = "ALRMH (w) register accessor: RTC Alarm Register High\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrmh::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmh`]
module"]
pub type ALRMH = crate::Reg<alrmh::ALRMH_SPEC>;
#[doc = "RTC Alarm Register High"]
pub mod alrmh;
#[doc = "ALRML (w) register accessor: RTC Alarm Register Low\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrml::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrml`]
module"]
pub type ALRML = crate::Reg<alrml::ALRML_SPEC>;
#[doc = "RTC Alarm Register Low"]
pub mod alrml;
