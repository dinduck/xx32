#[doc = "Register `MMCRGUFCR` reader"]
pub type R = crate::R<MMCRGUFCR_SPEC>;
#[doc = "Field `RGUFC` reader - Received good unicast frames counter"]
pub type RGUFC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Received good unicast frames counter"]
    #[inline(always)]
    pub fn rgufc(&self) -> RGUFC_R {
        RGUFC_R::new(self.bits)
    }
}
#[doc = "MMC received good unicast frames counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrgufcr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCRGUFCR_SPEC;
impl crate::RegisterSpec for MMCRGUFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcrgufcr::R`](R) reader structure"]
impl crate::Readable for MMCRGUFCR_SPEC {}
#[doc = "`reset()` method sets MMCRGUFCR to value 0"]
impl crate::Resettable for MMCRGUFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
