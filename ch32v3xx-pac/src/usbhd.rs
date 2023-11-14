#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB base control"]
    pub usb_ctrl: USB_CTRL,
    #[doc = "0x01 - USB HOST control"]
    pub uhost_ctrl: UHOST_CTRL,
    #[doc = "0x02 - USB interrupt enable"]
    pub usb_int_en: USB_INT_EN,
    #[doc = "0x03 - USB device address"]
    pub usb_dev_ad: USB_DEV_AD,
    #[doc = "0x04 - USB_FRAME_NO"]
    pub usb_frame_no: USB_FRAME_NO,
    #[doc = "0x06 - indicate USB suspend status"]
    pub usb_usb_suspend: USB_USB_SUSPEND,
    _reserved6: [u8; 0x01],
    #[doc = "0x08 - USB_SPEED_TYPE"]
    pub usb_speed_type: USB_SPEED_TYPE,
    #[doc = "0x09 - USB miscellaneous status"]
    pub usb_mis_st: USB_MIS_ST,
    #[doc = "0x0a - USB interrupt flag"]
    pub usb_int_fg: USB_INT_FG,
    #[doc = "0x0b - USB interrupt status"]
    pub usb_int_st: USB_INT_ST,
    #[doc = "0x0c - USB receiving length"]
    pub usb_rx_len: USB_RX_LEN,
    _reserved11: [u8; 0x02],
    #[doc = "0x10 - USB endpoint configuration"]
    pub uep_config: UEP_CONFIG,
    #[doc = "0x14 - USB endpoint type"]
    pub uep_type: UEP_TYPE,
    #[doc = "0x18 - USB endpoint buffer mode"]
    pub uep_buf_mod: UEP_BUF_MOD,
    #[doc = "0x1c - B endpoint 0 DMA buffer address"]
    pub uep0_dma: UEP0_DMA,
    _reserved15: [u8; 0x02],
    #[doc = "0x20 - endpoint 1 DMA RX buffer address"]
    pub uep1_rx_dma: UEP1_RX_DMA,
    _reserved16: [u8; 0x02],
    #[doc = "0x24 - endpoint 2 DMA RX buffer address/UH_RX_DMA"]
    pub uep2_rx_dma__uh_rx_dma: UEP2_RX_DMA__UH_RX_DMA,
    _reserved17: [u8; 0x02],
    #[doc = "0x28 - endpoint 3 DMA RX buffer address"]
    pub uep3_rx_dma: UEP3_RX_DMA,
    _reserved18: [u8; 0x02],
    #[doc = "0x2c - endpoint 4 DMA RX buffer address"]
    pub uep4_rx_dma: UEP4_RX_DMA,
    _reserved19: [u8; 0x02],
    #[doc = "0x30 - endpoint 5 DMA RX buffer address"]
    pub uep5_rx_dma: UEP5_RX_DMA,
    _reserved20: [u8; 0x02],
    #[doc = "0x34 - endpoint 6 DMA RX buffer address"]
    pub uep6_rx_dma: UEP6_RX_DMA,
    _reserved21: [u8; 0x02],
    #[doc = "0x38 - endpoint 7 DMA RX buffer address"]
    pub uep7_rx_dma: UEP7_RX_DMA,
    _reserved22: [u8; 0x02],
    #[doc = "0x3c - endpoint 8 DMA RX buffer address"]
    pub uep8_rx_dma: UEP8_RX_DMA,
    _reserved23: [u8; 0x02],
    #[doc = "0x40 - endpoint 9 DMA RX buffer address"]
    pub uep9_rx_dma: UEP9_RX_DMA,
    _reserved24: [u8; 0x02],
    #[doc = "0x44 - endpoint 10 DMA RX buffer address"]
    pub uep10_rx_dma: UEP10_RX_DMA,
    _reserved25: [u8; 0x02],
    #[doc = "0x48 - endpoint 11 DMA RX buffer address"]
    pub uep11_rx_dma: UEP11_RX_DMA,
    _reserved26: [u8; 0x02],
    #[doc = "0x4c - endpoint 12 DMA RX buffer address"]
    pub uep12_rx_dma: UEP12_RX_DMA,
    _reserved27: [u8; 0x02],
    #[doc = "0x50 - endpoint 13 DMA RX buffer address"]
    pub uep13_rx_dma: UEP13_RX_DMA,
    _reserved28: [u8; 0x02],
    #[doc = "0x54 - endpoint 14 DMA RX buffer address"]
    pub uep14_rx_dma: UEP14_RX_DMA,
    _reserved29: [u8; 0x02],
    #[doc = "0x58 - endpoint 15 DMA RX buffer address"]
    pub uep15_rx_dma: UEP15_RX_DMA,
    _reserved30: [u8; 0x02],
    #[doc = "0x5c - endpoint 1 DMA TX buffer address"]
    pub uep1_tx_dma: UEP1_TX_DMA,
    _reserved31: [u8; 0x02],
    #[doc = "0x60 - endpoint 2 DMA TX buffer address"]
    pub uep2_tx_dma: UEP2_TX_DMA,
    _reserved32: [u8; 0x02],
    #[doc = "0x64 - endpoint 3 DMA TX buffer address"]
    pub uep3_tx_dma__uh_tx_dma: UEP3_TX_DMA__UH_TX_DMA,
    _reserved33: [u8; 0x02],
    #[doc = "0x68 - endpoint 4 DMA TX buffer address"]
    pub uep4_tx_dma: UEP4_TX_DMA,
    _reserved34: [u8; 0x02],
    #[doc = "0x6c - endpoint 5 DMA TX buffer address"]
    pub uep5_tx_dma: UEP5_TX_DMA,
    _reserved35: [u8; 0x02],
    #[doc = "0x70 - endpoint 6 DMA TX buffer address"]
    pub uep6_tx_dma: UEP6_TX_DMA,
    _reserved36: [u8; 0x02],
    #[doc = "0x74 - endpoint 7 DMA TX buffer address"]
    pub uep7_tx_dma: UEP7_TX_DMA,
    _reserved37: [u8; 0x02],
    #[doc = "0x78 - endpoint 8 DMA TX buffer address"]
    pub uep8_tx_dma: UEP8_TX_DMA,
    _reserved38: [u8; 0x02],
    #[doc = "0x7c - endpoint 9 DMA TX buffer address"]
    pub uep9_tx_dma: UEP9_TX_DMA,
    _reserved39: [u8; 0x02],
    #[doc = "0x80 - endpoint 10 DMA TX buffer address"]
    pub uep10_tx_dma: UEP10_TX_DMA,
    _reserved40: [u8; 0x02],
    #[doc = "0x84 - endpoint 11 DMA TX buffer address"]
    pub uep11_tx_dma: UEP11_TX_DMA,
    _reserved41: [u8; 0x02],
    #[doc = "0x88 - endpoint 12 DMA TX buffer address"]
    pub uep12_tx_dma____uh_split_data: UEP12_TX_DMA____UH_SPLIT_DATA,
    _reserved42: [u8; 0x02],
    #[doc = "0x8c - endpoint 13 DMA TX buffer address"]
    pub uep13_tx_dma: UEP13_TX_DMA,
    _reserved43: [u8; 0x02],
    #[doc = "0x90 - endpoint 14 DMA TX buffer address"]
    pub uep14_tx_dma: UEP14_TX_DMA,
    _reserved44: [u8; 0x02],
    #[doc = "0x94 - endpoint 15 DMA TX buffer address"]
    pub uep15_tx_dma: UEP15_TX_DMA,
    _reserved45: [u8; 0x02],
    #[doc = "0x98 - endpoint 0 max acceptable length"]
    pub uep0_max_len: UEP0_MAX_LEN,
    _reserved46: [u8; 0x02],
    #[doc = "0x9c - endpoint 1 max acceptable length"]
    pub uep1_max_len: UEP1_MAX_LEN,
    _reserved47: [u8; 0x02],
    #[doc = "0xa0 - endpoint 2 max acceptable length"]
    pub uep2_max_len__uh_rx_max_len: UEP2_MAX_LEN__UH_RX_MAX_LEN,
    _reserved48: [u8; 0x02],
    #[doc = "0xa4 - endpoint 3 MAX_LEN TX"]
    pub uep3_max_len: UEP3_MAX_LEN,
    _reserved49: [u8; 0x02],
    #[doc = "0xa8 - endpoint 4 max acceptable length"]
    pub uep4_max_len: UEP4_MAX_LEN,
    _reserved50: [u8; 0x02],
    #[doc = "0xac - endpoint 5 max acceptable length"]
    pub uep5_max_len: UEP5_MAX_LEN,
    _reserved51: [u8; 0x02],
    #[doc = "0xb0 - endpoint 6 max acceptable length"]
    pub uep6_max_len: UEP6_MAX_LEN,
    _reserved52: [u8; 0x02],
    #[doc = "0xb4 - endpoint 7 max acceptable length"]
    pub uep7_max_len: UEP7_MAX_LEN,
    _reserved53: [u8; 0x02],
    #[doc = "0xb8 - endpoint 8 max acceptable length"]
    pub uep8_max_len: UEP8_MAX_LEN,
    _reserved54: [u8; 0x02],
    #[doc = "0xbc - endpoint 9 max acceptable length"]
    pub uep9_max_len: UEP9_MAX_LEN,
    _reserved55: [u8; 0x02],
    #[doc = "0xc0 - endpoint 10 max acceptable length"]
    pub uep10_max_len: UEP10_MAX_LEN,
    _reserved56: [u8; 0x02],
    #[doc = "0xc4 - endpoint 11 max acceptable length"]
    pub uep11_max_len: UEP11_MAX_LEN,
    _reserved57: [u8; 0x02],
    #[doc = "0xc8 - endpoint 12 max acceptable length"]
    pub uep12_max_len: UEP12_MAX_LEN,
    _reserved58: [u8; 0x02],
    #[doc = "0xcc - endpoint 13 max acceptable length"]
    pub uep13_max_len: UEP13_MAX_LEN,
    _reserved59: [u8; 0x02],
    #[doc = "0xd0 - endpoint 14 max acceptable length"]
    pub uep14_max_len: UEP14_MAX_LEN,
    _reserved60: [u8; 0x02],
    #[doc = "0xd4 - endpoint 15 max acceptable length"]
    pub uep15_max_len: UEP15_MAX_LEN,
    _reserved61: [u8; 0x02],
    #[doc = "0xd8 - endpoint 0 send the length"]
    pub uep0_t_len: UEP0_T_LEN,
    #[doc = "0xda - endpoint 0 send control"]
    pub uep0_t_ctrl: UEP0_T_CTRL,
    #[doc = "0xdb - endpoint 0 send control"]
    pub uep0_r_ctrl: UEP0_R_CTRL,
    #[doc = "0xdc - endpoint 1 send the length"]
    pub uep1_t_len: UEP1_T_LEN,
    #[doc = "0xde - endpoint 1 send control"]
    pub uep1_t_ctrl: UEP1_T_CTRL,
    #[doc = "0xdf - endpoint 1 send control"]
    pub uep1_r_ctrl: UEP1_R_CTRL,
    #[doc = "0xe0 - endpoint 2 send the length"]
    pub uep2_t_len__uh_ep_pid: UEP2_T_LEN__UH_EP_PID,
    #[doc = "0xe2 - endpoint 2 send control"]
    pub uep2_t_ctrl: UEP2_T_CTRL,
    #[doc = "0xe3 - endpoint 2 send control"]
    pub uep2_r_ctrl__uh_rx_ctrl: UEP2_R_CTRL__UH_RX_CTRL,
    #[doc = "0xe4 - endpoint 3 send the length"]
    pub uep3_t_len___uh_tx_len_h: UEP3_T_LEN___UH_TX_LEN_H,
    #[doc = "0xe6 - endpoint 3 send control"]
    pub uep3_t_ctrl___uh_tx_ctrl: UEP3_T_CTRL___UH_TX_CTRL,
    #[doc = "0xe7 - endpoint 3 send control"]
    pub uep3_r_ctrl: UEP3_R_CTRL,
    #[doc = "0xe8 - endpoint 4 send the length"]
    pub uep4_t_len: UEP4_T_LEN,
    #[doc = "0xea - endpoint 4 send control"]
    pub uep4_t_ctrl: UEP4_T_CTRL,
    #[doc = "0xeb - endpoint 4 send control"]
    pub uep4_r_ctrl: UEP4_R_CTRL,
    #[doc = "0xec - endpoint 5 send the length"]
    pub uep5_t_len: UEP5_T_LEN,
    #[doc = "0xee - endpoint 5 send control"]
    pub uep5_t_ctrl: UEP5_T_CTRL,
    #[doc = "0xef - endpoint 5 send control"]
    pub uep5_r_ctrl: UEP5_R_CTRL,
    #[doc = "0xf0 - endpoint 6 send the length"]
    pub uep6_t_len: UEP6_T_LEN,
    #[doc = "0xf2 - endpoint 6 send control"]
    pub uep6_t_ctrl: UEP6_T_CTRL,
    #[doc = "0xf3 - endpoint 6 send control"]
    pub uep6_r_ctrl: UEP6_R_CTRL,
    #[doc = "0xf4 - endpoint 7 send the length"]
    pub uep7_t_len: UEP7_T_LEN,
    #[doc = "0xf6 - endpoint 7 send control"]
    pub uep7_t_ctrl: UEP7_T_CTRL,
    #[doc = "0xf7 - endpoint 7 send control"]
    pub uep7_r_ctrl: UEP7_R_CTRL,
    #[doc = "0xf8 - endpoint 8 send the length"]
    pub uep8_t_len: UEP8_T_LEN,
    #[doc = "0xfa - endpoint 8 send control"]
    pub uep8_t_ctrl: UEP8_T_CTRL,
    #[doc = "0xfb - endpoint 8 send control"]
    pub uep8_r_ctrl: UEP8_R_CTRL,
    #[doc = "0xfc - endpoint9 send the length"]
    pub uep9_t_len: UEP9_T_LEN,
    #[doc = "0xfe - endpoint 9 send control"]
    pub uep9_t_ctrl: UEP9_T_CTRL,
    #[doc = "0xff - endpoint 9 send control"]
    pub uep9_r_ctrl: UEP9_R_CTRL,
    #[doc = "0x100 - endpoint 10 send the length"]
    pub uep10_t_len: UEP10_T_LEN,
    #[doc = "0x102 - endpoint 10 send control"]
    pub uep10_t_ctrl: UEP10_T_CTRL,
    #[doc = "0x103 - endpoint 10 send control"]
    pub uep10_r_ctrl: UEP10_R_CTRL,
    #[doc = "0x104 - endpoint 11 send the length"]
    pub uep11_t_len: UEP11_T_LEN,
    #[doc = "0x106 - endpoint 11 send control"]
    pub uep11_t_ctrl: UEP11_T_CTRL,
    #[doc = "0x107 - endpoint 11 send control"]
    pub uep11_r_ctrl: UEP11_R_CTRL,
    #[doc = "0x108 - endpoint 12 send the length"]
    pub uep12_t_len: UEP12_T_LEN,
    #[doc = "0x10a - endpoint 12 send control"]
    pub uep12_t_ctrl: UEP12_T_CTRL,
    #[doc = "0x10b - endpoint 12 send control"]
    pub uep12_r_ctrl: UEP12_R_CTRL,
    #[doc = "0x10c - endpoint 13 send the length"]
    pub uep13_t_len: UEP13_T_LEN,
    #[doc = "0x10e - endpoint 13 send control"]
    pub uep13_t_ctrl: UEP13_T_CTRL,
    #[doc = "0x10f - endpoint 13 send control"]
    pub uep13_r_ctrl: UEP13_R_CTRL,
    #[doc = "0x110 - endpoint 14 send the length"]
    pub uep14_t_len: UEP14_T_LEN,
    #[doc = "0x112 - endpoint 14 send control"]
    pub uep14_t_ctrl: UEP14_T_CTRL,
    #[doc = "0x113 - endpoint 14 send control"]
    pub uep14_r_ctrl: UEP14_R_CTRL,
    #[doc = "0x114 - endpoint 15 send the length"]
    pub uep15_t_len: UEP15_T_LEN,
    #[doc = "0x116 - endpoint 15 send control"]
    pub uep15_t_ctrl: UEP15_T_CTRL,
    #[doc = "0x117 - endpoint 15 send control"]
    pub uep15_r_ctrl: UEP15_R_CTRL,
}
#[doc = "USB_CTRL (rw) register accessor: USB base control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_ctrl`]
module"]
pub type USB_CTRL = crate::Reg<usb_ctrl::USB_CTRL_SPEC>;
#[doc = "USB base control"]
pub mod usb_ctrl;
#[doc = "UHOST_CTRL (rw) register accessor: USB HOST control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uhost_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhost_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhost_ctrl`]
module"]
pub type UHOST_CTRL = crate::Reg<uhost_ctrl::UHOST_CTRL_SPEC>;
#[doc = "USB HOST control"]
pub mod uhost_ctrl;
#[doc = "USB_INT_EN (rw) register accessor: USB interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_int_en`]
module"]
pub type USB_INT_EN = crate::Reg<usb_int_en::USB_INT_EN_SPEC>;
#[doc = "USB interrupt enable"]
pub mod usb_int_en;
#[doc = "USB_DEV_AD (rw) register accessor: USB device address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_dev_ad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_dev_ad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_dev_ad`]
module"]
pub type USB_DEV_AD = crate::Reg<usb_dev_ad::USB_DEV_AD_SPEC>;
#[doc = "USB device address"]
pub mod usb_dev_ad;
#[doc = "USB_FRAME_NO (r) register accessor: USB_FRAME_NO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_frame_no::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_frame_no`]
module"]
pub type USB_FRAME_NO = crate::Reg<usb_frame_no::USB_FRAME_NO_SPEC>;
#[doc = "USB_FRAME_NO"]
pub mod usb_frame_no;
#[doc = "USB_USB_SUSPEND (rw) register accessor: indicate USB suspend status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_usb_suspend::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_usb_suspend::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_usb_suspend`]
module"]
pub type USB_USB_SUSPEND = crate::Reg<usb_usb_suspend::USB_USB_SUSPEND_SPEC>;
#[doc = "indicate USB suspend status"]
pub mod usb_usb_suspend;
#[doc = "USB_SPEED_TYPE (r) register accessor: USB_SPEED_TYPE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_speed_type::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_speed_type`]
module"]
pub type USB_SPEED_TYPE = crate::Reg<usb_speed_type::USB_SPEED_TYPE_SPEC>;
#[doc = "USB_SPEED_TYPE"]
pub mod usb_speed_type;
#[doc = "USB_MIS_ST (r) register accessor: USB miscellaneous status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_mis_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_mis_st`]
module"]
pub type USB_MIS_ST = crate::Reg<usb_mis_st::USB_MIS_ST_SPEC>;
#[doc = "USB miscellaneous status"]
pub mod usb_mis_st;
#[doc = "USB_INT_FG (rw) register accessor: USB interrupt flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_int_fg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_int_fg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_int_fg`]
module"]
pub type USB_INT_FG = crate::Reg<usb_int_fg::USB_INT_FG_SPEC>;
#[doc = "USB interrupt flag"]
pub mod usb_int_fg;
#[doc = "USB_INT_ST (r) register accessor: USB interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_int_st`]
module"]
pub type USB_INT_ST = crate::Reg<usb_int_st::USB_INT_ST_SPEC>;
#[doc = "USB interrupt status"]
pub mod usb_int_st;
#[doc = "USB_RX_LEN (r) register accessor: USB receiving length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_rx_len::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_rx_len`]
module"]
pub type USB_RX_LEN = crate::Reg<usb_rx_len::USB_RX_LEN_SPEC>;
#[doc = "USB receiving length"]
pub mod usb_rx_len;
#[doc = "UEP_CONFIG (rw) register accessor: USB endpoint configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep_config`]
module"]
pub type UEP_CONFIG = crate::Reg<uep_config::UEP_CONFIG_SPEC>;
#[doc = "USB endpoint configuration"]
pub mod uep_config;
#[doc = "UEP_TYPE (rw) register accessor: USB endpoint type\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep_type::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep_type::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep_type`]
module"]
pub type UEP_TYPE = crate::Reg<uep_type::UEP_TYPE_SPEC>;
#[doc = "USB endpoint type"]
pub mod uep_type;
#[doc = "UEP_BUF_MOD (rw) register accessor: USB endpoint buffer mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep_buf_mod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep_buf_mod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep_buf_mod`]
module"]
pub type UEP_BUF_MOD = crate::Reg<uep_buf_mod::UEP_BUF_MOD_SPEC>;
#[doc = "USB endpoint buffer mode"]
pub mod uep_buf_mod;
#[doc = "UEP0_DMA (rw) register accessor: B endpoint 0 DMA buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep0_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep0_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep0_dma`]
module"]
pub type UEP0_DMA = crate::Reg<uep0_dma::UEP0_DMA_SPEC>;
#[doc = "B endpoint 0 DMA buffer address"]
pub mod uep0_dma;
#[doc = "UEP1_RX_DMA (rw) register accessor: endpoint 1 DMA RX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep1_rx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep1_rx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep1_rx_dma`]
module"]
pub type UEP1_RX_DMA = crate::Reg<uep1_rx_dma::UEP1_RX_DMA_SPEC>;
#[doc = "endpoint 1 DMA RX buffer address"]
pub mod uep1_rx_dma;
#[doc = "UEP2_RX_DMA__UH_RX_DMA (rw) register accessor: endpoint 2 DMA RX buffer address/UH_RX_DMA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep2_rx_dma__uh_rx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep2_rx_dma__uh_rx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep2_rx_dma__uh_rx_dma`]
module"]
pub type UEP2_RX_DMA__UH_RX_DMA = crate::Reg<uep2_rx_dma__uh_rx_dma::UEP2_RX_DMA__UH_RX_DMA_SPEC>;
#[doc = "endpoint 2 DMA RX buffer address/UH_RX_DMA"]
pub mod uep2_rx_dma__uh_rx_dma;
#[doc = "UEP3_RX_DMA (rw) register accessor: endpoint 3 DMA RX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep3_rx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep3_rx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep3_rx_dma`]
module"]
pub type UEP3_RX_DMA = crate::Reg<uep3_rx_dma::UEP3_RX_DMA_SPEC>;
#[doc = "endpoint 3 DMA RX buffer address"]
pub mod uep3_rx_dma;
#[doc = "UEP4_RX_DMA (rw) register accessor: endpoint 4 DMA RX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep4_rx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep4_rx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep4_rx_dma`]
module"]
pub type UEP4_RX_DMA = crate::Reg<uep4_rx_dma::UEP4_RX_DMA_SPEC>;
#[doc = "endpoint 4 DMA RX buffer address"]
pub mod uep4_rx_dma;
#[doc = "UEP5_RX_DMA (rw) register accessor: endpoint 5 DMA RX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep5_rx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep5_rx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep5_rx_dma`]
module"]
pub type UEP5_RX_DMA = crate::Reg<uep5_rx_dma::UEP5_RX_DMA_SPEC>;
#[doc = "endpoint 5 DMA RX buffer address"]
pub mod uep5_rx_dma;
#[doc = "UEP6_RX_DMA (rw) register accessor: endpoint 6 DMA RX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep6_rx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep6_rx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep6_rx_dma`]
module"]
pub type UEP6_RX_DMA = crate::Reg<uep6_rx_dma::UEP6_RX_DMA_SPEC>;
#[doc = "endpoint 6 DMA RX buffer address"]
pub mod uep6_rx_dma;
#[doc = "UEP7_RX_DMA (rw) register accessor: endpoint 7 DMA RX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep7_rx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep7_rx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep7_rx_dma`]
module"]
pub type UEP7_RX_DMA = crate::Reg<uep7_rx_dma::UEP7_RX_DMA_SPEC>;
#[doc = "endpoint 7 DMA RX buffer address"]
pub mod uep7_rx_dma;
#[doc = "UEP8_RX_DMA (rw) register accessor: endpoint 8 DMA RX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep8_rx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep8_rx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep8_rx_dma`]
module"]
pub type UEP8_RX_DMA = crate::Reg<uep8_rx_dma::UEP8_RX_DMA_SPEC>;
#[doc = "endpoint 8 DMA RX buffer address"]
pub mod uep8_rx_dma;
#[doc = "UEP9_RX_DMA (rw) register accessor: endpoint 9 DMA RX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep9_rx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep9_rx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep9_rx_dma`]
module"]
pub type UEP9_RX_DMA = crate::Reg<uep9_rx_dma::UEP9_RX_DMA_SPEC>;
#[doc = "endpoint 9 DMA RX buffer address"]
pub mod uep9_rx_dma;
#[doc = "UEP10_RX_DMA (rw) register accessor: endpoint 10 DMA RX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep10_rx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep10_rx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep10_rx_dma`]
module"]
pub type UEP10_RX_DMA = crate::Reg<uep10_rx_dma::UEP10_RX_DMA_SPEC>;
#[doc = "endpoint 10 DMA RX buffer address"]
pub mod uep10_rx_dma;
#[doc = "UEP11_RX_DMA (rw) register accessor: endpoint 11 DMA RX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep11_rx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep11_rx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep11_rx_dma`]
module"]
pub type UEP11_RX_DMA = crate::Reg<uep11_rx_dma::UEP11_RX_DMA_SPEC>;
#[doc = "endpoint 11 DMA RX buffer address"]
pub mod uep11_rx_dma;
#[doc = "UEP12_RX_DMA (rw) register accessor: endpoint 12 DMA RX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep12_rx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep12_rx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep12_rx_dma`]
module"]
pub type UEP12_RX_DMA = crate::Reg<uep12_rx_dma::UEP12_RX_DMA_SPEC>;
#[doc = "endpoint 12 DMA RX buffer address"]
pub mod uep12_rx_dma;
#[doc = "UEP13_RX_DMA (rw) register accessor: endpoint 13 DMA RX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep13_rx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep13_rx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep13_rx_dma`]
module"]
pub type UEP13_RX_DMA = crate::Reg<uep13_rx_dma::UEP13_RX_DMA_SPEC>;
#[doc = "endpoint 13 DMA RX buffer address"]
pub mod uep13_rx_dma;
#[doc = "UEP14_RX_DMA (rw) register accessor: endpoint 14 DMA RX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep14_rx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep14_rx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep14_rx_dma`]
module"]
pub type UEP14_RX_DMA = crate::Reg<uep14_rx_dma::UEP14_RX_DMA_SPEC>;
#[doc = "endpoint 14 DMA RX buffer address"]
pub mod uep14_rx_dma;
#[doc = "UEP15_RX_DMA (rw) register accessor: endpoint 15 DMA RX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep15_rx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep15_rx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep15_rx_dma`]
module"]
pub type UEP15_RX_DMA = crate::Reg<uep15_rx_dma::UEP15_RX_DMA_SPEC>;
#[doc = "endpoint 15 DMA RX buffer address"]
pub mod uep15_rx_dma;
#[doc = "UEP1_TX_DMA (rw) register accessor: endpoint 1 DMA TX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep1_tx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep1_tx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep1_tx_dma`]
module"]
pub type UEP1_TX_DMA = crate::Reg<uep1_tx_dma::UEP1_TX_DMA_SPEC>;
#[doc = "endpoint 1 DMA TX buffer address"]
pub mod uep1_tx_dma;
#[doc = "UEP2_TX_DMA (rw) register accessor: endpoint 2 DMA TX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep2_tx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep2_tx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep2_tx_dma`]
module"]
pub type UEP2_TX_DMA = crate::Reg<uep2_tx_dma::UEP2_TX_DMA_SPEC>;
#[doc = "endpoint 2 DMA TX buffer address"]
pub mod uep2_tx_dma;
#[doc = "UEP3_TX_DMA__UH_TX_DMA (rw) register accessor: endpoint 3 DMA TX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep3_tx_dma__uh_tx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep3_tx_dma__uh_tx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep3_tx_dma__uh_tx_dma`]
module"]
pub type UEP3_TX_DMA__UH_TX_DMA = crate::Reg<uep3_tx_dma__uh_tx_dma::UEP3_TX_DMA__UH_TX_DMA_SPEC>;
#[doc = "endpoint 3 DMA TX buffer address"]
pub mod uep3_tx_dma__uh_tx_dma;
#[doc = "UEP4_TX_DMA (rw) register accessor: endpoint 4 DMA TX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep4_tx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep4_tx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep4_tx_dma`]
module"]
pub type UEP4_TX_DMA = crate::Reg<uep4_tx_dma::UEP4_TX_DMA_SPEC>;
#[doc = "endpoint 4 DMA TX buffer address"]
pub mod uep4_tx_dma;
#[doc = "UEP5_TX_DMA (rw) register accessor: endpoint 5 DMA TX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep5_tx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep5_tx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep5_tx_dma`]
module"]
pub type UEP5_TX_DMA = crate::Reg<uep5_tx_dma::UEP5_TX_DMA_SPEC>;
#[doc = "endpoint 5 DMA TX buffer address"]
pub mod uep5_tx_dma;
#[doc = "UEP6_TX_DMA (rw) register accessor: endpoint 6 DMA TX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep6_tx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep6_tx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep6_tx_dma`]
module"]
pub type UEP6_TX_DMA = crate::Reg<uep6_tx_dma::UEP6_TX_DMA_SPEC>;
#[doc = "endpoint 6 DMA TX buffer address"]
pub mod uep6_tx_dma;
#[doc = "UEP7_TX_DMA (rw) register accessor: endpoint 7 DMA TX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep7_tx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep7_tx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep7_tx_dma`]
module"]
pub type UEP7_TX_DMA = crate::Reg<uep7_tx_dma::UEP7_TX_DMA_SPEC>;
#[doc = "endpoint 7 DMA TX buffer address"]
pub mod uep7_tx_dma;
#[doc = "UEP8_TX_DMA (rw) register accessor: endpoint 8 DMA TX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep8_tx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep8_tx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep8_tx_dma`]
module"]
pub type UEP8_TX_DMA = crate::Reg<uep8_tx_dma::UEP8_TX_DMA_SPEC>;
#[doc = "endpoint 8 DMA TX buffer address"]
pub mod uep8_tx_dma;
#[doc = "UEP9_TX_DMA (rw) register accessor: endpoint 9 DMA TX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep9_tx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep9_tx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep9_tx_dma`]
module"]
pub type UEP9_TX_DMA = crate::Reg<uep9_tx_dma::UEP9_TX_DMA_SPEC>;
#[doc = "endpoint 9 DMA TX buffer address"]
pub mod uep9_tx_dma;
#[doc = "UEP10_TX_DMA (rw) register accessor: endpoint 10 DMA TX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep10_tx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep10_tx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep10_tx_dma`]
module"]
pub type UEP10_TX_DMA = crate::Reg<uep10_tx_dma::UEP10_TX_DMA_SPEC>;
#[doc = "endpoint 10 DMA TX buffer address"]
pub mod uep10_tx_dma;
#[doc = "UEP11_TX_DMA (rw) register accessor: endpoint 11 DMA TX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep11_tx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep11_tx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep11_tx_dma`]
module"]
pub type UEP11_TX_DMA = crate::Reg<uep11_tx_dma::UEP11_TX_DMA_SPEC>;
#[doc = "endpoint 11 DMA TX buffer address"]
pub mod uep11_tx_dma;
#[doc = "UEP12_TX_DMA____UH_SPLIT_DATA (rw) register accessor: endpoint 12 DMA TX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep12_tx_dma____uh_split_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep12_tx_dma____uh_split_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep12_tx_dma____uh_split_data`]
module"]
pub type UEP12_TX_DMA____UH_SPLIT_DATA =
    crate::Reg<uep12_tx_dma____uh_split_data::UEP12_TX_DMA____UH_SPLIT_DATA_SPEC>;
