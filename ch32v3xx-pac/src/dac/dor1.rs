#[doc = "Register `DOR1` reader"]
pub type R = crate::R<DOR1_SPEC>;
#[doc = "Field `DACC1DOR` reader - DAC channel1 data output"]
pub type DACC1DOR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - DAC channel1 data output"]
    #[inline(always)]
    pub fn dacc1dor(&self) -> DACC1DOR_R {
        DACC1DOR_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DAC channel1 data output register (DAC_DOR1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dor1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOR1_SPEC;
impl crate::RegisterSpec for DOR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dor1::R`](R) reader structure"]
impl crate::Readable for DOR1_SPEC {}
#[doc = "`reset()` method sets DOR1 to value 0"]
impl crate::Resettable for DOR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
