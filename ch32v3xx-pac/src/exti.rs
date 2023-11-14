#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt mask register (EXTI_INTENR)"]
    pub intenr: INTENR,
    #[doc = "0x04 - Event mask register (EXTI_EVENR)"]
    pub evenr: EVENR,
    #[doc = "0x08 - Rising Trigger selection register (EXTI_RTENR)"]
    pub rtenr: RTENR,
    #[doc = "0x0c - Falling Trigger selection register (EXTI_FTENR)"]
    pub ftenr: FTENR,
    #[doc = "0x10 - Software interrupt event register (EXTI_SWIEVR)"]
    pub swievr: SWIEVR,
    #[doc = "0x14 - Pending register (EXTI_INTFR)"]
    pub intfr: INTFR,
}
#[doc = "INTENR (rw) register accessor: Interrupt mask register (EXTI_INTENR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenr`]
module"]
pub type INTENR = crate::Reg<intenr::INTENR_SPEC>;
#[doc = "Interrupt mask register (EXTI_INTENR)"]
pub mod intenr;
#[doc = "EVENR (rw) register accessor: Event mask register (EXTI_EVENR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evenr`]
module"]
pub type EVENR = crate::Reg<evenr::EVENR_SPEC>;
#[doc = "Event mask register (EXTI_EVENR)"]
pub mod evenr;
#[doc = "RTENR (rw) register accessor: Rising Trigger selection register (EXTI_RTENR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtenr`]
module"]
pub type RTENR = crate::Reg<rtenr::RTENR_SPEC>;
#[doc = "Rising Trigger selection register (EXTI_RTENR)"]
pub mod rtenr;
#[doc = "FTENR (rw) register accessor: Falling Trigger selection register (EXTI_FTENR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftenr`]
module"]
pub type FTENR = crate::Reg<ftenr::FTENR_SPEC>;
#[doc = "Falling Trigger selection register (EXTI_FTENR)"]
pub mod ftenr;
#[doc = "SWIEVR (rw) register accessor: Software interrupt event register (EXTI_SWIEVR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swievr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swievr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swievr`]
module"]
pub type SWIEVR = crate::Reg<swievr::SWIEVR_SPEC>;
#[doc = "Software interrupt event register (EXTI_SWIEVR)"]
pub mod swievr;
#[doc = "INTFR (rw) register accessor: Pending register (EXTI_INTFR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intfr`]
module"]
pub type INTFR = crate::Reg<intfr::INTFR_SPEC>;
#[doc = "Pending register (EXTI_INTFR)"]
pub mod intfr;
