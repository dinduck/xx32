#[doc = "Register `MMCRFCECR` reader"]
pub type R = crate::R<MMCRFCECR_SPEC>;
#[doc = "Field `RFCFC` reader - Received frames with CRC error counter"]
pub type RFCFC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Received frames with CRC error counter"]
    #[inline(always)]
    pub fn rfcfc(&self) -> RFCFC_R {
        RFCFC_R::new(self.bits)
    }
}
#[doc = "Ethernet MMC received frames with CRC error counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrfcecr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCRFCECR_SPEC;
impl crate::RegisterSpec for MMCRFCECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcrfcecr::R`](R) reader structure"]
impl crate::Readable for MMCRFCECR_SPEC {}
#[doc = "`reset()` method sets MMCRFCECR to value 0"]
impl crate::Resettable for MMCRFCECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
