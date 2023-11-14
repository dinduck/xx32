#[doc = "Register `IDATAR2` reader"]
pub type R = crate::R<IDATAR2_SPEC>;
#[doc = "Field `JDATA` reader - Injected data"]
pub type JDATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Injected data"]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "injected data register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idatar2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDATAR2_SPEC;
impl crate::RegisterSpec for IDATAR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idatar2::R`](R) reader structure"]
impl crate::Readable for IDATAR2_SPEC {}
#[doc = "`reset()` method sets IDATAR2 to value 0"]
impl crate::Resettable for IDATAR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
