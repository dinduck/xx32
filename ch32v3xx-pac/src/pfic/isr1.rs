#[doc = "Register `ISR1` reader"]
pub type R = crate::R<ISR1_SPEC>;
#[doc = "Field `INTENSTA2_3` reader - Interrupt ID Status"]
pub type INTENSTA2_3_R = crate::FieldReader;
#[doc = "Field `INTENSTA12_31` reader - Interrupt ID Status"]
pub type INTENSTA12_31_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 2:3 - Interrupt ID Status"]
    #[inline(always)]
    pub fn intensta2_3(&self) -> INTENSTA2_3_R {
        INTENSTA2_3_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 12:31 - Interrupt ID Status"]
    #[inline(always)]
    pub fn intensta12_31(&self) -> INTENSTA12_31_R {
        INTENSTA12_31_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR1_SPEC;
impl crate::RegisterSpec for ISR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr1::R`](R) reader structure"]
impl crate::Readable for ISR1_SPEC {}
#[doc = "`reset()` method sets ISR1 to value 0x0c"]
impl crate::Resettable for ISR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c;
}
