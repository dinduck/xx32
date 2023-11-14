#[doc = "Register `TCRCR` reader"]
pub type R = crate::R<TCRCR_SPEC>;
#[doc = "Field `TXCRC` reader - Tx CRC register"]
pub type TXCRC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Tx CRC register"]
    #[inline(always)]
    pub fn txcrc(&self) -> TXCRC_R {
        TXCRC_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "TX CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcrcr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCRCR_SPEC;
impl crate::RegisterSpec for TCRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcrcr::R`](R) reader structure"]
impl crate::Readable for TCRCR_SPEC {}
#[doc = "`reset()` method sets TCRCR to value 0"]
impl crate::Resettable for TCRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