#[doc = "endpoint 12 DMA TX buffer address"]
pub mod uep12_tx_dma____uh_split_data;
#[doc = "UEP13_TX_DMA (rw) register accessor: endpoint 13 DMA TX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep13_tx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep13_tx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep13_tx_dma`]
module"]
pub type UEP13_TX_DMA = crate::Reg<uep13_tx_dma::UEP13_TX_DMA_SPEC>;
#[doc = "endpoint 13 DMA TX buffer address"]
pub mod uep13_tx_dma;
#[doc = "UEP14_TX_DMA (rw) register accessor: endpoint 14 DMA TX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep14_tx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep14_tx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep14_tx_dma`]
module"]
pub type UEP14_TX_DMA = crate::Reg<uep14_tx_dma::UEP14_TX_DMA_SPEC>;
#[doc = "endpoint 14 DMA TX buffer address"]
pub mod uep14_tx_dma;
#[doc = "UEP15_TX_DMA (rw) register accessor: endpoint 15 DMA TX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep15_tx_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep15_tx_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep15_tx_dma`]
module"]
pub type UEP15_TX_DMA = crate::Reg<uep15_tx_dma::UEP15_TX_DMA_SPEC>;
#[doc = "endpoint 15 DMA TX buffer address"]
pub mod uep15_tx_dma;
#[doc = "UEP0_MAX_LEN (rw) register accessor: endpoint 0 max acceptable length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep0_max_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep0_max_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep0_max_len`]
module"]
pub type UEP0_MAX_LEN = crate::Reg<uep0_max_len::UEP0_MAX_LEN_SPEC>;
#[doc = "endpoint 0 max acceptable length"]
pub mod uep0_max_len;
#[doc = "UEP1_MAX_LEN (rw) register accessor: endpoint 1 max acceptable length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep1_max_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep1_max_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep1_max_len`]
module"]
pub type UEP1_MAX_LEN = crate::Reg<uep1_max_len::UEP1_MAX_LEN_SPEC>;
#[doc = "endpoint 1 max acceptable length"]
pub mod uep1_max_len;
#[doc = "UEP2_MAX_LEN__UH_RX_MAX_LEN (rw) register accessor: endpoint 2 max acceptable length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep2_max_len__uh_rx_max_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep2_max_len__uh_rx_max_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep2_max_len__uh_rx_max_len`]
module"]
pub type UEP2_MAX_LEN__UH_RX_MAX_LEN =
    crate::Reg<uep2_max_len__uh_rx_max_len::UEP2_MAX_LEN__UH_RX_MAX_LEN_SPEC>;
