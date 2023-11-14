#[doc = "Register `IDATAR1_CHGOFFSET` reader"]
pub type R = crate::R<IDATAR1_CHGOFFSET_SPEC>;
#[doc = "Field `IDATA0_7_TKCGOFFSET` reader - Injected data_Touch key charge data offset for injected channel x"]
pub type IDATA0_7_TKCGOFFSET_R = crate::FieldReader;
#[doc = "Field `IDATA8_15` reader - Injected data"]
pub type IDATA8_15_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Injected data_Touch key charge data offset for injected channel x"]
    #[inline(always)]
    pub fn idata0_7_tkcgoffset(&self) -> IDATA0_7_TKCGOFFSET_R {
        IDATA0_7_TKCGOFFSET_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Injected data"]
    #[inline(always)]
    pub fn idata8_15(&self) -> IDATA8_15_R {
        IDATA8_15_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "injected data register x_Charge data offset for injected channel x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idatar1_chgoffset::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDATAR1_CHGOFFSET_SPEC;
impl crate::RegisterSpec for IDATAR1_CHGOFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idatar1_chgoffset::R`](R) reader structure"]
impl crate::Readable for IDATAR1_CHGOFFSET_SPEC {}
#[doc = "`reset()` method sets IDATAR1_CHGOFFSET to value 0"]
impl crate::Resettable for IDATAR1_CHGOFFSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
