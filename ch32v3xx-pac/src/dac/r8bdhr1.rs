#[doc = "Register `R8BDHR1` reader"]
pub type R = crate::R<R8BDHR1_SPEC>;
#[doc = "Register `R8BDHR1` writer"]
pub type W = crate::W<R8BDHR1_SPEC>;
#[doc = "Field `DACC1DHR` reader - DAC channel1 8-bit right-aligned data"]
pub type DACC1DHR_R = crate::FieldReader;
#[doc = "Field `DACC1DHR` writer - DAC channel1 8-bit right-aligned data"]
pub type DACC1DHR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DAC channel1 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC channel1 8-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W<R8BDHR1_SPEC, 0> {
        DACC1DHR_W::new(self)
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
#[doc = "DAC channel1 8-bit right aligned data holding register (DAC_DHR8R1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8bdhr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8bdhr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8BDHR1_SPEC;
impl crate::RegisterSpec for R8BDHR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r8bdhr1::R`](R) reader structure"]
impl crate::Readable for R8BDHR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`r8bdhr1::W`](W) writer structure"]
impl crate::Writable for R8BDHR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets R8BDHR1 to value 0"]
impl crate::Resettable for R8BDHR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
