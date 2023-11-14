#[doc = "Register `DR` reader"]
pub type R = crate::R<DR_SPEC>;
#[doc = "Field `RB_DVP_DR` reader - Prevent DMA overflow"]
pub type RB_DVP_DR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Prevent DMA overflow"]
    #[inline(always)]
    pub fn rb_dvp_dr(&self) -> RB_DVP_DR_R {
        RB_DVP_DR_R::new(self.bits)
    }
}
#[doc = "Digital Video Data register (DVP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DR_SPEC {}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
