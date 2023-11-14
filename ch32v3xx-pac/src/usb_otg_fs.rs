#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB base control"]
    pub usbhd_base_ctrl: USBHD_BASE_CTRL,
    #[doc = "0x01 - USB device/host physical prot control"]
    pub usbhd_udev_ctrl__usbhd_uhost_ctrl: USBHD_UDEV_CTRL__USBHD_UHOST_CTRL,
    #[doc = "0x02 - USB interrupt enable"]
    pub r8_usb_int_en: R8_USB_INT_EN,
    #[doc = "0x03 - USB device address"]
    pub r8_usb_dev_ad: R8_USB_DEV_AD,
    _reserved4: [u8; 0x01],
    #[doc = "0x05 - USB miscellaneous status"]
    pub r8_usb_mis_st: R8_USB_MIS_ST,
    #[doc = "0x06 - USB interrupt flag"]
    pub r8_usb_int_fg: R8_USB_INT_FG,
    #[doc = "0x07 - USB interrupt status"]
    pub r8_usb_int_st: R8_USB_INT_ST,
    #[doc = "0x08 - USB receiving length"]
    pub r16_usb_rx_len: R16_USB_RX_LEN,
    _reserved8: [u8; 0x02],
    #[doc = "0x0c - endpoint 4/1 mode"]
    pub r8_uep4_1_mod: R8_UEP4_1_MOD,
    #[doc = "0x0d - endpoint 2/3 mode;host endpoint mode"]
    pub r8_uep2_3_mod__r8_uh_ep_mod: R8_UEP2_3_MOD__R8_UH_EP_MOD,
    #[doc = "0x0e - endpoint 5/6 mode"]
    pub r8_uep5_6_mod: R8_UEP5_6_MOD,
    #[doc = "0x0f - endpoint 7 mode"]
    pub r8_uep7_mod: R8_UEP7_MOD,
    #[doc = "0x10 - endpoint 0 DMA buffer address"]
    pub r32_uep0_dma: R32_UEP0_DMA,
    #[doc = "0x14 - endpoint 1 DMA buffer address"]
    pub r32_uep1_dma: R32_UEP1_DMA,
    #[doc = "0x18 - endpoint 2 DMA buffer address;host rx endpoint buffer high address"]
    pub r32_uep2_dma__r32_uh_rx_dma: R32_UEP2_DMA__R32_UH_RX_DMA,
    #[doc = "0x1c - endpoint 3 DMA buffer address;host tx endpoint buffer high address"]
    pub r32_uep3_dma__r32_uh_tx_dma: R32_UEP3_DMA__R32_UH_TX_DMA,
    #[doc = "0x20 - endpoint 4 DMA buffer address"]
    pub r32_uep4_dma: R32_UEP4_DMA,
    #[doc = "0x24 - endpoint 5 DMA buffer address"]
    pub r32_uep5_dma: R32_UEP5_DMA,
    #[doc = "0x28 - endpoint 6 DMA buffer address"]
    pub r32_uep6_dma: R32_UEP6_DMA,
    #[doc = "0x2c - endpoint 7 DMA buffer address"]
    pub r32_uep7_dma: R32_UEP7_DMA,
    #[doc = "0x30 - endpoint 0 transmittal length"]
    pub r8_uep0_t_len: R8_UEP0_T_LEN,
    _reserved21: [u8; 0x01],
    #[doc = "0x32 - endpoint 0 control"]
    pub r8_uep0_t_ctrl: R8_UEP0_T_CTRL,
    #[doc = "0x33 - endpoint 0 control"]
    pub r8_uep0_r_ctrl: R8_UEP0_R_CTRL,
    #[doc = "0x34 - endpoint 1 transmittal length"]
    pub r8_uep1_t_len: R8_UEP1_T_LEN,
    _reserved24: [u8; 0x01],
    #[doc = "0x36 - endpoint 1 control"]
    pub r8_uep1_t_ctrl___usbhd_uh_setup: R8_UEP1_T_CTRL___USBHD_UH_SETUP,
    #[doc = "0x37 - endpoint 1 control"]
    pub r8_uep1_r_ctrl: R8_UEP1_R_CTRL,
    #[doc = "0x38 - endpoint 2 transmittal length"]
    pub r8_uep2_t_len__usbhd_uh_ep_pid: R8_UEP2_T_LEN__USBHD_UH_EP_PID,
    _reserved27: [u8; 0x01],
    #[doc = "0x3a - endpoint 2 control"]
    pub r8_uep2_t_ctrl: R8_UEP2_T_CTRL,
    #[doc = "0x3b - endpoint 2 control"]
    pub r8_uep2_r_ctrl__usbhd_uh_rx_ctrl: R8_UEP2_R_CTRL__USBHD_UH_RX_CTRL,
    #[doc = "0x3c - endpoint 3 transmittal length"]
    pub r8_uep3_t_len__usbhd_uh_tx_len: R8_UEP3_T_LEN__USBHD_UH_TX_LEN,
    _reserved30: [u8; 0x01],
    #[doc = "0x3e - endpoint 3 control"]
    pub r8_uep3_t_ctrl__usbhd_uh_tx_ctrl: R8_UEP3_T_CTRL__USBHD_UH_TX_CTRL,
    #[doc = "0x3f - endpoint 3 control"]
    pub r8_uep3_r_ctrl_: R8_UEP3_R_CTRL_,
    #[doc = "0x40 - endpoint 4 transmittal length"]
    pub r8_uep4_t_len: R8_UEP4_T_LEN,
    _reserved33: [u8; 0x01],
    #[doc = "0x42 - endpoint 4 control"]
    pub r8_uep4_t_ctrl: R8_UEP4_T_CTRL,
    #[doc = "0x43 - endpoint 4 control"]
    pub r8_uep4_r_ctrl_: R8_UEP4_R_CTRL_,
    #[doc = "0x44 - endpoint 5 transmittal length"]
    pub r8_uep5_t_len: R8_UEP5_T_LEN,
    _reserved36: [u8; 0x01],
    #[doc = "0x46 - endpoint 5 control"]
    pub r8_uep5_t_ctrl: R8_UEP5_T_CTRL,
    #[doc = "0x47 - endpoint 5 control"]
    pub r8_uep5_r_ctrl_: R8_UEP5_R_CTRL_,
    #[doc = "0x48 - endpoint 6 transmittal length"]
    pub r8_uep6_t_len: R8_UEP6_T_LEN,
    _reserved39: [u8; 0x01],
    #[doc = "0x4a - endpoint 6 control"]
    pub r8_uep6_t_ctrl: R8_UEP6_T_CTRL,
    #[doc = "0x4b - endpoint 6 control"]
    pub r8_uep6_r_ctrl_: R8_UEP6_R_CTRL_,
    #[doc = "0x4c - endpoint 7 transmittal length"]
    pub r8_uep7_t_len: R8_UEP7_T_LEN,
    _reserved42: [u8; 0x01],
    #[doc = "0x4e - endpoint 7 control"]
    pub r8_uep7_t_ctrl: R8_UEP7_T_CTRL,
    #[doc = "0x4f - endpoint 7 control"]
    pub r8_uep7_r_ctrl_: R8_UEP7_R_CTRL_,
    _reserved44: [u8; 0x04],
    #[doc = "0x54 - usb otg control"]
    pub usb_otg_cr: USB_OTG_CR,
    #[doc = "0x58 - usb otg status"]
    pub usb_otg_sr: USB_OTG_SR,
}
#[doc = "USBHD_BASE_CTRL (rw) register accessor: USB base control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbhd_base_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbhd_base_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbhd_base_ctrl`]
module"]
pub type USBHD_BASE_CTRL = crate::Reg<usbhd_base_ctrl::USBHD_BASE_CTRL_SPEC>;
#[doc = "USB base control"]
pub mod usbhd_base_ctrl;
#[doc = "USBHD_UDEV_CTRL__USBHD_UHOST_CTRL (rw) register accessor: USB device/host physical prot control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbhd_udev_ctrl__usbhd_uhost_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbhd_udev_ctrl__usbhd_uhost_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbhd_udev_ctrl__usbhd_uhost_ctrl`]
module"]
pub type USBHD_UDEV_CTRL__USBHD_UHOST_CTRL =
    crate::Reg<usbhd_udev_ctrl__usbhd_uhost_ctrl::USBHD_UDEV_CTRL__USBHD_UHOST_CTRL_SPEC>;
