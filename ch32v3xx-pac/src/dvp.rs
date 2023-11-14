#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Digital Video control register (DVP_CR0)"]
    pub cr0: CR0,
    #[doc = "0x01 - Digital Video control register (DVP_CR1)"]
    pub cr1: CR1,
    #[doc = "0x02 - Digital Video Interrupt register (DVP_IER)"]
    pub ier: IER,
    _reserved3: [u8; 0x01],
    #[doc = "0x04 - Image line count configuration register (DVP_ROW_NUM)"]
    pub row_num: ROW_NUM,
    #[doc = "0x06 - Image column number configuration register (DVP_COL_NUM)"]
    pub col_num: COL_NUM,
    #[doc = "0x08 - Digital Video DMA address register (DVP_DMA_BUF0)"]
    pub dma_buf0: DMA_BUF0,
    #[doc = "0x0c - Digital Video DMA address register (DVP_DMA_BUF1)"]
    pub dma_buf1: DMA_BUF1,
    #[doc = "0x10 - Digital Video Flag register (DVP_IFR)"]
    pub ifr: IFR,
    #[doc = "0x11 - Digital Video STATUS register (DVP_STATUS)"]
    pub status: STATUS,
    _reserved9: [u8; 0x02],
    #[doc = "0x14 - Digital Video line counter register (DVP_ROW_CNT)"]
    pub row_cnt: ROW_CNT,
    _reserved10: [u8; 0x02],
    #[doc = "0x18 - Digital Video horizontal displacement register (DVP_HOFFCNT)"]
    pub hoffcnt: HOFFCNT,
    #[doc = "0x1a - Digital Video line number register (DVP_VST)"]
    pub vst: VST,
    #[doc = "0x1c - Digital Video Capture count register (DVP_CAPCNT)"]
    pub capcnt: CAPCNT,
    #[doc = "0x1e - Digital Video Vertical line count register (DVP_VLINE)"]
    pub vline: VLINE,
    #[doc = "0x20 - Digital Video Data register (DVP_DR)"]
    pub dr: DR,
}
#[doc = "CR0 (rw) register accessor: Digital Video control register (DVP_CR0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr0`]
module"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "Digital Video control register (DVP_CR0)"]
pub mod cr0;
#[doc = "CR1 (rw) register accessor: Digital Video control register (DVP_CR1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "Digital Video control register (DVP_CR1)"]
pub mod cr1;
#[doc = "IER (rw) register accessor: Digital Video Interrupt register (DVP_IER)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Digital Video Interrupt register (DVP_IER)"]
pub mod ier;
#[doc = "ROW_NUM (rw) register accessor: Image line count configuration register (DVP_ROW_NUM)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`row_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`row_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@row_num`]
module"]
pub type ROW_NUM = crate::Reg<row_num::ROW_NUM_SPEC>;
#[doc = "Image line count configuration register (DVP_ROW_NUM)"]
pub mod row_num;
#[doc = "COL_NUM (rw) register accessor: Image column number configuration register (DVP_COL_NUM)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`col_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`col_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@col_num`]
module"]
pub type COL_NUM = crate::Reg<col_num::COL_NUM_SPEC>;
#[doc = "Image column number configuration register (DVP_COL_NUM)"]
pub mod col_num;
#[doc = "DMA_BUF0 (rw) register accessor: Digital Video DMA address register (DVP_DMA_BUF0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_buf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_buf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_buf0`]
module"]
pub type DMA_BUF0 = crate::Reg<dma_buf0::DMA_BUF0_SPEC>;
#[doc = "Digital Video DMA address register (DVP_DMA_BUF0)"]
pub mod dma_buf0;
#[doc = "DMA_BUF1 (rw) register accessor: Digital Video DMA address register (DVP_DMA_BUF1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_buf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_buf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_buf1`]
module"]
pub type DMA_BUF1 = crate::Reg<dma_buf1::DMA_BUF1_SPEC>;
#[doc = "Digital Video DMA address register (DVP_DMA_BUF1)"]
pub mod dma_buf1;
#[doc = "IFR (rw) register accessor: Digital Video Flag register (DVP_IFR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifr`]
module"]
pub type IFR = crate::Reg<ifr::IFR_SPEC>;
#[doc = "Digital Video Flag register (DVP_IFR)"]
pub mod ifr;
#[doc = "STATUS (r) register accessor: Digital Video STATUS register (DVP_STATUS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Digital Video STATUS register (DVP_STATUS)"]
pub mod status;
#[doc = "ROW_CNT (r) register accessor: Digital Video line counter register (DVP_ROW_CNT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`row_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@row_cnt`]
module"]
pub type ROW_CNT = crate::Reg<row_cnt::ROW_CNT_SPEC>;
#[doc = "Digital Video line counter register (DVP_ROW_CNT)"]
pub mod row_cnt;
#[doc = "HOFFCNT (rw) register accessor: Digital Video horizontal displacement register (DVP_HOFFCNT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hoffcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hoffcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hoffcnt`]
module"]
pub type HOFFCNT = crate::Reg<hoffcnt::HOFFCNT_SPEC>;
#[doc = "Digital Video horizontal displacement register (DVP_HOFFCNT)"]
pub mod hoffcnt;
#[doc = "VST (rw) register accessor: Digital Video line number register (DVP_VST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vst`]
module"]
pub type VST = crate::Reg<vst::VST_SPEC>;
#[doc = "Digital Video line number register (DVP_VST)"]
pub mod vst;
#[doc = "CAPCNT (rw) register accessor: Digital Video Capture count register (DVP_CAPCNT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`capcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capcnt`]
module"]
pub type CAPCNT = crate::Reg<capcnt::CAPCNT_SPEC>;
#[doc = "Digital Video Capture count register (DVP_CAPCNT)"]
pub mod capcnt;
#[doc = "VLINE (rw) register accessor: Digital Video Vertical line count register (DVP_VLINE)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vline::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vline::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vline`]
module"]
pub type VLINE = crate::Reg<vline::VLINE_SPEC>;
#[doc = "Digital Video Vertical line count register (DVP_VLINE)"]
pub mod vline;
#[doc = "DR (r) register accessor: Digital Video Data register (DVP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Digital Video Data register (DVP_DR)"]
pub mod dr;
