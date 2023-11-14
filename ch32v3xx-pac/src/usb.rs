#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - endpoint 0 register"]
    pub ep0r: EP0R,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - endpoint 1 register"]
    pub ep1r: EP1R,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - endpoint 2 register"]
    pub ep2r: EP2R,
    _reserved3: [u8; 0x02],
    #[doc = "0x0c - endpoint 3 register"]
    pub ep3r: EP3R,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - endpoint 4 register"]
    pub ep4r: EP4R,
    _reserved5: [u8; 0x02],
    #[doc = "0x14 - endpoint 5 register"]
    pub ep5r: EP5R,
    _reserved6: [u8; 0x02],
    #[doc = "0x18 - endpoint 6 register"]
    pub ep6r: EP6R,
    _reserved7: [u8; 0x02],
    #[doc = "0x1c - endpoint 7 register"]
    pub ep7r: EP7R,
    _reserved8: [u8; 0x22],
    #[doc = "0x40 - control register"]
    pub cntr: CNTR,
    _reserved9: [u8; 0x02],
    #[doc = "0x44 - interrupt status register"]
    pub istr: ISTR,
    _reserved10: [u8; 0x02],
    #[doc = "0x48 - frame number register"]
    pub fnr: FNR,
    _reserved11: [u8; 0x02],
    #[doc = "0x4c - device address"]
    pub daddr: DADDR,
    _reserved12: [u8; 0x02],
    #[doc = "0x50 - Buffer table address"]
    pub btable: BTABLE,
}
#[doc = "EP0R (rw) register accessor: endpoint 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0r`]
module"]
pub type EP0R = crate::Reg<ep0r::EP0R_SPEC>;
#[doc = "endpoint 0 register"]
pub mod ep0r;
#[doc = "EP1R (rw) register accessor: endpoint 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep1r`]
module"]
pub type EP1R = crate::Reg<ep1r::EP1R_SPEC>;
#[doc = "endpoint 1 register"]
pub mod ep1r;
#[doc = "EP2R (rw) register accessor: endpoint 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep2r`]
module"]
pub type EP2R = crate::Reg<ep2r::EP2R_SPEC>;
#[doc = "endpoint 2 register"]
pub mod ep2r;
#[doc = "EP3R (rw) register accessor: endpoint 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep3r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep3r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep3r`]
module"]
pub type EP3R = crate::Reg<ep3r::EP3R_SPEC>;
#[doc = "endpoint 3 register"]
pub mod ep3r;
#[doc = "EP4R (rw) register accessor: endpoint 4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep4r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep4r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep4r`]
module"]
pub type EP4R = crate::Reg<ep4r::EP4R_SPEC>;
#[doc = "endpoint 4 register"]
pub mod ep4r;
#[doc = "EP5R (rw) register accessor: endpoint 5 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep5r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep5r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep5r`]
module"]
pub type EP5R = crate::Reg<ep5r::EP5R_SPEC>;
#[doc = "endpoint 5 register"]
pub mod ep5r;
#[doc = "EP6R (rw) register accessor: endpoint 6 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep6r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep6r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep6r`]
module"]
pub type EP6R = crate::Reg<ep6r::EP6R_SPEC>;
#[doc = "endpoint 6 register"]
pub mod ep6r;
#[doc = "EP7R (rw) register accessor: endpoint 7 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep7r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep7r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep7r`]
module"]
pub type EP7R = crate::Reg<ep7r::EP7R_SPEC>;
#[doc = "endpoint 7 register"]
pub mod ep7r;
#[doc = "CNTR (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr`]
module"]
pub type CNTR = crate::Reg<cntr::CNTR_SPEC>;
#[doc = "control register"]
pub mod cntr;
#[doc = "ISTR (rw) register accessor: interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`istr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`istr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@istr`]
module"]
pub type ISTR = crate::Reg<istr::ISTR_SPEC>;
#[doc = "interrupt status register"]
pub mod istr;
#[doc = "FNR (r) register accessor: frame number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fnr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fnr`]
module"]
pub type FNR = crate::Reg<fnr::FNR_SPEC>;
#[doc = "frame number register"]
pub mod fnr;
#[doc = "DADDR (rw) register accessor: device address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daddr`]
module"]
pub type DADDR = crate::Reg<daddr::DADDR_SPEC>;
#[doc = "device address"]
pub mod daddr;
#[doc = "BTABLE (rw) register accessor: Buffer table address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btable`]
module"]
pub type BTABLE = crate::Reg<btable::BTABLE_SPEC>;
#[doc = "Buffer table address"]
pub mod btable;
