#[doc = "Register `R8BDHR2` reader"]
pub type R = crate::R<R8BDHR2_SPEC>;
#[doc = "Register `R8BDHR2` writer"]
pub type W = crate::W<R8BDHR2_SPEC>;
#[doc = "Field `DACC2DHR` reader - DAC channel2 8-bit right-aligned data"]
pub type DACC2DHR_R = crate::FieldReader;
#[doc = "Field `DACC2DHR` writer - DAC channel2 8-bit right-aligned data"]
pub type DACC2DHR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DAC channel2 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC channel2 8-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W<R8BDHR2_SPEC, 0> {
        DACC2DHR_W::new(self)
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
#[doc = "DAC channel2 8-bit right-aligned data holding register (DAC_DHR8R2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8bdhr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8bdhr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8BDHR2_SPEC;
impl crate::RegisterSpec for R8BDHR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r8bdhr2::R`](R) reader structure"]
impl crate::Readable for R8BDHR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`r8bdhr2::W`](W) writer structure"]
impl crate::Writable for R8BDHR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets R8BDHR2 to value 0"]
impl crate::Resettable for R8BDHR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
