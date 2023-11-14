#[doc = "Register `UEP0_DMA` reader"]
pub type R = crate::R<UEP0_DMA_SPEC>;
#[doc = "Register `UEP0_DMA` writer"]
pub type W = crate::W<UEP0_DMA_SPEC>;
#[doc = "Field `UEP0_DMA` reader - endpoint 0 DMA buffer address"]
pub type UEP0_DMA_R = crate::FieldReader<u16>;
#[doc = "Field `UEP0_DMA` writer - endpoint 0 DMA buffer address"]
pub type UEP0_DMA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - endpoint 0 DMA buffer address"]
    #[inline(always)]
    pub fn uep0_dma(&self) -> UEP0_DMA_R {
        UEP0_DMA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - endpoint 0 DMA buffer address"]
    #[inline(always)]
    #[must_use]
    pub fn uep0_dma(&mut self) -> UEP0_DMA_W<UEP0_DMA_SPEC, 0> {
        UEP0_DMA_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "B endpoint 0 DMA buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep0_dma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep0_dma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UEP0_DMA_SPEC;
impl crate::RegisterSpec for UEP0_DMA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uep0_dma::R`](R) reader structure"]
impl crate::Readable for UEP0_DMA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uep0_dma::W`](W) writer structure"]
impl crate::Writable for UEP0_DMA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEP0_DMA to value 0"]
impl crate::Resettable for UEP0_DMA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
