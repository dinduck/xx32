#[doc = "Register `PTPTSLR` reader"]
pub type R = crate::R<PTPTSLR_SPEC>;
#[doc = "Field `STSS` reader - System time subseconds"]
pub type STSS_R = crate::FieldReader<u32>;
#[doc = "Field `STPNS` reader - System time positive or negative sign"]
pub type STPNS_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:30 - System time subseconds"]
    #[inline(always)]
    pub fn stss(&self) -> STSS_R {
        STSS_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - System time positive or negative sign"]
    #[inline(always)]
    pub fn stpns(&self) -> STPNS_R {
        STPNS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Ethernet PTP time stamp low register (ETH_PTPTSLR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptslr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTSLR_SPEC;
impl crate::RegisterSpec for PTPTSLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptslr::R`](R) reader structure"]
impl crate::Readable for PTPTSLR_SPEC {}
#[doc = "`reset()` method sets PTPTSLR to value 0"]
impl crate::Resettable for PTPTSLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
