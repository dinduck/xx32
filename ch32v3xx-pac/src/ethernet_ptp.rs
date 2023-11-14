#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet PTP time stamp control register (ETH_PTPTSCR)"]
    pub ptptscr: PTPTSCR,
    #[doc = "0x04 - Ethernet PTP subsecond increment register"]
    pub ptpssir: PTPSSIR,
    #[doc = "0x08 - Ethernet PTP time stamp high register"]
    pub ptptshr: PTPTSHR,
    #[doc = "0x0c - Ethernet PTP time stamp low register (ETH_PTPTSLR)"]
    pub ptptslr: PTPTSLR,
    #[doc = "0x10 - Ethernet PTP time stamp high update register"]
    pub ptptshur: PTPTSHUR,
    #[doc = "0x14 - Ethernet PTP time stamp low update register (ETH_PTPTSLUR)"]
    pub ptptslur: PTPTSLUR,
    #[doc = "0x18 - Ethernet PTP time stamp addend register"]
    pub ptptsar: PTPTSAR,
    #[doc = "0x1c - Ethernet PTP target time high register"]
    pub ptptthr: PTPTTHR,
    #[doc = "0x20 - Ethernet PTP target time low register"]
    pub ptpttlr: PTPTTLR,
}
#[doc = "PTPTSCR (rw) register accessor: Ethernet PTP time stamp control register (ETH_PTPTSCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptscr`]
module"]
pub type PTPTSCR = crate::Reg<ptptscr::PTPTSCR_SPEC>;
#[doc = "Ethernet PTP time stamp control register (ETH_PTPTSCR)"]
pub mod ptptscr;
#[doc = "PTPSSIR (rw) register accessor: Ethernet PTP subsecond increment register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptpssir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptpssir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptpssir`]
module"]
pub type PTPSSIR = crate::Reg<ptpssir::PTPSSIR_SPEC>;
#[doc = "Ethernet PTP subsecond increment register"]
pub mod ptpssir;
#[doc = "PTPTSHR (r) register accessor: Ethernet PTP time stamp high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptshr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptshr`]
module"]
pub type PTPTSHR = crate::Reg<ptptshr::PTPTSHR_SPEC>;
#[doc = "Ethernet PTP time stamp high register"]
pub mod ptptshr;
#[doc = "PTPTSLR (r) register accessor: Ethernet PTP time stamp low register (ETH_PTPTSLR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptslr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptslr`]
module"]
pub type PTPTSLR = crate::Reg<ptptslr::PTPTSLR_SPEC>;
#[doc = "Ethernet PTP time stamp low register (ETH_PTPTSLR)"]
pub mod ptptslr;
#[doc = "PTPTSHUR (rw) register accessor: Ethernet PTP time stamp high update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptshur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptshur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptshur`]
module"]
pub type PTPTSHUR = crate::Reg<ptptshur::PTPTSHUR_SPEC>;
#[doc = "Ethernet PTP time stamp high update register"]
pub mod ptptshur;
#[doc = "PTPTSLUR (rw) register accessor: Ethernet PTP time stamp low update register (ETH_PTPTSLUR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptslur::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptslur::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptslur`]
module"]
pub type PTPTSLUR = crate::Reg<ptptslur::PTPTSLUR_SPEC>;
#[doc = "Ethernet PTP time stamp low update register (ETH_PTPTSLUR)"]
pub mod ptptslur;
#[doc = "PTPTSAR (rw) register accessor: Ethernet PTP time stamp addend register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptsar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptsar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptsar`]
module"]
pub type PTPTSAR = crate::Reg<ptptsar::PTPTSAR_SPEC>;
#[doc = "Ethernet PTP time stamp addend register"]
pub mod ptptsar;
#[doc = "PTPTTHR (rw) register accessor: Ethernet PTP target time high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptthr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptthr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptthr`]
module"]
pub type PTPTTHR = crate::Reg<ptptthr::PTPTTHR_SPEC>;
#[doc = "Ethernet PTP target time high register"]
pub mod ptptthr;
#[doc = "PTPTTLR (rw) register accessor: Ethernet PTP target time low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptpttlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptpttlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptpttlr`]
module"]
pub type PTPTTLR = crate::Reg<ptpttlr::PTPTTLR_SPEC>;
#[doc = "Ethernet PTP target time low register"]
pub mod ptpttlr;
