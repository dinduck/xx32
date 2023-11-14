#[doc = "Register `MMCTGFSCCR` reader"]
pub type R = crate::R<MMCTGFSCCR_SPEC>;
#[doc = "Field `TGFSCC` reader - Transmitted good frames after a single collision counter"]
pub type TGFSCC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted good frames after a single collision counter"]
    #[inline(always)]
    pub fn tgfscc(&self) -> TGFSCC_R {
        TGFSCC_R::new(self.bits)
    }
}
#[doc = "Ethernet MMC transmitted good frames after a single collision counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctgfsccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCTGFSCCR_SPEC;
impl crate::RegisterSpec for MMCTGFSCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctgfsccr::R`](R) reader structure"]
impl crate::Readable for MMCTGFSCCR_SPEC {}
#[doc = "`reset()` method sets MMCTGFSCCR to value 0"]
impl crate::Resettable for MMCTGFSCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
