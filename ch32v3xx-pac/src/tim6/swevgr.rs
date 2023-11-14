#[doc = "Register `SWEVGR` writer"]
pub type W = crate::W<SWEVGR_SPEC>;
#[doc = "Field `UG` writer - Update generation"]
pub type UG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<SWEVGR_SPEC, 0> {
        UG_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swevgr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWEVGR_SPEC;
impl crate::RegisterSpec for SWEVGR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swevgr::W`](W) writer structure"]
impl crate::Writable for SWEVGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWEVGR to value 0"]
impl crate::Resettable for SWEVGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
