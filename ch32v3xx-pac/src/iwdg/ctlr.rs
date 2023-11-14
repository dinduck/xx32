#[doc = "Register `CTLR` writer"]
pub type W = crate::W<CTLR_SPEC>;
#[doc = "Field `KEY` writer - Key value"]
pub type KEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl W {
    #[doc = "Bits 0:15 - Key value"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<CTLR_SPEC, 0> {
        KEY_W::new(self)
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
#[doc = "Key register (IWDG_CTLR)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTLR_SPEC;
impl crate::RegisterSpec for CTLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ctlr::W`](W) writer structure"]
impl crate::Writable for CTLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTLR to value 0"]
impl crate::Resettable for CTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
