#[doc = "Register `ISR3` reader"]
pub type R = crate::R<ISR3_SPEC>;
#[doc = "Field `INTENSTA` reader - Interrupt ID Status"]
pub type INTENSTA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt ID Status"]
    #[inline(always)]
    pub fn intensta(&self) -> INTENSTA_R {
        INTENSTA_R::new(self.bits)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR3_SPEC;
impl crate::RegisterSpec for ISR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr3::R`](R) reader structure"]
impl crate::Readable for ISR3_SPEC {}
#[doc = "`reset()` method sets ISR3 to value 0"]
impl crate::Resettable for ISR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
