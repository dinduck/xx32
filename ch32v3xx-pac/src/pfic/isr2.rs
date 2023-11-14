#[doc = "Register `ISR2` reader"]
pub type R = crate::R<ISR2_SPEC>;
#[doc = "Field `INTENSTA` reader - Interrupt ID Status"]
pub type INTENSTA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt ID Status"]
    #[inline(always)]
    pub fn intensta(&self) -> INTENSTA_R {
        INTENSTA_R::new(self.bits)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR2_SPEC;
impl crate::RegisterSpec for ISR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr2::R`](R) reader structure"]
impl crate::Readable for ISR2_SPEC {}
#[doc = "`reset()` method sets ISR2 to value 0"]
impl crate::Resettable for ISR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
