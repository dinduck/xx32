#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet MMC control register (ETH_MMCCR)"]
    pub mmccr: MMCCR,
    #[doc = "0x04 - Ethernet MMC receive interrupt register (ETH_MMCRIR)"]
    pub mmcrir: MMCRIR,
    #[doc = "0x08 - Ethernet MMC transmit interrupt register (ETH_MMCTIR)"]
    pub mmctir: MMCTIR,
    #[doc = "0x0c - Ethernet MMC receive interrupt mask register (ETH_MMCRIMR)"]
    pub mmcrimr: MMCRIMR,
    #[doc = "0x10 - Ethernet MMC transmit interrupt mask register (ETH_MMCTIMR)"]
    pub mmctimr: MMCTIMR,
    _reserved5: [u8; 0x38],
    #[doc = "0x4c - Ethernet MMC transmitted good frames after a single collision counter"]
    pub mmctgfsccr: MMCTGFSCCR,
    #[doc = "0x50 - Ethernet MMC transmitted good frames after more than a single collision"]
    pub mmctgfmsccr: MMCTGFMSCCR,
    _reserved7: [u8; 0x14],
    #[doc = "0x68 - Ethernet MMC transmitted good frames counter register"]
    pub mmctgfcr: MMCTGFCR,
    _reserved8: [u8; 0x28],
    #[doc = "0x94 - Ethernet MMC received frames with CRC error counter register"]
    pub mmcrfcecr: MMCRFCECR,
    #[doc = "0x98 - Ethernet MMC received frames with alignment error counter register"]
    pub mmcrfaecr: MMCRFAECR,
    _reserved10: [u8; 0x28],
    #[doc = "0xc4 - MMC received good unicast frames counter register"]
    pub mmcrgufcr: MMCRGUFCR,
}
#[doc = "MMCCR (rw) register accessor: Ethernet MMC control register (ETH_MMCCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmccr`]
module"]
pub type MMCCR = crate::Reg<mmccr::MMCCR_SPEC>;
#[doc = "Ethernet MMC control register (ETH_MMCCR)"]
pub mod mmccr;
#[doc = "MMCRIR (rw) register accessor: Ethernet MMC receive interrupt register (ETH_MMCRIR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmcrir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmcrir`]
module"]
pub type MMCRIR = crate::Reg<mmcrir::MMCRIR_SPEC>;
#[doc = "Ethernet MMC receive interrupt register (ETH_MMCRIR)"]
pub mod mmcrir;
#[doc = "MMCTIR (rw) register accessor: Ethernet MMC transmit interrupt register (ETH_MMCTIR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmctir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmctir`]
module"]
pub type MMCTIR = crate::Reg<mmctir::MMCTIR_SPEC>;
#[doc = "Ethernet MMC transmit interrupt register (ETH_MMCTIR)"]
pub mod mmctir;
#[doc = "MMCRIMR (rw) register accessor: Ethernet MMC receive interrupt mask register (ETH_MMCRIMR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrimr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmcrimr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmcrimr`]
module"]
pub type MMCRIMR = crate::Reg<mmcrimr::MMCRIMR_SPEC>;
#[doc = "Ethernet MMC receive interrupt mask register (ETH_MMCRIMR)"]
pub mod mmcrimr;
#[doc = "MMCTIMR (rw) register accessor: Ethernet MMC transmit interrupt mask register (ETH_MMCTIMR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctimr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmctimr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmctimr`]
module"]
pub type MMCTIMR = crate::Reg<mmctimr::MMCTIMR_SPEC>;
#[doc = "Ethernet MMC transmit interrupt mask register (ETH_MMCTIMR)"]
pub mod mmctimr;
#[doc = "MMCTGFSCCR (r) register accessor: Ethernet MMC transmitted good frames after a single collision counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctgfsccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmctgfsccr`]
module"]
pub type MMCTGFSCCR = crate::Reg<mmctgfsccr::MMCTGFSCCR_SPEC>;
#[doc = "Ethernet MMC transmitted good frames after a single collision counter"]
pub mod mmctgfsccr;
#[doc = "MMCTGFMSCCR (r) register accessor: Ethernet MMC transmitted good frames after more than a single collision\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctgfmsccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmctgfmsccr`]
module"]
pub type MMCTGFMSCCR = crate::Reg<mmctgfmsccr::MMCTGFMSCCR_SPEC>;
#[doc = "Ethernet MMC transmitted good frames after more than a single collision"]
pub mod mmctgfmsccr;
#[doc = "MMCTGFCR (r) register accessor: Ethernet MMC transmitted good frames counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctgfcr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmctgfcr`]
module"]
pub type MMCTGFCR = crate::Reg<mmctgfcr::MMCTGFCR_SPEC>;
#[doc = "Ethernet MMC transmitted good frames counter register"]
pub mod mmctgfcr;
#[doc = "MMCRFCECR (r) register accessor: Ethernet MMC received frames with CRC error counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrfcecr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmcrfcecr`]
module"]
pub type MMCRFCECR = crate::Reg<mmcrfcecr::MMCRFCECR_SPEC>;
#[doc = "Ethernet MMC received frames with CRC error counter register"]
pub mod mmcrfcecr;
#[doc = "MMCRFAECR (r) register accessor: Ethernet MMC received frames with alignment error counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrfaecr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmcrfaecr`]
module"]
pub type MMCRFAECR = crate::Reg<mmcrfaecr::MMCRFAECR_SPEC>;
#[doc = "Ethernet MMC received frames with alignment error counter register"]
pub mod mmcrfaecr;
#[doc = "MMCRGUFCR (r) register accessor: MMC received good unicast frames counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrgufcr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmcrgufcr`]
module"]
pub type MMCRGUFCR = crate::Reg<mmcrgufcr::MMCRGUFCR_SPEC>;
#[doc = "MMC received good unicast frames counter register"]
pub mod mmcrgufcr;
