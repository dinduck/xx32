#[doc = "Register `R32_UEP7_DMA` reader"]
pub type R = crate::R<R32_UEP7_DMA_SPEC>;
#[doc = "Register `R32_UEP7_DMA` writer"]
pub type W = crate::W<R32_UEP7_DMA_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<R32_UEP7_DMA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
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
#[doc = "endpoint 7 DMA buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r32_uep7_dma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r32_uep7_dma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32_UEP7_DMA_SPEC;
impl crate::RegisterSpec for R32_UEP7_DMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_uep7_dma::R`](R) reader structure"]
impl crate::Readable for R32_UEP7_DMA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`r32_uep7_dma::W`](W) writer structure"]
impl crate::Writable for R32_UEP7_DMA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets R32_UEP7_DMA to value 0"]
impl crate::Resettable for R32_UEP7_DMA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
