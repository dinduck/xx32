#[doc = "Register `ISR4` reader"]
pub type R = crate::R<ISR4_SPEC>;
#[doc = "Field `INTENSTA` reader - Interrupt ID Status"]
pub type INTENSTA_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Interrupt ID Status"]
    #[inline(always)]
    pub fn intensta(&self) -> INTENSTA_R {
        INTENSTA_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR4_SPEC;
impl crate::RegisterSpec for ISR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr4::R`](R) reader structure"]
impl crate::Readable for ISR4_SPEC {}
#[doc = "`reset()` method sets ISR4 to value 0"]
impl crate::Resettable for ISR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