#[doc = "endpoint 2 max acceptable length"]
pub mod uep2_max_len__uh_rx_max_len;
#[doc = "UEP3_MAX_LEN (rw) register accessor: endpoint 3 MAX_LEN TX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep3_max_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep3_max_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep3_max_len`]
module"]
pub type UEP3_MAX_LEN = crate::Reg<uep3_max_len::UEP3_MAX_LEN_SPEC>;
#[doc = "endpoint 3 MAX_LEN TX"]
pub mod uep3_max_len;
#[doc = "UEP4_MAX_LEN (rw) register accessor: endpoint 4 max acceptable length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep4_max_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep4_max_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep4_max_len`]
module"]
pub type UEP4_MAX_LEN = crate::Reg<uep4_max_len::UEP4_MAX_LEN_SPEC>;
#[doc = "endpoint 4 max acceptable length"]
pub mod uep4_max_len;
#[doc = "UEP5_MAX_LEN (rw) register accessor: endpoint 5 max acceptable length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep5_max_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep5_max_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep5_max_len`]
module"]
pub type UEP5_MAX_LEN = crate::Reg<uep5_max_len::UEP5_MAX_LEN_SPEC>;
#[doc = "endpoint 5 max acceptable length"]
pub mod uep5_max_len;
#[doc = "UEP6_MAX_LEN (rw) register accessor: endpoint 6 max acceptable length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep6_max_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep6_max_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep6_max_len`]
module"]
pub type UEP6_MAX_LEN = crate::Reg<uep6_max_len::UEP6_MAX_LEN_SPEC>;
#[doc = "endpoint 6 max acceptable length"]
pub mod uep6_max_len;
#[doc = "UEP7_MAX_LEN (rw) register accessor: endpoint 7 max acceptable length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep7_max_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep7_max_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep7_max_len`]
module"]
pub type UEP7_MAX_LEN = crate::Reg<uep7_max_len::UEP7_MAX_LEN_SPEC>;
#[doc = "endpoint 7 max acceptable length"]
pub mod uep7_max_len;
#[doc = "UEP8_MAX_LEN (rw) register accessor: endpoint 8 max acceptable length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep8_max_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep8_max_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep8_max_len`]
module"]
pub type UEP8_MAX_LEN = crate::Reg<uep8_max_len::UEP8_MAX_LEN_SPEC>;
#[doc = "endpoint 8 max acceptable length"]
pub mod uep8_max_len;
#[doc = "UEP9_MAX_LEN (rw) register accessor: endpoint 9 max acceptable length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep9_max_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep9_max_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep9_max_len`]
module"]
pub type UEP9_MAX_LEN = crate::Reg<uep9_max_len::UEP9_MAX_LEN_SPEC>;
#[doc = "endpoint 9 max acceptable length"]
pub mod uep9_max_len;
#[doc = "UEP10_MAX_LEN (rw) register accessor: endpoint 10 max acceptable length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep10_max_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep10_max_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep10_max_len`]
module"]
pub type UEP10_MAX_LEN = crate::Reg<uep10_max_len::UEP10_MAX_LEN_SPEC>;
#[doc = "endpoint 10 max acceptable length"]
pub mod uep10_max_len;
#[doc = "UEP11_MAX_LEN (rw) register accessor: endpoint 11 max acceptable length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep11_max_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep11_max_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep11_max_len`]
module"]
pub type UEP11_MAX_LEN = crate::Reg<uep11_max_len::UEP11_MAX_LEN_SPEC>;
#[doc = "endpoint 11 max acceptable length"]
pub mod uep11_max_len;
#[doc = "UEP12_MAX_LEN (rw) register accessor: endpoint 12 max acceptable length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep12_max_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep12_max_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep12_max_len`]
module"]
pub type UEP12_MAX_LEN = crate::Reg<uep12_max_len::UEP12_MAX_LEN_SPEC>;
#[doc = "endpoint 12 max acceptable length"]
pub mod uep12_max_len;
#[doc = "UEP13_MAX_LEN (rw) register accessor: endpoint 13 max acceptable length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep13_max_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep13_max_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep13_max_len`]
module"]
pub type UEP13_MAX_LEN = crate::Reg<uep13_max_len::UEP13_MAX_LEN_SPEC>;
#[doc = "endpoint 13 max acceptable length"]
pub mod uep13_max_len;
#[doc = "UEP14_MAX_LEN (rw) register accessor: endpoint 14 max acceptable length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep14_max_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep14_max_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep14_max_len`]
module"]
pub type UEP14_MAX_LEN = crate::Reg<uep14_max_len::UEP14_MAX_LEN_SPEC>;
#[doc = "endpoint 14 max acceptable length"]
pub mod uep14_max_len;
#[doc = "UEP15_MAX_LEN (rw) register accessor: endpoint 15 max acceptable length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep15_max_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep15_max_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep15_max_len`]
module"]
pub type UEP15_MAX_LEN = crate::Reg<uep15_max_len::UEP15_MAX_LEN_SPEC>;
#[doc = "endpoint 15 max acceptable length"]
pub mod uep15_max_len;
#[doc = "UEP0_T_LEN (rw) register accessor: endpoint 0 send the length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep0_t_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep0_t_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep0_t_len`]
module"]
pub type UEP0_T_LEN = crate::Reg<uep0_t_len::UEP0_T_LEN_SPEC>;
#[doc = "endpoint 0 send the length"]
pub mod uep0_t_len;
#[doc = "UEP0_T_CTRL (rw) register accessor: endpoint 0 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep0_t_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep0_t_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep0_t_ctrl`]
module"]
pub type UEP0_T_CTRL = crate::Reg<uep0_t_ctrl::UEP0_T_CTRL_SPEC>;
#[doc = "endpoint 0 send control"]
pub mod uep0_t_ctrl;
#[doc = "UEP0_R_CTRL (rw) register accessor: endpoint 0 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep0_r_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep0_r_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep0_r_ctrl`]
module"]
pub type UEP0_R_CTRL = crate::Reg<uep0_r_ctrl::UEP0_R_CTRL_SPEC>;
#[doc = "endpoint 0 send control"]
pub mod uep0_r_ctrl;
#[doc = "UEP1_T_LEN (rw) register accessor: endpoint 1 send the length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep1_t_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep1_t_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep1_t_len`]
module"]
pub type UEP1_T_LEN = crate::Reg<uep1_t_len::UEP1_T_LEN_SPEC>;
#[doc = "endpoint 1 send the length"]
pub mod uep1_t_len;
#[doc = "UEP1_T_CTRL (rw) register accessor: endpoint 1 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep1_t_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep1_t_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep1_t_ctrl`]
module"]
pub type UEP1_T_CTRL = crate::Reg<uep1_t_ctrl::UEP1_T_CTRL_SPEC>;
#[doc = "endpoint 1 send control"]
pub mod uep1_t_ctrl;
#[doc = "UEP1_R_CTRL (rw) register accessor: endpoint 1 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep1_r_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep1_r_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep1_r_ctrl`]
module"]
pub type UEP1_R_CTRL = crate::Reg<uep1_r_ctrl::UEP1_R_CTRL_SPEC>;
#[doc = "endpoint 1 send control"]
pub mod uep1_r_ctrl;
#[doc = "UEP2_T_LEN__UH_EP_PID (rw) register accessor: endpoint 2 send the length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep2_t_len__uh_ep_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep2_t_len__uh_ep_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep2_t_len__uh_ep_pid`]
module"]
pub type UEP2_T_LEN__UH_EP_PID = crate::Reg<uep2_t_len__uh_ep_pid::UEP2_T_LEN__UH_EP_PID_SPEC>;
#[doc = "endpoint 2 send the length"]
pub mod uep2_t_len__uh_ep_pid;
#[doc = "UEP2_T_CTRL (rw) register accessor: endpoint 2 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep2_t_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep2_t_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep2_t_ctrl`]
module"]
pub type UEP2_T_CTRL = crate::Reg<uep2_t_ctrl::UEP2_T_CTRL_SPEC>;
#[doc = "endpoint 2 send control"]
pub mod uep2_t_ctrl;
#[doc = "UEP2_R_CTRL__UH_RX_CTRL (rw) register accessor: endpoint 2 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep2_r_ctrl__uh_rx_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep2_r_ctrl__uh_rx_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep2_r_ctrl__uh_rx_ctrl`]
module"]
pub type UEP2_R_CTRL__UH_RX_CTRL =
    crate::Reg<uep2_r_ctrl__uh_rx_ctrl::UEP2_R_CTRL__UH_RX_CTRL_SPEC>;