#[doc = "USB device/host physical prot control"]
pub mod usbhd_udev_ctrl__usbhd_uhost_ctrl;
#[doc = "R8_USB_INT_EN (rw) register accessor: USB interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_usb_int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_usb_int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_usb_int_en`]
module"]
pub type R8_USB_INT_EN = crate::Reg<r8_usb_int_en::R8_USB_INT_EN_SPEC>;
#[doc = "USB interrupt enable"]
pub mod r8_usb_int_en;
#[doc = "R8_USB_DEV_AD (rw) register accessor: USB device address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_usb_dev_ad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_usb_dev_ad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_usb_dev_ad`]
module"]
pub type R8_USB_DEV_AD = crate::Reg<r8_usb_dev_ad::R8_USB_DEV_AD_SPEC>;
#[doc = "USB device address"]
pub mod r8_usb_dev_ad;
#[doc = "R8_USB_MIS_ST (r) register accessor: USB miscellaneous status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_usb_mis_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_usb_mis_st`]
module"]
pub type R8_USB_MIS_ST = crate::Reg<r8_usb_mis_st::R8_USB_MIS_ST_SPEC>;
#[doc = "USB miscellaneous status"]
pub mod r8_usb_mis_st;
#[doc = "R8_USB_INT_FG (rw) register accessor: USB interrupt flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_usb_int_fg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_usb_int_fg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_usb_int_fg`]
module"]
pub type R8_USB_INT_FG = crate::Reg<r8_usb_int_fg::R8_USB_INT_FG_SPEC>;
#[doc = "USB interrupt flag"]
pub mod r8_usb_int_fg;
#[doc = "R8_USB_INT_ST (r) register accessor: USB interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_usb_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_usb_int_st`]
module"]
pub type R8_USB_INT_ST = crate::Reg<r8_usb_int_st::R8_USB_INT_ST_SPEC>;
#[doc = "USB interrupt status"]
pub mod r8_usb_int_st;
#[doc = "R16_USB_RX_LEN (r) register accessor: USB receiving length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r16_usb_rx_len::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r16_usb_rx_len`]
module"]
pub type R16_USB_RX_LEN = crate::Reg<r16_usb_rx_len::R16_USB_RX_LEN_SPEC>;
#[doc = "USB receiving length"]
pub mod r16_usb_rx_len;
#[doc = "R8_UEP4_1_MOD (rw) register accessor: endpoint 4/1 mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep4_1_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep4_1_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep4_1_mod`]
module"]
pub type R8_UEP4_1_MOD = crate::Reg<r8_uep4_1_mod::R8_UEP4_1_MOD_SPEC>;
#[doc = "endpoint 4/1 mode"]
pub mod r8_uep4_1_mod;
#[doc = "R8_UEP2_3_MOD__R8_UH_EP_MOD (rw) register accessor: endpoint 2/3 mode;host endpoint mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep2_3_mod__r8_uh_ep_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep2_3_mod__r8_uh_ep_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep2_3_mod__r8_uh_ep_mod`]
module"]
pub type R8_UEP2_3_MOD__R8_UH_EP_MOD =
    crate::Reg<r8_uep2_3_mod__r8_uh_ep_mod::R8_UEP2_3_MOD__R8_UH_EP_MOD_SPEC>;
