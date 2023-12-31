#[doc = "Register `IENR1` writer"]
pub type W = crate::W<IENR1_SPEC>;
#[doc = "Field `INTEN` writer - INTEN"]
pub type INTEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 20, O, u32>;
impl W {
    #[doc = "Bits 12:31 - INTEN"]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> INTEN_W<IENR1_SPEC, 12> {
        INTEN_W::new(self)
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
#[doc = "Interrupt Setting Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ienr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IENR1_SPEC;
impl crate::RegisterSpec for IENR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ienr1::W`](W) writer structure"]
impl crate::Writable for IENR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IENR1 to value 0"]
impl crate::Resettable for IENR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
