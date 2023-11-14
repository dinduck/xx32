#[doc = "Register `PADDR8` reader"]
pub type R = crate::R<PADDR8_SPEC>;
#[doc = "Register `PADDR8` writer"]
pub type W = crate::W<PADDR8_SPEC>;
#[doc = "Field `PA` reader - Peripheral address"]
pub type PA_R = crate::FieldReader<u32>;
#[doc = "Field `PA` writer - Peripheral address"]
pub type PA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<PADDR8_SPEC, 0> {
        PA_W::new(self)
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
#[doc = "DMA channel 8 peripheral address register used in ch32v30x_D8/D8C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`paddr8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`paddr8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PADDR8_SPEC;
impl crate::RegisterSpec for PADDR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`paddr8::R`](R) reader structure"]
impl crate::Readable for PADDR8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`paddr8::W`](W) writer structure"]
impl crate::Writable for PADDR8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PADDR8 to value 0"]
impl crate::Resettable for PADDR8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
