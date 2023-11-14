#[doc = "Register `GISR` reader"]
pub type R = crate::R<GISR_SPEC>;
#[doc = "Field `NESTSTA` reader - NESTSTA"]
pub type NESTSTA_R = crate::FieldReader;
#[doc = "Field `GACTSTA` reader - GACTSTA"]
pub type GACTSTA_R = crate::BitReader;
#[doc = "Field `GPENDSTA` reader - GPENDSTA"]
pub type GPENDSTA_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - NESTSTA"]
    #[inline(always)]
    pub fn neststa(&self) -> NESTSTA_R {
        NESTSTA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - GACTSTA"]
    #[inline(always)]
    pub fn gactsta(&self) -> GACTSTA_R {
        GACTSTA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPENDSTA"]
    #[inline(always)]
    pub fn gpendsta(&self) -> GPENDSTA_R {
        GPENDSTA_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Interrupt Global Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gisr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GISR_SPEC;
impl crate::RegisterSpec for GISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gisr::R`](R) reader structure"]
impl crate::Readable for GISR_SPEC {}
#[doc = "`reset()` method sets GISR to value 0"]
impl crate::Resettable for GISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
