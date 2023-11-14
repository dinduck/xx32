#[doc = "Register `MODEKEYR` writer"]
pub type W = crate::W<MODEKEYR_SPEC>;
#[doc = "Field `MODEKEYR` writer - Mode select"]
pub type MODEKEYR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Mode select"]
    #[inline(always)]
    #[must_use]
    pub fn modekeyr(&mut self) -> MODEKEYR_W<MODEKEYR_SPEC, 0> {
        MODEKEYR_W::new(self)
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
#[doc = "Mode select register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modekeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODEKEYR_SPEC;
impl crate::RegisterSpec for MODEKEYR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`modekeyr::W`](W) writer structure"]
impl crate::Writable for MODEKEYR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODEKEYR to value 0"]
impl crate::Resettable for MODEKEYR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