#[doc = "endpoint 2/3 mode;host endpoint mode"]
pub mod r8_uep2_3_mod__r8_uh_ep_mod;
#[doc = "R8_UEP5_6_MOD (rw) register accessor: endpoint 5/6 mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep5_6_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep5_6_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep5_6_mod`]
module"]
pub type R8_UEP5_6_MOD = crate::Reg<r8_uep5_6_mod::R8_UEP5_6_MOD_SPEC>;
#[doc = "endpoint 5/6 mode"]
pub mod r8_uep5_6_mod;
#[doc = "R8_UEP7_MOD (rw) register accessor: endpoint 7 mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep7_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep7_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep7_mod`]
module"]
pub type R8_UEP7_MOD = crate::Reg<r8_uep7_mod::R8_UEP7_MOD_SPEC>;
#[doc = "endpoint 7 mode"]
pub mod r8_uep7_mod;
#[doc = "R32_UEP0_DMA (rw) register accessor: endpoint 0 DMA buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r32_uep0_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r32_uep0_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_uep0_dma`]
module"]
pub type R32_UEP0_DMA = crate::Reg<r32_uep0_dma::R32_UEP0_DMA_SPEC>;
#[doc = "endpoint 0 DMA buffer address"]
pub mod r32_uep0_dma;
#[doc = "R32_UEP1_DMA (rw) register accessor: endpoint 1 DMA buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r32_uep1_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r32_uep1_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_uep1_dma`]
module"]
pub type R32_UEP1_DMA = crate::Reg<r32_uep1_dma::R32_UEP1_DMA_SPEC>;
#[doc = "endpoint 1 DMA buffer address"]
pub mod r32_uep1_dma;
#[doc = "R32_UEP2_DMA__R32_UH_RX_DMA (rw) register accessor: endpoint 2 DMA buffer address;host rx endpoint buffer high address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r32_uep2_dma__r32_uh_rx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r32_uep2_dma__r32_uh_rx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_uep2_dma__r32_uh_rx_dma`]
module"]
pub type R32_UEP2_DMA__R32_UH_RX_DMA =
    crate::Reg<r32_uep2_dma__r32_uh_rx_dma::R32_UEP2_DMA__R32_UH_RX_DMA_SPEC>;
