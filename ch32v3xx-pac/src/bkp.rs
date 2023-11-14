#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Backup data register (BKP_DR)"]
    pub datar1: DATAR1,
    #[doc = "0x08 - Backup data register (BKP_DR)"]
    pub datar2: DATAR2,
    #[doc = "0x0c - Backup data register (BKP_DR)"]
    pub datar3: DATAR3,
    #[doc = "0x10 - Backup data register (BKP_DR)"]
    pub datar4: DATAR4,
    #[doc = "0x14 - Backup data register (BKP_DR)"]
    pub datar5: DATAR5,
    #[doc = "0x18 - Backup data register (BKP_DR)"]
    pub datar6: DATAR6,
    #[doc = "0x1c - Backup data register (BKP_DR)"]
    pub datar7: DATAR7,
    #[doc = "0x20 - Backup data register (BKP_DR)"]
    pub datar8: DATAR8,
    #[doc = "0x24 - Backup data register (BKP_DR)"]
    pub datar9: DATAR9,
    #[doc = "0x28 - Backup data register (BKP_DR)"]
    pub datar10: DATAR10,
    #[doc = "0x2c - RTC clock calibration register (BKP_OCTLR)"]
    pub octlr: OCTLR,
    #[doc = "0x30 - Backup control register (BKP_TPCTLR)"]
    pub tpctlr: TPCTLR,
    #[doc = "0x34 - BKP_TPCSR control/status register (BKP_CSR)"]
    pub tpcsr: TPCSR,
    _reserved13: [u8; 0x08],
    #[doc = "0x40 - Backup data register (BKP_DR)"]
    pub datar11: DATAR11,
    #[doc = "0x44 - Backup data register (BKP_DR)"]
    pub datar12: DATAR12,
    #[doc = "0x48 - Backup data register (BKP_DR)"]
    pub datar13: DATAR13,
    #[doc = "0x4c - Backup data register (BKP_DR)"]
    pub datar14: DATAR14,
    #[doc = "0x50 - Backup data register (BKP_DR)"]
    pub datar15: DATAR15,
    #[doc = "0x54 - Backup data register (BKP_DR)"]
    pub datar16: DATAR16,
    #[doc = "0x58 - Backup data register (BKP_DR)"]
    pub datar17: DATAR17,
    #[doc = "0x5c - Backup data register (BKP_DR)"]
    pub datar18: DATAR18,
    #[doc = "0x60 - Backup data register (BKP_DR)"]
    pub datar19: DATAR19,
    #[doc = "0x64 - Backup data register (BKP_DR)"]
    pub datar20: DATAR20,
    #[doc = "0x68 - Backup data register (BKP_DR)"]
    pub datar21: DATAR21,
    #[doc = "0x6c - Backup data register (BKP_DR)"]
    pub datar22: DATAR22,
    #[doc = "0x70 - Backup data register (BKP_DR)"]
    pub datar23: DATAR23,
    #[doc = "0x74 - Backup data register (BKP_DR)"]
    pub datar24: DATAR24,
    #[doc = "0x78 - Backup data register (BKP_DR)"]
    pub datar25: DATAR25,
    #[doc = "0x7c - Backup data register (BKP_DR)"]
    pub datar26: DATAR26,
    #[doc = "0x80 - Backup data register (BKP_DR)"]
    pub datar27: DATAR27,
    #[doc = "0x84 - Backup data register (BKP_DR)"]
    pub datar28: DATAR28,
    #[doc = "0x88 - Backup data register (BKP_DR)"]
    pub datar29: DATAR29,
    #[doc = "0x8c - Backup data register (BKP_DR)"]
    pub datar30: DATAR30,
    #[doc = "0x90 - Backup data register (BKP_DR)"]
    pub datar31: DATAR31,
    #[doc = "0x94 - Backup data register (BKP_DR)"]
    pub datar32: DATAR32,
    #[doc = "0x98 - Backup data register (BKP_DR)"]
    pub datar33: DATAR33,
    #[doc = "0x9c - Backup data register (BKP_DR)"]
    pub datar34: DATAR34,
    #[doc = "0xa0 - Backup data register (BKP_DR)"]
    pub datar35: DATAR35,
    #[doc = "0xa4 - Backup data register (BKP_DR)"]
    pub datar36: DATAR36,
    #[doc = "0xa8 - Backup data register (BKP_DR)"]
    pub datar37: DATAR37,
    #[doc = "0xac - Backup data register (BKP_DR)"]
    pub datar38: DATAR38,
    #[doc = "0xb0 - Backup data register (BKP_DR)"]
    pub datar39: DATAR39,
    #[doc = "0xb4 - Backup data register (BKP_DR)"]
    pub datar40: DATAR40,
    #[doc = "0xb8 - Backup data register (BKP_DR)"]
    pub datar41: DATAR41,
    #[doc = "0xbc - Backup data register (BKP_DR)"]
    pub datar42: DATAR42,
}
#[doc = "DATAR1 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar1`]
module"]
pub type DATAR1 = crate::Reg<datar1::DATAR1_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar1;
#[doc = "DATAR2 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar2`]
module"]
pub type DATAR2 = crate::Reg<datar2::DATAR2_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar2;
#[doc = "DATAR3 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar3`]
module"]
pub type DATAR3 = crate::Reg<datar3::DATAR3_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar3;
#[doc = "DATAR4 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar4`]
module"]
pub type DATAR4 = crate::Reg<datar4::DATAR4_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar4;
#[doc = "DATAR5 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar5`]
module"]
pub type DATAR5 = crate::Reg<datar5::DATAR5_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar5;
#[doc = "DATAR6 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar6`]
module"]
pub type DATAR6 = crate::Reg<datar6::DATAR6_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar6;
#[doc = "DATAR7 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar7`]
module"]
pub type DATAR7 = crate::Reg<datar7::DATAR7_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar7;
#[doc = "DATAR8 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar8`]
module"]
pub type DATAR8 = crate::Reg<datar8::DATAR8_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar8;
#[doc = "DATAR9 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar9`]
module"]
pub type DATAR9 = crate::Reg<datar9::DATAR9_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar9;
#[doc = "DATAR10 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar10`]
module"]
pub type DATAR10 = crate::Reg<datar10::DATAR10_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar10;
#[doc = "DATAR11 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar11`]
module"]
pub type DATAR11 = crate::Reg<datar11::DATAR11_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar11;
#[doc = "DATAR12 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar12`]
module"]
pub type DATAR12 = crate::Reg<datar12::DATAR12_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar12;
#[doc = "DATAR13 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar13`]
module"]
pub type DATAR13 = crate::Reg<datar13::DATAR13_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar13;
#[doc = "DATAR14 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar14`]
module"]
pub type DATAR14 = crate::Reg<datar14::DATAR14_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar14;
#[doc = "DATAR15 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar15`]
module"]
pub type DATAR15 = crate::Reg<datar15::DATAR15_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar15;
#[doc = "DATAR16 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar16`]
module"]
pub type DATAR16 = crate::Reg<datar16::DATAR16_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar16;
#[doc = "DATAR17 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar17`]
module"]
pub type DATAR17 = crate::Reg<datar17::DATAR17_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar17;
#[doc = "DATAR18 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar18`]
module"]
pub type DATAR18 = crate::Reg<datar18::DATAR18_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar18;
#[doc = "DATAR19 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar19`]
module"]
pub type DATAR19 = crate::Reg<datar19::DATAR19_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar19;
#[doc = "DATAR20 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar20`]
module"]
pub type DATAR20 = crate::Reg<datar20::DATAR20_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar20;
#[doc = "DATAR21 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar21`]
module"]
pub type DATAR21 = crate::Reg<datar21::DATAR21_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar21;
#[doc = "DATAR22 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar22`]
module"]
pub type DATAR22 = crate::Reg<datar22::DATAR22_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar22;
#[doc = "DATAR23 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar23`]
module"]
pub type DATAR23 = crate::Reg<datar23::DATAR23_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar23;
#[doc = "DATAR24 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar24`]
module"]
pub type DATAR24 = crate::Reg<datar24::DATAR24_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar24;
#[doc = "DATAR25 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar25`]
module"]
pub type DATAR25 = crate::Reg<datar25::DATAR25_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar25;
#[doc = "DATAR26 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar26`]
module"]
pub type DATAR26 = crate::Reg<datar26::DATAR26_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar26;
#[doc = "DATAR27 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar27`]
module"]
pub type DATAR27 = crate::Reg<datar27::DATAR27_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar27;
#[doc = "DATAR28 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar28`]
module"]
pub type DATAR28 = crate::Reg<datar28::DATAR28_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar28;
#[doc = "DATAR29 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar29`]
module"]
pub type DATAR29 = crate::Reg<datar29::DATAR29_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar29;
#[doc = "DATAR30 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar30`]
module"]
pub type DATAR30 = crate::Reg<datar30::DATAR30_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar30;
#[doc = "DATAR31 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar31`]
module"]
pub type DATAR31 = crate::Reg<datar31::DATAR31_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar31;
#[doc = "DATAR32 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar32`]
module"]
pub type DATAR32 = crate::Reg<datar32::DATAR32_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar32;
#[doc = "DATAR33 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar33`]
module"]
pub type DATAR33 = crate::Reg<datar33::DATAR33_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar33;
#[doc = "DATAR34 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar34`]
module"]
pub type DATAR34 = crate::Reg<datar34::DATAR34_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar34;
#[doc = "DATAR35 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar35`]
module"]
pub type DATAR35 = crate::Reg<datar35::DATAR35_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar35;
#[doc = "DATAR36 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar36`]
module"]
pub type DATAR36 = crate::Reg<datar36::DATAR36_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar36;
#[doc = "DATAR37 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar37`]
module"]
pub type DATAR37 = crate::Reg<datar37::DATAR37_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar37;
#[doc = "DATAR38 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar38`]
module"]
pub type DATAR38 = crate::Reg<datar38::DATAR38_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar38;
#[doc = "DATAR39 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar39`]
module"]
pub type DATAR39 = crate::Reg<datar39::DATAR39_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar39;
#[doc = "DATAR40 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar40`]
module"]
pub type DATAR40 = crate::Reg<datar40::DATAR40_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar40;
#[doc = "DATAR41 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar41`]
module"]
pub type DATAR41 = crate::Reg<datar41::DATAR41_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar41;
#[doc = "DATAR42 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar42::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datar42`]
module"]
pub type DATAR42 = crate::Reg<datar42::DATAR42_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar42;
#[doc = "OCTLR (rw) register accessor: RTC clock calibration register (BKP_OCTLR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`octlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`octlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@octlr`]
module"]
pub type OCTLR = crate::Reg<octlr::OCTLR_SPEC>;
#[doc = "RTC clock calibration register (BKP_OCTLR)"]
pub mod octlr;
#[doc = "TPCTLR (rw) register accessor: Backup control register (BKP_TPCTLR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpctlr`]
module"]
pub type TPCTLR = crate::Reg<tpctlr::TPCTLR_SPEC>;
#[doc = "Backup control register (BKP_TPCTLR)"]
pub mod tpctlr;
#[doc = "TPCSR (rw) register accessor: BKP_TPCSR control/status register (BKP_CSR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpcsr`]
module"]
pub type TPCSR = crate::Reg<tpcsr::TPCSR_SPEC>;
#[doc = "BKP_TPCSR control/status register (BKP_CSR)"]
pub mod tpcsr;
