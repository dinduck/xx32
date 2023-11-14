#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet MAC configuration register (ETH_MACCR)"]
    pub maccr: MACCR,
    #[doc = "0x04 - Ethernet MAC frame filter register (ETH_MACCFFR)"]
    pub macffr: MACFFR,
    #[doc = "0x08 - Ethernet MAC hash table high register"]
    pub machthr: MACHTHR,
    #[doc = "0x0c - Ethernet MAC hash table low register"]
    pub machtlr: MACHTLR,
    #[doc = "0x10 - Ethernet MAC MII address register (ETH_MACMIIAR)"]
    pub macmiiar: MACMIIAR,
    #[doc = "0x14 - Ethernet MAC MII data register (ETH_MACMIIDR)"]
    pub macmiidr: MACMIIDR,
    #[doc = "0x18 - Ethernet MAC flow control register (ETH_MACFCR)"]
    pub macfcr: MACFCR,
    #[doc = "0x1c - Ethernet MAC VLAN tag register (ETH_MACVLANTR)"]
    pub macvlantr: MACVLANTR,
    _reserved8: [u8; 0x08],
    #[doc = "0x28 - Ethernet MAC remote wakeup frame filter register (ETH_MACRWUFFR)"]
    pub macrwuffr: MACRWUFFR,
    #[doc = "0x2c - Ethernet MAC PMT control and status register (ETH_MACPMTCSR)"]
    pub macpmtcsr: MACPMTCSR,
    _reserved10: [u8; 0x08],
    #[doc = "0x38 - Ethernet MAC interrupt status register (ETH_MACSR)"]
    pub macsr: MACSR,
    #[doc = "0x3c - Ethernet MAC interrupt mask register (ETH_MACIMR)"]
    pub macimr: MACIMR,
    #[doc = "0x40 - Ethernet MAC address 0 high register (ETH_MACA0HR)"]
    pub maca0hr: MACA0HR,
    #[doc = "0x44 - Ethernet MAC address 0 low register"]
    pub maca0lr: MACA0LR,
    #[doc = "0x48 - Ethernet MAC address 1 high register (ETH_MACA1HR)"]
    pub maca1hr: MACA1HR,
    #[doc = "0x4c - Ethernet MAC address1 low register"]
    pub maca1lr: MACA1LR,
    #[doc = "0x50 - Ethernet MAC address 2 high register (ETH_MACA2HR)"]
    pub maca2hr: MACA2HR,
    #[doc = "0x54 - Ethernet MAC address 2 low register"]
    pub maca2lr: MACA2LR,
    #[doc = "0x58 - Ethernet MAC address 3 high register (ETH_MACA3HR)"]
    pub maca3hr: MACA3HR,
    #[doc = "0x5c - Ethernet MAC address 3 low register"]
    pub maca3lr: MACA3LR,
}
#[doc = "MACCR (rw) register accessor: Ethernet MAC configuration register (ETH_MACCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maccr`]
module"]
pub type MACCR = crate::Reg<maccr::MACCR_SPEC>;
#[doc = "Ethernet MAC configuration register (ETH_MACCR)"]
pub mod maccr;
#[doc = "MACFFR (rw) register accessor: Ethernet MAC frame filter register (ETH_MACCFFR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macffr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macffr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macffr`]
module"]
pub type MACFFR = crate::Reg<macffr::MACFFR_SPEC>;
#[doc = "Ethernet MAC frame filter register (ETH_MACCFFR)"]
pub mod macffr;
#[doc = "MACHTHR (rw) register accessor: Ethernet MAC hash table high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`machthr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`machthr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@machthr`]
module"]
pub type MACHTHR = crate::Reg<machthr::MACHTHR_SPEC>;
#[doc = "Ethernet MAC hash table high register"]
pub mod machthr;
#[doc = "MACHTLR (rw) register accessor: Ethernet MAC hash table low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`machtlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`machtlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@machtlr`]
module"]
pub type MACHTLR = crate::Reg<machtlr::MACHTLR_SPEC>;
#[doc = "Ethernet MAC hash table low register"]
pub mod machtlr;
#[doc = "MACMIIAR (rw) register accessor: Ethernet MAC MII address register (ETH_MACMIIAR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macmiiar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macmiiar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macmiiar`]
module"]
pub type MACMIIAR = crate::Reg<macmiiar::MACMIIAR_SPEC>;
#[doc = "Ethernet MAC MII address register (ETH_MACMIIAR)"]
pub mod macmiiar;
#[doc = "MACMIIDR (rw) register accessor: Ethernet MAC MII data register (ETH_MACMIIDR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macmiidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macmiidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macmiidr`]
module"]
pub type MACMIIDR = crate::Reg<macmiidr::MACMIIDR_SPEC>;
#[doc = "Ethernet MAC MII data register (ETH_MACMIIDR)"]
pub mod macmiidr;
#[doc = "MACFCR (rw) register accessor: Ethernet MAC flow control register (ETH_MACFCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macfcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macfcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macfcr`]
module"]
pub type MACFCR = crate::Reg<macfcr::MACFCR_SPEC>;
#[doc = "Ethernet MAC flow control register (ETH_MACFCR)"]
pub mod macfcr;
#[doc = "MACVLANTR (rw) register accessor: Ethernet MAC VLAN tag register (ETH_MACVLANTR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macvlantr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macvlantr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macvlantr`]
module"]
pub type MACVLANTR = crate::Reg<macvlantr::MACVLANTR_SPEC>;
#[doc = "Ethernet MAC VLAN tag register (ETH_MACVLANTR)"]
pub mod macvlantr;
#[doc = "MACRWUFFR (rw) register accessor: Ethernet MAC remote wakeup frame filter register (ETH_MACRWUFFR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macrwuffr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macrwuffr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macrwuffr`]
module"]
pub type MACRWUFFR = crate::Reg<macrwuffr::MACRWUFFR_SPEC>;
#[doc = "Ethernet MAC remote wakeup frame filter register (ETH_MACRWUFFR)"]
pub mod macrwuffr;
#[doc = "MACPMTCSR (rw) register accessor: Ethernet MAC PMT control and status register (ETH_MACPMTCSR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macpmtcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macpmtcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macpmtcsr`]
module"]
pub type MACPMTCSR = crate::Reg<macpmtcsr::MACPMTCSR_SPEC>;
#[doc = "Ethernet MAC PMT control and status register (ETH_MACPMTCSR)"]
pub mod macpmtcsr;
#[doc = "MACSR (rw) register accessor: Ethernet MAC interrupt status register (ETH_MACSR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macsr`]
module"]
pub type MACSR = crate::Reg<macsr::MACSR_SPEC>;
#[doc = "Ethernet MAC interrupt status register (ETH_MACSR)"]
pub mod macsr;
#[doc = "MACIMR (rw) register accessor: Ethernet MAC interrupt mask register (ETH_MACIMR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macimr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macimr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macimr`]
module"]
pub type MACIMR = crate::Reg<macimr::MACIMR_SPEC>;
#[doc = "Ethernet MAC interrupt mask register (ETH_MACIMR)"]
pub mod macimr;
#[doc = "MACA0HR (rw) register accessor: Ethernet MAC address 0 high register (ETH_MACA0HR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca0hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca0hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca0hr`]
module"]
pub type MACA0HR = crate::Reg<maca0hr::MACA0HR_SPEC>;
#[doc = "Ethernet MAC address 0 high register (ETH_MACA0HR)"]
pub mod maca0hr;
#[doc = "MACA0LR (rw) register accessor: Ethernet MAC address 0 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca0lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca0lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca0lr`]
module"]
pub type MACA0LR = crate::Reg<maca0lr::MACA0LR_SPEC>;
#[doc = "Ethernet MAC address 0 low register"]
pub mod maca0lr;
#[doc = "MACA1HR (rw) register accessor: Ethernet MAC address 1 high register (ETH_MACA1HR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca1hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca1hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca1hr`]
module"]
pub type MACA1HR = crate::Reg<maca1hr::MACA1HR_SPEC>;
#[doc = "Ethernet MAC address 1 high register (ETH_MACA1HR)"]
pub mod maca1hr;
#[doc = "MACA1LR (rw) register accessor: Ethernet MAC address1 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca1lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca1lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca1lr`]
module"]
pub type MACA1LR = crate::Reg<maca1lr::MACA1LR_SPEC>;
#[doc = "Ethernet MAC address1 low register"]
pub mod maca1lr;
#[doc = "MACA2HR (rw) register accessor: Ethernet MAC address 2 high register (ETH_MACA2HR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca2hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca2hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca2hr`]
module"]
pub type MACA2HR = crate::Reg<maca2hr::MACA2HR_SPEC>;
#[doc = "Ethernet MAC address 2 high register (ETH_MACA2HR)"]
pub mod maca2hr;
#[doc = "MACA2LR (rw) register accessor: Ethernet MAC address 2 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca2lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca2lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca2lr`]
module"]
pub type MACA2LR = crate::Reg<maca2lr::MACA2LR_SPEC>;
#[doc = "Ethernet MAC address 2 low register"]
pub mod maca2lr;
#[doc = "MACA3HR (rw) register accessor: Ethernet MAC address 3 high register (ETH_MACA3HR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca3hr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca3hr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca3hr`]
module"]
pub type MACA3HR = crate::Reg<maca3hr::MACA3HR_SPEC>;
#[doc = "Ethernet MAC address 3 high register (ETH_MACA3HR)"]
pub mod maca3hr;
#[doc = "MACA3LR (rw) register accessor: Ethernet MAC address 3 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca3lr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca3lr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca3lr`]
module"]
pub type MACA3LR = crate::Reg<maca3lr::MACA3LR_SPEC>;
#[doc = "Ethernet MAC address 3 low register"]
pub mod maca3lr;
