#[doc = "Register `IDATAR3` reader"]
pub type R = crate::R<IDATAR3_SPEC>;
#[doc = "Field `JDATA` reader - Injected data"]
pub type JDATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Injected data"]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "injected data register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idatar3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDATAR3_SPEC;
impl crate::RegisterSpec for IDATAR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idatar3::R`](R) reader structure"]
impl crate::Readable for IDATAR3_SPEC {}
#[doc = "`reset()` method sets IDATAR3 to value 0"]
impl crate::Resettable for IDATAR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