#[doc = "endpoint 2 send control"]
pub mod uep2_r_ctrl__uh_rx_ctrl;
#[doc = "UEP3_T_LEN___UH_TX_LEN_H (rw) register accessor: endpoint 3 send the length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep3_t_len___uh_tx_len_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep3_t_len___uh_tx_len_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep3_t_len___uh_tx_len_h`]
module"]
pub type UEP3_T_LEN___UH_TX_LEN_H =
    crate::Reg<uep3_t_len___uh_tx_len_h::UEP3_T_LEN___UH_TX_LEN_H_SPEC>;
#[doc = "endpoint 3 send the length"]
pub mod uep3_t_len___uh_tx_len_h;
#[doc = "UEP3_T_CTRL___UH_TX_CTRL (rw) register accessor: endpoint 3 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep3_t_ctrl___uh_tx_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep3_t_ctrl___uh_tx_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep3_t_ctrl___uh_tx_ctrl`]
module"]
pub type UEP3_T_CTRL___UH_TX_CTRL =
    crate::Reg<uep3_t_ctrl___uh_tx_ctrl::UEP3_T_CTRL___UH_TX_CTRL_SPEC>;
#[doc = "endpoint 3 send control"]
pub mod uep3_t_ctrl___uh_tx_ctrl;
#[doc = "UEP3_R_CTRL (rw) register accessor: endpoint 3 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep3_r_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep3_r_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep3_r_ctrl`]
module"]
pub type UEP3_R_CTRL = crate::Reg<uep3_r_ctrl::UEP3_R_CTRL_SPEC>;
#[doc = "endpoint 3 send control"]
pub mod uep3_r_ctrl;
#[doc = "UEP4_T_LEN (rw) register accessor: endpoint 4 send the length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep4_t_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep4_t_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep4_t_len`]
module"]
pub type UEP4_T_LEN = crate::Reg<uep4_t_len::UEP4_T_LEN_SPEC>;
#[doc = "endpoint 4 send the length"]
pub mod uep4_t_len;
#[doc = "UEP4_T_CTRL (rw) register accessor: endpoint 4 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep4_t_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep4_t_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep4_t_ctrl`]
module"]
pub type UEP4_T_CTRL = crate::Reg<uep4_t_ctrl::UEP4_T_CTRL_SPEC>;
#[doc = "endpoint 4 send control"]
pub mod uep4_t_ctrl;
#[doc = "UEP4_R_CTRL (rw) register accessor: endpoint 4 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep4_r_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep4_r_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep4_r_ctrl`]
module"]
pub type UEP4_R_CTRL = crate::Reg<uep4_r_ctrl::UEP4_R_CTRL_SPEC>;
#[doc = "endpoint 4 send control"]
pub mod uep4_r_ctrl;
#[doc = "UEP5_T_LEN (rw) register accessor: endpoint 5 send the length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep5_t_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep5_t_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep5_t_len`]
module"]
pub type UEP5_T_LEN = crate::Reg<uep5_t_len::UEP5_T_LEN_SPEC>;
#[doc = "endpoint 5 send the length"]
pub mod uep5_t_len;
#[doc = "UEP5_T_CTRL (rw) register accessor: endpoint 5 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep5_t_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep5_t_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep5_t_ctrl`]
module"]
pub type UEP5_T_CTRL = crate::Reg<uep5_t_ctrl::UEP5_T_CTRL_SPEC>;
#[doc = "endpoint 5 send control"]
pub mod uep5_t_ctrl;
#[doc = "UEP5_R_CTRL (rw) register accessor: endpoint 5 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep5_r_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep5_r_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep5_r_ctrl`]
module"]
pub type UEP5_R_CTRL = crate::Reg<uep5_r_ctrl::UEP5_R_CTRL_SPEC>;
#[doc = "endpoint 5 send control"]
pub mod uep5_r_ctrl;
#[doc = "UEP6_T_LEN (rw) register accessor: endpoint 6 send the length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep6_t_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep6_t_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep6_t_len`]
module"]
pub type UEP6_T_LEN = crate::Reg<uep6_t_len::UEP6_T_LEN_SPEC>;
#[doc = "endpoint 6 send the length"]
pub mod uep6_t_len;
#[doc = "UEP6_T_CTRL (rw) register accessor: endpoint 6 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep6_t_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep6_t_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep6_t_ctrl`]
module"]
pub type UEP6_T_CTRL = crate::Reg<uep6_t_ctrl::UEP6_T_CTRL_SPEC>;
#[doc = "endpoint 6 send control"]
pub mod uep6_t_ctrl;
#[doc = "UEP6_R_CTRL (rw) register accessor: endpoint 6 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep6_r_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep6_r_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep6_r_ctrl`]
module"]
pub type UEP6_R_CTRL = crate::Reg<uep6_r_ctrl::UEP6_R_CTRL_SPEC>;
#[doc = "endpoint 6 send control"]
pub mod uep6_r_ctrl;
#[doc = "UEP7_T_LEN (rw) register accessor: endpoint 7 send the length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep7_t_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep7_t_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep7_t_len`]
module"]
pub type UEP7_T_LEN = crate::Reg<uep7_t_len::UEP7_T_LEN_SPEC>;
#[doc = "endpoint 7 send the length"]
pub mod uep7_t_len;
#[doc = "UEP7_T_CTRL (rw) register accessor: endpoint 7 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep7_t_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep7_t_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep7_t_ctrl`]
module"]
pub type UEP7_T_CTRL = crate::Reg<uep7_t_ctrl::UEP7_T_CTRL_SPEC>;
#[doc = "endpoint 7 send control"]
pub mod uep7_t_ctrl;
#[doc = "UEP7_R_CTRL (rw) register accessor: endpoint 7 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep7_r_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep7_r_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep7_r_ctrl`]
module"]
pub type UEP7_R_CTRL = crate::Reg<uep7_r_ctrl::UEP7_R_CTRL_SPEC>;
#[doc = "endpoint 7 send control"]
pub mod uep7_r_ctrl;
#[doc = "UEP8_T_LEN (rw) register accessor: endpoint 8 send the length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep8_t_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep8_t_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep8_t_len`]
module"]
pub type UEP8_T_LEN = crate::Reg<uep8_t_len::UEP8_T_LEN_SPEC>;
#[doc = "endpoint 8 send the length"]
pub mod uep8_t_len;
#[doc = "UEP8_T_CTRL (rw) register accessor: endpoint 8 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep8_t_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep8_t_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep8_t_ctrl`]
module"]
pub type UEP8_T_CTRL = crate::Reg<uep8_t_ctrl::UEP8_T_CTRL_SPEC>;
#[doc = "endpoint 8 send control"]
pub mod uep8_t_ctrl;
#[doc = "UEP8_R_CTRL (rw) register accessor: endpoint 8 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep8_r_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep8_r_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep8_r_ctrl`]
module"]
pub type UEP8_R_CTRL = crate::Reg<uep8_r_ctrl::UEP8_R_CTRL_SPEC>;
#[doc = "endpoint 8 send control"]
pub mod uep8_r_ctrl;
#[doc = "UEP9_T_LEN (rw) register accessor: endpoint9 send the length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep9_t_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep9_t_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep9_t_len`]
module"]
pub type UEP9_T_LEN = crate::Reg<uep9_t_len::UEP9_T_LEN_SPEC>;
#[doc = "endpoint9 send the length"]
pub mod uep9_t_len;
#[doc = "UEP9_T_CTRL (rw) register accessor: endpoint 9 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep9_t_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep9_t_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep9_t_ctrl`]
module"]
pub type UEP9_T_CTRL = crate::Reg<uep9_t_ctrl::UEP9_T_CTRL_SPEC>;
#[doc = "endpoint 9 send control"]
pub mod uep9_t_ctrl;
#[doc = "UEP9_R_CTRL (rw) register accessor: endpoint 9 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep9_r_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep9_r_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep9_r_ctrl`]
module"]
pub type UEP9_R_CTRL = crate::Reg<uep9_r_ctrl::UEP9_R_CTRL_SPEC>;
#[doc = "endpoint 9 send control"]
pub mod uep9_r_ctrl;
#[doc = "UEP10_T_LEN (rw) register accessor: endpoint 10 send the length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep10_t_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep10_t_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep10_t_len`]
module"]
pub type UEP10_T_LEN = crate::Reg<uep10_t_len::UEP10_T_LEN_SPEC>;
#[doc = "endpoint 10 send the length"]
pub mod uep10_t_len;
#[doc = "UEP10_T_CTRL (rw) register accessor: endpoint 10 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep10_t_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep10_t_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep10_t_ctrl`]
module"]
pub type UEP10_T_CTRL = crate::Reg<uep10_t_ctrl::UEP10_T_CTRL_SPEC>;
#[doc = "endpoint 10 send control"]
pub mod uep10_t_ctrl;
#[doc = "UEP10_R_CTRL (rw) register accessor: endpoint 10 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep10_r_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep10_r_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep10_r_ctrl`]
module"]
pub type UEP10_R_CTRL = crate::Reg<uep10_r_ctrl::UEP10_R_CTRL_SPEC>;
#[doc = "endpoint 10 send control"]
pub mod uep10_r_ctrl;
#[doc = "UEP11_T_LEN (rw) register accessor: endpoint 11 send the length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep11_t_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep11_t_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep11_t_len`]
module"]
pub type UEP11_T_LEN = crate::Reg<uep11_t_len::UEP11_T_LEN_SPEC>;
#[doc = "endpoint 11 send the length"]
pub mod uep11_t_len;
#[doc = "UEP11_T_CTRL (rw) register accessor: endpoint 11 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep11_t_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep11_t_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep11_t_ctrl`]
module"]
pub type UEP11_T_CTRL = crate::Reg<uep11_t_ctrl::UEP11_T_CTRL_SPEC>;
#[doc = "endpoint 11 send control"]
pub mod uep11_t_ctrl;
#[doc = "UEP11_R_CTRL (rw) register accessor: endpoint 11 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep11_r_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep11_r_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep11_r_ctrl`]
module"]
pub type UEP11_R_CTRL = crate::Reg<uep11_r_ctrl::UEP11_R_CTRL_SPEC>;
#[doc = "endpoint 11 send control"]
pub mod uep11_r_ctrl;
#[doc = "UEP12_T_LEN (rw) register accessor: endpoint 12 send the length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep12_t_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep12_t_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep12_t_len`]
module"]
pub type UEP12_T_LEN = crate::Reg<uep12_t_len::UEP12_T_LEN_SPEC>;
#[doc = "endpoint 12 send the length"]
pub mod uep12_t_len;
#[doc = "UEP12_T_CTRL (rw) register accessor: endpoint 12 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep12_t_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep12_t_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep12_t_ctrl`]
module"]
pub type UEP12_T_CTRL = crate::Reg<uep12_t_ctrl::UEP12_T_CTRL_SPEC>;
#[doc = "endpoint 12 send control"]
pub mod uep12_t_ctrl;
#[doc = "UEP12_R_CTRL (rw) register accessor: endpoint 12 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep12_r_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep12_r_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep12_r_ctrl`]
module"]
pub type UEP12_R_CTRL = crate::Reg<uep12_r_ctrl::UEP12_R_CTRL_SPEC>;
#[doc = "endpoint 12 send control"]
pub mod uep12_r_ctrl;
#[doc = "UEP13_T_LEN (rw) register accessor: endpoint 13 send the length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep13_t_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep13_t_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep13_t_len`]
module"]
pub type UEP13_T_LEN = crate::Reg<uep13_t_len::UEP13_T_LEN_SPEC>;
#[doc = "endpoint 13 send the length"]
pub mod uep13_t_len;
#[doc = "UEP13_T_CTRL (rw) register accessor: endpoint 13 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep13_t_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep13_t_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep13_t_ctrl`]
module"]
pub type UEP13_T_CTRL = crate::Reg<uep13_t_ctrl::UEP13_T_CTRL_SPEC>;
#[doc = "endpoint 13 send control"]
pub mod uep13_t_ctrl;
#[doc = "UEP13_R_CTRL (rw) register accessor: endpoint 13 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep13_r_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep13_r_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep13_r_ctrl`]
module"]
pub type UEP13_R_CTRL = crate::Reg<uep13_r_ctrl::UEP13_R_CTRL_SPEC>;
#[doc = "endpoint 13 send control"]
pub mod uep13_r_ctrl;
#[doc = "UEP14_T_LEN (rw) register accessor: endpoint 14 send the length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep14_t_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep14_t_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep14_t_len`]
module"]
pub type UEP14_T_LEN = crate::Reg<uep14_t_len::UEP14_T_LEN_SPEC>;
#[doc = "endpoint 14 send the length"]
pub mod uep14_t_len;
#[doc = "UEP14_T_CTRL (rw) register accessor: endpoint 14 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep14_t_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep14_t_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep14_t_ctrl`]
module"]
pub type UEP14_T_CTRL = crate::Reg<uep14_t_ctrl::UEP14_T_CTRL_SPEC>;
#[doc = "endpoint 14 send control"]
pub mod uep14_t_ctrl;
#[doc = "UEP14_R_CTRL (rw) register accessor: endpoint 14 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep14_r_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep14_r_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep14_r_ctrl`]
module"]
pub type UEP14_R_CTRL = crate::Reg<uep14_r_ctrl::UEP14_R_CTRL_SPEC>;
#[doc = "endpoint 14 send control"]
pub mod uep14_r_ctrl;
#[doc = "UEP15_T_LEN (rw) register accessor: endpoint 15 send the length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep15_t_len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep15_t_len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep15_t_len`]
module"]
pub type UEP15_T_LEN = crate::Reg<uep15_t_len::UEP15_T_LEN_SPEC>;
#[doc = "endpoint 15 send the length"]
pub mod uep15_t_len;
#[doc = "UEP15_T_CTRL (rw) register accessor: endpoint 15 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep15_t_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep15_t_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep15_t_ctrl`]
module"]
pub type UEP15_T_CTRL = crate::Reg<uep15_t_ctrl::UEP15_T_CTRL_SPEC>;
#[doc = "endpoint 15 send control"]
pub mod uep15_t_ctrl;
#[doc = "UEP15_R_CTRL (rw) register accessor: endpoint 15 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep15_r_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep15_r_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uep15_r_ctrl`]
module"]
pub type UEP15_R_CTRL = crate::Reg<uep15_r_ctrl::UEP15_R_CTRL_SPEC>;
#[doc = "endpoint 15 send control"]
pub mod uep15_r_ctrl;
