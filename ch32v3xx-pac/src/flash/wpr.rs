#[doc = "Register `WPR` reader"]
pub type R = crate::R<WPR_SPEC>;
#[doc = "Field `WRP` reader - Write protect"]
pub type WRP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Write protect"]
    #[inline(always)]
    pub fn wrp(&self) -> WRP_R {
        WRP_R::new(self.bits)
    }
}
#[doc = "Write protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPR_SPEC;
impl crate::RegisterSpec for WPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpr::R`](R) reader structure"]
impl crate::Readable for WPR_SPEC {}
#[doc = "`reset()` method sets WPR to value 0xffff_ffff"]
impl crate::Resettable for WPR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
