#[doc = "Register `IRER4` writer"]
pub type W = crate::W<IRER4_SPEC>;
#[doc = "Field `INTRSET` writer - INTRSET"]
pub type INTRSET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl W {
    #[doc = "Bits 0:7 - INTRSET"]
    #[inline(always)]
    #[must_use]
    pub fn intrset(&mut self) -> INTRSET_W<IRER4_SPEC, 0> {
        INTRSET_W::new(self)
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
#[doc = "Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irer4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRER4_SPEC;
impl crate::RegisterSpec for IRER4_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`irer4::W`](W) writer structure"]
impl crate::Writable for IRER4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRER4 to value 0"]
impl crate::Resettable for IRER4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
