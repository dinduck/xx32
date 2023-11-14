#[doc = "Register `STATR` reader"]
pub type R = crate::R<STATR_SPEC>;
#[doc = "Field `PVU` reader - Watchdog prescaler value update"]
pub type PVU_R = crate::BitReader;
#[doc = "Field `RVU` reader - Watchdog counter reload value update"]
pub type RVU_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Watchdog prescaler value update"]
    #[inline(always)]
    pub fn pvu(&self) -> PVU_R {
        PVU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog counter reload value update"]
    #[inline(always)]
    pub fn rvu(&self) -> RVU_R {
        RVU_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Status register (IWDG_SR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATR_SPEC;
impl crate::RegisterSpec for STATR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statr::R`](R) reader structure"]
impl crate::Readable for STATR_SPEC {}
#[doc = "`reset()` method sets STATR to value 0"]
impl crate::Resettable for STATR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
