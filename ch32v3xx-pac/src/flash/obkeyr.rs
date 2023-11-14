#[doc = "Register `OBKEYR` writer"]
pub type W = crate::W<OBKEYR_SPEC>;
#[doc = "Field `OPTKEY` writer - Option byte key"]
pub type OPTKEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Option byte key"]
    #[inline(always)]
    #[must_use]
    pub fn optkey(&mut self) -> OPTKEY_W<OBKEYR_SPEC, 0> {
        OPTKEY_W::new(self)
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
#[doc = "Flash option key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OBKEYR_SPEC;
impl crate::RegisterSpec for OBKEYR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`obkeyr::W`](W) writer structure"]
impl crate::Writable for OBKEYR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OBKEYR to value 0"]
impl crate::Resettable for OBKEYR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
