#[doc = "Register `UEP2_RX_DMA__UH_RX_DMA` reader"]
pub type R = crate::R<UEP2_RX_DMA__UH_RX_DMA_SPEC>;
#[doc = "Register `UEP2_RX_DMA__UH_RX_DMA` writer"]
pub type W = crate::W<UEP2_RX_DMA__UH_RX_DMA_SPEC>;
#[doc = "Field `UEP2_RX_DMA__UH_RX_DMA` reader - endpoint 2 DMA buffer address"]
pub type UEP2_RX_DMA__UH_RX_DMA_R = crate::FieldReader<u16>;
#[doc = "Field `UEP2_RX_DMA__UH_RX_DMA` writer - endpoint 2 DMA buffer address"]
pub type UEP2_RX_DMA__UH_RX_DMA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - endpoint 2 DMA buffer address"]
    #[inline(always)]
    pub fn uep2_rx_dma__uh_rx_dma(&self) -> UEP2_RX_DMA__UH_RX_DMA_R {
        UEP2_RX_DMA__UH_RX_DMA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - endpoint 2 DMA buffer address"]
    #[inline(always)]
    #[must_use]
    pub fn uep2_rx_dma__uh_rx_dma(
        &mut self,
    ) -> UEP2_RX_DMA__UH_RX_DMA_W<UEP2_RX_DMA__UH_RX_DMA_SPEC, 0> {
        UEP2_RX_DMA__UH_RX_DMA_W::new(self)
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
#[doc = "endpoint 2 DMA RX buffer address/UH_RX_DMA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep2_rx_dma__uh_rx_dma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep2_rx_dma__uh_rx_dma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UEP2_RX_DMA__UH_RX_DMA_SPEC;
impl crate::RegisterSpec for UEP2_RX_DMA__UH_RX_DMA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uep2_rx_dma__uh_rx_dma::R`](R) reader structure"]
impl crate::Readable for UEP2_RX_DMA__UH_RX_DMA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uep2_rx_dma__uh_rx_dma::W`](W) writer structure"]
impl crate::Writable for UEP2_RX_DMA__UH_RX_DMA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEP2_RX_DMA__UH_RX_DMA to value 0"]
impl crate::Resettable for UEP2_RX_DMA__UH_RX_DMA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
