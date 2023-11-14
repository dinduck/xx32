#[doc = "Register `ROW_CNT` reader"]
pub type R = crate::R<ROW_CNT_SPEC>;
#[doc = "Field `RB_DVP_ROW_CNT` reader - The number of rows of frame image data"]
pub type RB_DVP_ROW_CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The number of rows of frame image data"]
    #[inline(always)]
    pub fn rb_dvp_row_cnt(&self) -> RB_DVP_ROW_CNT_R {
        RB_DVP_ROW_CNT_R::new(self.bits)
    }
}
#[doc = "Digital Video line counter register (DVP_ROW_CNT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`row_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROW_CNT_SPEC;
impl crate::RegisterSpec for ROW_CNT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`row_cnt::R`](R) reader structure"]
impl crate::Readable for ROW_CNT_SPEC {}
#[doc = "`reset()` method sets ROW_CNT to value 0"]
impl crate::Resettable for ROW_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
