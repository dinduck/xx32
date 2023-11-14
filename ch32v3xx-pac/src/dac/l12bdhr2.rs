#[doc = "Register `L12BDHR2` reader"]
pub type R = crate::R<L12BDHR2_SPEC>;
#[doc = "Register `L12BDHR2` writer"]
pub type W = crate::W<L12BDHR2_SPEC>;
#[doc = "Field `DACC2DHR` reader - DAC channel2 12-bit left-aligned data"]
pub type DACC2DHR_R = crate::FieldReader<u16>;
#[doc = "Field `DACC2DHR` writer - DAC channel2 12-bit left-aligned data"]
pub type DACC2DHR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 4:15 - DAC channel2 12-bit left-aligned data"]
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - DAC channel2 12-bit left-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W<L12BDHR2_SPEC, 4> {
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
#[doc = "DAC channel2 12-bit left aligned data holding register (DAC_DHR12L2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l12bdhr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l12bdhr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L12BDHR2_SPEC;
impl crate::RegisterSpec for L12BDHR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l12bdhr2::R`](R) reader structure"]
impl crate::Readable for L12BDHR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l12bdhr2::W`](W) writer structure"]
impl crate::Writable for L12BDHR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L12BDHR2 to value 0"]
impl crate::Resettable for L12BDHR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
