#[doc = "Register `ECCR2` reader"]
pub type R = crate::R<ECCR2_SPEC>;
#[doc = "Field `ECCx` reader - ECC result"]
pub type ECCX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC result"]
    #[inline(always)]
    pub fn eccx(&self) -> ECCX_R {
        ECCX_R::new(self.bits)
    }
}
#[doc = "ECC result register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECCR2_SPEC;
impl crate::RegisterSpec for ECCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccr2::R`](R) reader structure"]
impl crate::Readable for ECCR2_SPEC {}
#[doc = "`reset()` method sets ECCR2 to value 0"]
impl crate::Resettable for ECCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
