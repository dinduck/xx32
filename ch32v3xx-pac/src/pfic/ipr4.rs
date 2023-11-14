#[doc = "Register `IPR4` reader"]
pub type R = crate::R<IPR4_SPEC>;
#[doc = "Field `PENDSTA` reader - PENDSTA"]
pub type PENDSTA_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - PENDSTA"]
    #[inline(always)]
    pub fn pendsta(&self) -> PENDSTA_R {
        PENDSTA_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipr4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPR4_SPEC;
impl crate::RegisterSpec for IPR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipr4::R`](R) reader structure"]
impl crate::Readable for IPR4_SPEC {}
#[doc = "`reset()` method sets IPR4 to value 0"]
impl crate::Resettable for IPR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
