#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - status register"]
    pub statr: STATR,
    #[doc = "0x04 - control register 1/TKEY_V_CTLR"]
    pub ctlr1: CTLR1,
    #[doc = "0x08 - control register 2"]
    pub ctlr2: CTLR2,
    #[doc = "0x0c - sample time register 1"]
    pub samptr1_charge1: SAMPTR1_CHARGE1,
    #[doc = "0x10 - sample time register 2"]
    pub samptr2_charge2: SAMPTR2_CHARGE2,
    #[doc = "0x14 - injected channel data offset register x"]
    pub iofr1: IOFR1,
    #[doc = "0x18 - injected channel data offset register x"]
    pub iofr2: IOFR2,
    #[doc = "0x1c - injected channel data offset register x"]
    pub iofr3: IOFR3,
    #[doc = "0x20 - injected channel data offset register x"]
    pub iofr4: IOFR4,
    #[doc = "0x24 - watchdog higher threshold register"]
    pub wdhtr: WDHTR,
    #[doc = "0x28 - watchdog lower threshold register"]
    pub wdltr: WDLTR,
    #[doc = "0x2c - regular sequence register 1"]
    pub rsqr1: RSQR1,
    #[doc = "0x30 - regular sequence register 2"]
    pub rsqr2: RSQR2,
    #[doc = "0x34 - regular sequence register 3;TKEY_V_CHANNEL"]
    pub rsqr3__channel: RSQR3__CHANNEL,
    #[doc = "0x38 - injected sequence register"]
    pub isqr: ISQR,
    #[doc = "0x3c - injected data register x_Charge data offset for injected channel x"]
    pub idatar1_chgoffset: IDATAR1_CHGOFFSET,
    #[doc = "0x40 - injected data register x"]
    pub idatar2: IDATAR2,
    #[doc = "0x44 - injected data register x"]
    pub idatar3: IDATAR3,
    #[doc = "0x48 - injected data register x"]
    pub idatar4: IDATAR4,
    #[doc = "0x4c - regular data register_start and discharge time register"]
    pub rdatar_dr_act_dcg: RDATAR_DR_ACT_DCG,
}
#[doc = "STATR (rw) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`statr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statr`]
module"]
pub type STATR = crate::Reg<statr::STATR_SPEC>;
#[doc = "status register"]
pub mod statr;
#[doc = "CTLR1 (rw) register accessor: control register 1/TKEY_V_CTLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlr1`]
module"]
pub type CTLR1 = crate::Reg<ctlr1::CTLR1_SPEC>;
#[doc = "control register 1/TKEY_V_CTLR"]
pub mod ctlr1;
#[doc = "CTLR2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlr2`]
module"]
pub type CTLR2 = crate::Reg<ctlr2::CTLR2_SPEC>;
#[doc = "control register 2"]
pub mod ctlr2;
#[doc = "SAMPTR1_CHARGE1 (rw) register accessor: sample time register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`samptr1_charge1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`samptr1_charge1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@samptr1_charge1`]
module"]
pub type SAMPTR1_CHARGE1 = crate::Reg<samptr1_charge1::SAMPTR1_CHARGE1_SPEC>;
#[doc = "sample time register 1"]
pub mod samptr1_charge1;
#[doc = "SAMPTR2_CHARGE2 (rw) register accessor: sample time register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`samptr2_charge2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`samptr2_charge2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@samptr2_charge2`]
module"]
pub type SAMPTR2_CHARGE2 = crate::Reg<samptr2_charge2::SAMPTR2_CHARGE2_SPEC>;
#[doc = "sample time register 2"]
pub mod samptr2_charge2;
#[doc = "IOFR1 (rw) register accessor: injected channel data offset register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iofr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iofr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iofr1`]
module"]
pub type IOFR1 = crate::Reg<iofr1::IOFR1_SPEC>;
#[doc = "injected channel data offset register x"]
pub mod iofr1;
#[doc = "IOFR2 (rw) register accessor: injected channel data offset register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iofr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iofr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iofr2`]
module"]
pub type IOFR2 = crate::Reg<iofr2::IOFR2_SPEC>;
#[doc = "injected channel data offset register x"]
pub mod iofr2;
#[doc = "IOFR3 (rw) register accessor: injected channel data offset register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iofr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iofr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iofr3`]
module"]
pub type IOFR3 = crate::Reg<iofr3::IOFR3_SPEC>;
#[doc = "injected channel data offset register x"]
pub mod iofr3;
#[doc = "IOFR4 (rw) register accessor: injected channel data offset register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iofr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iofr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iofr4`]
module"]
pub type IOFR4 = crate::Reg<iofr4::IOFR4_SPEC>;
#[doc = "injected channel data offset register x"]
pub mod iofr4;
#[doc = "WDHTR (rw) register accessor: watchdog higher threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdhtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdhtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdhtr`]
module"]
pub type WDHTR = crate::Reg<wdhtr::WDHTR_SPEC>;
#[doc = "watchdog higher threshold register"]
pub mod wdhtr;
#[doc = "WDLTR (rw) register accessor: watchdog lower threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdltr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdltr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdltr`]
module"]
pub type WDLTR = crate::Reg<wdltr::WDLTR_SPEC>;
#[doc = "watchdog lower threshold register"]
pub mod wdltr;
#[doc = "RSQR1 (rw) register accessor: regular sequence register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsqr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsqr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsqr1`]
module"]
pub type RSQR1 = crate::Reg<rsqr1::RSQR1_SPEC>;
#[doc = "regular sequence register 1"]
pub mod rsqr1;
#[doc = "RSQR2 (rw) register accessor: regular sequence register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsqr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsqr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsqr2`]
module"]
pub type RSQR2 = crate::Reg<rsqr2::RSQR2_SPEC>;
#[doc = "regular sequence register 2"]
pub mod rsqr2;
#[doc = "RSQR3__CHANNEL (rw) register accessor: regular sequence register 3;TKEY_V_CHANNEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsqr3__channel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsqr3__channel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsqr3__channel`]
module"]
pub type RSQR3__CHANNEL = crate::Reg<rsqr3__channel::RSQR3__CHANNEL_SPEC>;
#[doc = "regular sequence register 3;TKEY_V_CHANNEL"]
pub mod rsqr3__channel;
#[doc = "ISQR (rw) register accessor: injected sequence register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isqr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isqr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isqr`]
module"]
pub type ISQR = crate::Reg<isqr::ISQR_SPEC>;
#[doc = "injected sequence register"]
pub mod isqr;
#[doc = "IDATAR1_CHGOFFSET (r) register accessor: injected data register x_Charge data offset for injected channel x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idatar1_chgoffset::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idatar1_chgoffset`]
module"]
pub type IDATAR1_CHGOFFSET = crate::Reg<idatar1_chgoffset::IDATAR1_CHGOFFSET_SPEC>;
#[doc = "injected data register x_Charge data offset for injected channel x"]
pub mod idatar1_chgoffset;
#[doc = "IDATAR2 (r) register accessor: injected data register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idatar2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idatar2`]
module"]
pub type IDATAR2 = crate::Reg<idatar2::IDATAR2_SPEC>;
#[doc = "injected data register x"]
pub mod idatar2;
#[doc = "IDATAR3 (r) register accessor: injected data register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idatar3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idatar3`]
module"]
pub type IDATAR3 = crate::Reg<idatar3::IDATAR3_SPEC>;
#[doc = "injected data register x"]
pub mod idatar3;
#[doc = "IDATAR4 (r) register accessor: injected data register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idatar4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idatar4`]
module"]
pub type IDATAR4 = crate::Reg<idatar4::IDATAR4_SPEC>;
#[doc = "injected data register x"]
pub mod idatar4;
#[doc = "RDATAR_DR_ACT_DCG (rw) register accessor: regular data register_start and discharge time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdatar_dr_act_dcg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdatar_dr_act_dcg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdatar_dr_act_dcg`]
module"]
pub type RDATAR_DR_ACT_DCG = crate::Reg<rdatar_dr_act_dcg::RDATAR_DR_ACT_DCG_SPEC>;
#[doc = "regular data register_start and discharge time register"]
pub mod rdatar_dr_act_dcg;
