#[doc = "Register `RCRCR` reader"]
pub type R = crate::R<RCRCR_SPEC>;
#[doc = "Field `RXCRC` reader - Rx CRC register"]
pub type RXCRC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Rx CRC register"]
    #[inline(always)]
    pub fn rxcrc(&self) -> RXCRC_R {
        RXCRC_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "RX CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcrcr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCRCR_SPEC;
impl crate::RegisterSpec for RCRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcrcr::R`](R) reader structure"]
impl crate::Readable for RCRCR_SPEC {}
#[doc = "`reset()` method sets RCRCR to value 0"]
impl crate::Resettable for RCRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