#[doc = "endpoint 2 DMA buffer address;host rx endpoint buffer high address"]
pub mod r32_uep2_dma__r32_uh_rx_dma;
#[doc = "R32_UEP3_DMA__R32_UH_TX_DMA (rw) register accessor: endpoint 3 DMA buffer address;host tx endpoint buffer high address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r32_uep3_dma__r32_uh_tx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r32_uep3_dma__r32_uh_tx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_uep3_dma__r32_uh_tx_dma`]
module"]
pub type R32_UEP3_DMA__R32_UH_TX_DMA =
    crate::Reg<r32_uep3_dma__r32_uh_tx_dma::R32_UEP3_DMA__R32_UH_TX_DMA_SPEC>;
#[doc = "endpoint 3 DMA buffer address;host tx endpoint buffer high address"]
pub mod r32_uep3_dma__r32_uh_tx_dma;
#[doc = "R32_UEP4_DMA (rw) register accessor: endpoint 4 DMA buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r32_uep4_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r32_uep4_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_uep4_dma`]
module"]
pub type R32_UEP4_DMA = crate::Reg<r32_uep4_dma::R32_UEP4_DMA_SPEC>;
#[doc = "endpoint 4 DMA buffer address"]
pub mod r32_uep4_dma;
#[doc = "R32_UEP5_DMA (rw) register accessor: endpoint 5 DMA buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r32_uep5_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r32_uep5_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_uep5_dma`]
module"]
pub type R32_UEP5_DMA = crate::Reg<r32_uep5_dma::R32_UEP5_DMA_SPEC>;
#[doc = "endpoint 5 DMA buffer address"]
pub mod r32_uep5_dma;
#[doc = "R32_UEP6_DMA (rw) register accessor: endpoint 6 DMA buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r32_uep6_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r32_uep6_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_uep6_dma`]
module"]
pub type R32_UEP6_DMA = crate::Reg<r32_uep6_dma::R32_UEP6_DMA_SPEC>;
#[doc = "endpoint 6 DMA buffer address"]
pub mod r32_uep6_dma;
#[doc = "R32_UEP7_DMA (rw) register accessor: endpoint 7 DMA buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r32_uep7_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r32_uep7_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r32_uep7_dma`]
module"]
pub type R32_UEP7_DMA = crate::Reg<r32_uep7_dma::R32_UEP7_DMA_SPEC>;
#[doc = "endpoint 7 DMA buffer address"]
pub mod r32_uep7_dma;
#[doc = "R8_UEP0_T_LEN (rw) register accessor: endpoint 0 transmittal length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep0_t_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep0_t_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep0_t_len`]
module"]
pub type R8_UEP0_T_LEN = crate::Reg<r8_uep0_t_len::R8_UEP0_T_LEN_SPEC>;
#[doc = "endpoint 0 transmittal length"]
pub mod r8_uep0_t_len;
#[doc = "R8_UEP0_T_CTRL (rw) register accessor: endpoint 0 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep0_t_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep0_t_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep0_t_ctrl`]
module"]
pub type R8_UEP0_T_CTRL = crate::Reg<r8_uep0_t_ctrl::R8_UEP0_T_CTRL_SPEC>;
#[doc = "endpoint 0 control"]
pub mod r8_uep0_t_ctrl;
#[doc = "R8_UEP0_R_CTRL (rw) register accessor: endpoint 0 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep0_r_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep0_r_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep0_r_ctrl`]
module"]
pub type R8_UEP0_R_CTRL = crate::Reg<r8_uep0_r_ctrl::R8_UEP0_R_CTRL_SPEC>;
#[doc = "endpoint 0 control"]
pub mod r8_uep0_r_ctrl;
#[doc = "R8_UEP1_T_LEN (rw) register accessor: endpoint 1 transmittal length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep1_t_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep1_t_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep1_t_len`]
module"]
pub type R8_UEP1_T_LEN = crate::Reg<r8_uep1_t_len::R8_UEP1_T_LEN_SPEC>;
#[doc = "endpoint 1 transmittal length"]
pub mod r8_uep1_t_len;
#[doc = "R8_UEP1_T_CTRL___USBHD_UH_SETUP (rw) register accessor: endpoint 1 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep1_t_ctrl___usbhd_uh_setup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep1_t_ctrl___usbhd_uh_setup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep1_t_ctrl___usbhd_uh_setup`]
module"]
pub type R8_UEP1_T_CTRL___USBHD_UH_SETUP =
    crate::Reg<r8_uep1_t_ctrl___usbhd_uh_setup::R8_UEP1_T_CTRL___USBHD_UH_SETUP_SPEC>;
