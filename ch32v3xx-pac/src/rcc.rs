#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    pub ctlr: CTLR,
    #[doc = "0x04 - Clock configuration register (RCC_CFGR0)"]
    pub cfgr0: CFGR0,
    #[doc = "0x08 - Clock interrupt register (RCC_INTR)"]
    pub intr: INTR,
    #[doc = "0x0c - APB2 peripheral reset register (RCC_APB2PRSTR)"]
    pub apb2prstr: APB2PRSTR,
    #[doc = "0x10 - APB1 peripheral reset register (RCC_APB1PRSTR)"]
    pub apb1prstr: APB1PRSTR,
    #[doc = "0x14 - AHB Peripheral Clock enable register (RCC_AHBPCENR)"]
    pub ahbpcenr: AHBPCENR,
    #[doc = "0x18 - APB2 peripheral clock enable register (RCC_APB2PCENR)"]
    pub apb2pcenr: APB2PCENR,
    #[doc = "0x1c - APB1 peripheral clock enable register (RCC_APB1PCENR)"]
    pub apb1pcenr: APB1PCENR,
    #[doc = "0x20 - Backup domain control register (RCC_BDCTLR)"]
    pub bdctlr: BDCTLR,
    #[doc = "0x24 - Control/status register (RCC_RSTSCKR)"]
    pub rstsckr: RSTSCKR,
    #[doc = "0x28 - AHB reset register (RCC_APHBRSTR)"]
    pub ahbrstr: AHBRSTR,
    #[doc = "0x2c - Clock configuration register2 (RCC_CFGR2)"]
    pub cfgr2: CFGR2,
}
#[doc = "CTLR (rw) register accessor: Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlr`]
module"]
pub type CTLR = crate::Reg<ctlr::CTLR_SPEC>;
#[doc = "Clock control register"]
pub mod ctlr;
#[doc = "CFGR0 (rw) register accessor: Clock configuration register (RCC_CFGR0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr0`]
module"]
pub type CFGR0 = crate::Reg<cfgr0::CFGR0_SPEC>;
#[doc = "Clock configuration register (RCC_CFGR0)"]
pub mod cfgr0;
#[doc = "INTR (rw) register accessor: Clock interrupt register (RCC_INTR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr`]
module"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Clock interrupt register (RCC_INTR)"]
pub mod intr;
#[doc = "APB2PRSTR (rw) register accessor: APB2 peripheral reset register (RCC_APB2PRSTR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2prstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2prstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2prstr`]
module"]
pub type APB2PRSTR = crate::Reg<apb2prstr::APB2PRSTR_SPEC>;
#[doc = "APB2 peripheral reset register (RCC_APB2PRSTR)"]
pub mod apb2prstr;
#[doc = "APB1PRSTR (rw) register accessor: APB1 peripheral reset register (RCC_APB1PRSTR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1prstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1prstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1prstr`]
module"]
pub type APB1PRSTR = crate::Reg<apb1prstr::APB1PRSTR_SPEC>;
#[doc = "APB1 peripheral reset register (RCC_APB1PRSTR)"]
pub mod apb1prstr;
#[doc = "AHBPCENR (rw) register accessor: AHB Peripheral Clock enable register (RCC_AHBPCENR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbpcenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbpcenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbpcenr`]
module"]
pub type AHBPCENR = crate::Reg<ahbpcenr::AHBPCENR_SPEC>;
#[doc = "AHB Peripheral Clock enable register (RCC_AHBPCENR)"]
pub mod ahbpcenr;
#[doc = "APB2PCENR (rw) register accessor: APB2 peripheral clock enable register (RCC_APB2PCENR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2pcenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2pcenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2pcenr`]
module"]
pub type APB2PCENR = crate::Reg<apb2pcenr::APB2PCENR_SPEC>;
#[doc = "APB2 peripheral clock enable register (RCC_APB2PCENR)"]
pub mod apb2pcenr;
#[doc = "APB1PCENR (rw) register accessor: APB1 peripheral clock enable register (RCC_APB1PCENR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1pcenr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1pcenr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1pcenr`]
module"]
pub type APB1PCENR = crate::Reg<apb1pcenr::APB1PCENR_SPEC>;
#[doc = "APB1 peripheral clock enable register (RCC_APB1PCENR)"]
pub mod apb1pcenr;
#[doc = "BDCTLR (rw) register accessor: Backup domain control register (RCC_BDCTLR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdctlr`]
module"]
pub type BDCTLR = crate::Reg<bdctlr::BDCTLR_SPEC>;
#[doc = "Backup domain control register (RCC_BDCTLR)"]
pub mod bdctlr;
#[doc = "RSTSCKR (rw) register accessor: Control/status register (RCC_RSTSCKR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstsckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstsckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstsckr`]
module"]
pub type RSTSCKR = crate::Reg<rstsckr::RSTSCKR_SPEC>;
#[doc = "Control/status register (RCC_RSTSCKR)"]
pub mod rstsckr;
#[doc = "AHBRSTR (rw) register accessor: AHB reset register (RCC_APHBRSTR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrstr`]
module"]
pub type AHBRSTR = crate::Reg<ahbrstr::AHBRSTR_SPEC>;
#[doc = "AHB reset register (RCC_APHBRSTR)"]
pub mod ahbrstr;
#[doc = "CFGR2 (rw) register accessor: Clock configuration register2 (RCC_CFGR2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`]
module"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "Clock configuration register2 (RCC_CFGR2)"]
pub mod cfgr2;
