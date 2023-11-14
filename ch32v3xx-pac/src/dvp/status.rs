#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `RB_DVP_FIFO_RDY` reader - DVP frame start interrupt enable"]
pub type RB_DVP_FIFO_RDY_R = crate::BitReader;
#[doc = "Field `RB_DVP_FIFO_FULL` reader - DVP row received done interrupt enable"]
pub type RB_DVP_FIFO_FULL_R = crate::BitReader;
#[doc = "Field `RB_DVP_FIFO_OV` reader - DVP frame received done interrupt enable"]
pub type RB_DVP_FIFO_OV_R = crate::BitReader;
#[doc = "Field `RB_DVP_MSK_FIFO_CNT` reader - DVP receive fifo overflow interrupt enable"]
pub type RB_DVP_MSK_FIFO_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - DVP frame start interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_fifo_rdy(&self) -> RB_DVP_FIFO_RDY_R {
        RB_DVP_FIFO_RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DVP row received done interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_fifo_full(&self) -> RB_DVP_FIFO_FULL_R {
        RB_DVP_FIFO_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DVP frame received done interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_fifo_ov(&self) -> RB_DVP_FIFO_OV_R {
        RB_DVP_FIFO_OV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - DVP receive fifo overflow interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_msk_fifo_cnt(&self) -> RB_DVP_MSK_FIFO_CNT_R {
        RB_DVP_MSK_FIFO_CNT_R::new((self.bits >> 4) & 7)
    }
}
#[doc = "Digital Video STATUS register (DVP_STATUS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