#[doc = "endpoint 1 control"]
pub mod r8_uep1_t_ctrl___usbhd_uh_setup;
#[doc = "R8_UEP1_R_CTRL (rw) register accessor: endpoint 1 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep1_r_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep1_r_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep1_r_ctrl`]
module"]
pub type R8_UEP1_R_CTRL = crate::Reg<r8_uep1_r_ctrl::R8_UEP1_R_CTRL_SPEC>;
#[doc = "endpoint 1 control"]
pub mod r8_uep1_r_ctrl;
#[doc = "R8_UEP2_T_LEN__USBHD_UH_EP_PID (rw) register accessor: endpoint 2 transmittal length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep2_t_len__usbhd_uh_ep_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep2_t_len__usbhd_uh_ep_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep2_t_len__usbhd_uh_ep_pid`]
module"]
pub type R8_UEP2_T_LEN__USBHD_UH_EP_PID =
    crate::Reg<r8_uep2_t_len__usbhd_uh_ep_pid::R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC>;
#[doc = "endpoint 2 transmittal length"]
pub mod r8_uep2_t_len__usbhd_uh_ep_pid;
#[doc = "R8_UEP2_T_CTRL (rw) register accessor: endpoint 2 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep2_t_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep2_t_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep2_t_ctrl`]
module"]
pub type R8_UEP2_T_CTRL = crate::Reg<r8_uep2_t_ctrl::R8_UEP2_T_CTRL_SPEC>;
#[doc = "endpoint 2 control"]
pub mod r8_uep2_t_ctrl;
#[doc = "R8_UEP2_R_CTRL__USBHD_UH_RX_CTRL (rw) register accessor: endpoint 2 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep2_r_ctrl__usbhd_uh_rx_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep2_r_ctrl__usbhd_uh_rx_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep2_r_ctrl__usbhd_uh_rx_ctrl`]
module"]
pub type R8_UEP2_R_CTRL__USBHD_UH_RX_CTRL =
    crate::Reg<r8_uep2_r_ctrl__usbhd_uh_rx_ctrl::R8_UEP2_R_CTRL__USBHD_UH_RX_CTRL_SPEC>;
