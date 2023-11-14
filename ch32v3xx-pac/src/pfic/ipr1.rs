#[doc = "Register `IPR1` reader"]
pub type R = crate::R<IPR1_SPEC>;
#[doc = "Field `PENDSTA2_3` reader - PENDSTA"]
pub type PENDSTA2_3_R = crate::FieldReader;
#[doc = "Field `PENDSTA12_31` reader - PENDSTA"]
pub type PENDSTA12_31_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 2:3 - PENDSTA"]
    #[inline(always)]
    pub fn pendsta2_3(&self) -> PENDSTA2_3_R {
        PENDSTA2_3_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 12:31 - PENDSTA"]
    #[inline(always)]
    pub fn pendsta12_31(&self) -> PENDSTA12_31_R {
        PENDSTA12_31_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
#[doc = "Interrupt Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPR1_SPEC;
impl crate::RegisterSpec for IPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipr1::R`](R) reader structure"]
impl crate::Readable for IPR1_SPEC {}
#[doc = "`reset()` method sets IPR1 to value 0"]
impl crate::Resettable for IPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
