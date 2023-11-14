#[doc = "Register `IPR3` reader"]
pub type R = crate::R<IPR3_SPEC>;
#[doc = "Field `PENDSTA` reader - PENDSTA"]
pub type PENDSTA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PENDSTA"]
    #[inline(always)]
    pub fn pendsta(&self) -> PENDSTA_R {
        PENDSTA_R::new(self.bits)
    }
}
#[doc = "Interrupt Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPR3_SPEC;
impl crate::RegisterSpec for IPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipr3::R`](R) reader structure"]
impl crate::Readable for IPR3_SPEC {}
#[doc = "`reset()` method sets IPR3 to value 0"]
impl crate::Resettable for IPR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