#[doc = "endpoint 2 control"]
pub mod r8_uep2_r_ctrl__usbhd_uh_rx_ctrl;
#[doc = "R8_UEP3_T_LEN__USBHD_UH_TX_LEN (rw) register accessor: endpoint 3 transmittal length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep3_t_len__usbhd_uh_tx_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep3_t_len__usbhd_uh_tx_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep3_t_len__usbhd_uh_tx_len`]
module"]
pub type R8_UEP3_T_LEN__USBHD_UH_TX_LEN =
    crate::Reg<r8_uep3_t_len__usbhd_uh_tx_len::R8_UEP3_T_LEN__USBHD_UH_TX_LEN_SPEC>;
#[doc = "endpoint 3 transmittal length"]
pub mod r8_uep3_t_len__usbhd_uh_tx_len;
#[doc = "R8_UEP3_T_CTRL__USBHD_UH_TX_CTRL (rw) register accessor: endpoint 3 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep3_t_ctrl__usbhd_uh_tx_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep3_t_ctrl__usbhd_uh_tx_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep3_t_ctrl__usbhd_uh_tx_ctrl`]
module"]
pub type R8_UEP3_T_CTRL__USBHD_UH_TX_CTRL =
    crate::Reg<r8_uep3_t_ctrl__usbhd_uh_tx_ctrl::R8_UEP3_T_CTRL__USBHD_UH_TX_CTRL_SPEC>;
#[doc = "endpoint 3 control"]
pub mod r8_uep3_t_ctrl__usbhd_uh_tx_ctrl;
#[doc = "R8_UEP3_R_CTRL_ (rw) register accessor: endpoint 3 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep3_r_ctrl_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep3_r_ctrl_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep3_r_ctrl_`]
module"]
pub type R8_UEP3_R_CTRL_ = crate::Reg<r8_uep3_r_ctrl_::R8_UEP3_R_CTRL__SPEC>;
#[doc = "endpoint 3 control"]
pub mod r8_uep3_r_ctrl_;
#[doc = "R8_UEP4_T_LEN (rw) register accessor: endpoint 4 transmittal length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep4_t_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep4_t_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep4_t_len`]
module"]
pub type R8_UEP4_T_LEN = crate::Reg<r8_uep4_t_len::R8_UEP4_T_LEN_SPEC>;
#[doc = "endpoint 4 transmittal length"]
pub mod r8_uep4_t_len;
#[doc = "R8_UEP4_T_CTRL (rw) register accessor: endpoint 4 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep4_t_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep4_t_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep4_t_ctrl`]
module"]
pub type R8_UEP4_T_CTRL = crate::Reg<r8_uep4_t_ctrl::R8_UEP4_T_CTRL_SPEC>;
#[doc = "endpoint 4 control"]
pub mod r8_uep4_t_ctrl;
#[doc = "R8_UEP4_R_CTRL_ (rw) register accessor: endpoint 4 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep4_r_ctrl_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep4_r_ctrl_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep4_r_ctrl_`]
module"]
pub type R8_UEP4_R_CTRL_ = crate::Reg<r8_uep4_r_ctrl_::R8_UEP4_R_CTRL__SPEC>;
#[doc = "endpoint 4 control"]
pub mod r8_uep4_r_ctrl_;
#[doc = "R8_UEP5_T_LEN (rw) register accessor: endpoint 5 transmittal length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep5_t_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep5_t_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep5_t_len`]
module"]
pub type R8_UEP5_T_LEN = crate::Reg<r8_uep5_t_len::R8_UEP5_T_LEN_SPEC>;
#[doc = "endpoint 5 transmittal length"]
pub mod r8_uep5_t_len;
#[doc = "R8_UEP5_T_CTRL (rw) register accessor: endpoint 5 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep5_t_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep5_t_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep5_t_ctrl`]
module"]
pub type R8_UEP5_T_CTRL = crate::Reg<r8_uep5_t_ctrl::R8_UEP5_T_CTRL_SPEC>;
#[doc = "endpoint 5 control"]
pub mod r8_uep5_t_ctrl;
#[doc = "R8_UEP5_R_CTRL_ (rw) register accessor: endpoint 5 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep5_r_ctrl_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep5_r_ctrl_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep5_r_ctrl_`]
module"]
pub type R8_UEP5_R_CTRL_ = crate::Reg<r8_uep5_r_ctrl_::R8_UEP5_R_CTRL__SPEC>;
#[doc = "endpoint 5 control"]
pub mod r8_uep5_r_ctrl_;
#[doc = "R8_UEP6_T_LEN (rw) register accessor: endpoint 6 transmittal length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep6_t_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep6_t_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep6_t_len`]
module"]
pub type R8_UEP6_T_LEN = crate::Reg<r8_uep6_t_len::R8_UEP6_T_LEN_SPEC>;
#[doc = "endpoint 6 transmittal length"]
pub mod r8_uep6_t_len;
#[doc = "R8_UEP6_T_CTRL (rw) register accessor: endpoint 6 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep6_t_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep6_t_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep6_t_ctrl`]
module"]
pub type R8_UEP6_T_CTRL = crate::Reg<r8_uep6_t_ctrl::R8_UEP6_T_CTRL_SPEC>;
#[doc = "endpoint 6 control"]
pub mod r8_uep6_t_ctrl;
#[doc = "R8_UEP6_R_CTRL_ (rw) register accessor: endpoint 6 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep6_r_ctrl_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep6_r_ctrl_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep6_r_ctrl_`]
module"]
pub type R8_UEP6_R_CTRL_ = crate::Reg<r8_uep6_r_ctrl_::R8_UEP6_R_CTRL__SPEC>;
#[doc = "endpoint 6 control"]
pub mod r8_uep6_r_ctrl_;
#[doc = "R8_UEP7_T_LEN (rw) register accessor: endpoint 7 transmittal length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep7_t_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep7_t_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep7_t_len`]
module"]
pub type R8_UEP7_T_LEN = crate::Reg<r8_uep7_t_len::R8_UEP7_T_LEN_SPEC>;
#[doc = "endpoint 7 transmittal length"]
pub mod r8_uep7_t_len;
#[doc = "R8_UEP7_T_CTRL (rw) register accessor: endpoint 7 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep7_t_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep7_t_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep7_t_ctrl`]
module"]
pub type R8_UEP7_T_CTRL = crate::Reg<r8_uep7_t_ctrl::R8_UEP7_T_CTRL_SPEC>;
#[doc = "endpoint 7 control"]
pub mod r8_uep7_t_ctrl;
#[doc = "R8_UEP7_R_CTRL_ (rw) register accessor: endpoint 7 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep7_r_ctrl_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep7_r_ctrl_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r8_uep7_r_ctrl_`]
module"]
pub type R8_UEP7_R_CTRL_ = crate::Reg<r8_uep7_r_ctrl_::R8_UEP7_R_CTRL__SPEC>;
#[doc = "endpoint 7 control"]
pub mod r8_uep7_r_ctrl_;
#[doc = "USB_OTG_CR (rw) register accessor: usb otg control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_otg_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_otg_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_otg_cr`]
module"]
pub type USB_OTG_CR = crate::Reg<usb_otg_cr::USB_OTG_CR_SPEC>;
#[doc = "usb otg control"]
pub mod usb_otg_cr;
#[doc = "USB_OTG_SR (rw) register accessor: usb otg status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_otg_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_otg_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_otg_sr`]
module"]
pub type USB_OTG_SR = crate::Reg<usb_otg_sr::USB_OTG_SR_SPEC>;
#[doc = "usb otg status"]
pub mod usb_otg_sr;
