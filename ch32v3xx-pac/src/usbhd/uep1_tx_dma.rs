#[doc = "Register `UEP1_TX_DMA` reader"]
pub type R = crate::R<UEP1_TX_DMA_SPEC>;
#[doc = "Register `UEP1_TX_DMA` writer"]
pub type W = crate::W<UEP1_TX_DMA_SPEC>;
#[doc = "Field `UEP1_TX_DMA` reader - endpoint 1 DMA buffer address"]
pub type UEP1_TX_DMA_R = crate::FieldReader<u16>;
#[doc = "Field `UEP1_TX_DMA` writer - endpoint 1 DMA buffer address"]
pub type UEP1_TX_DMA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - endpoint 1 DMA buffer address"]
    #[inline(always)]
    pub fn uep1_tx_dma(&self) -> UEP1_TX_DMA_R {
        UEP1_TX_DMA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - endpoint 1 DMA buffer address"]
    #[inline(always)]
    #[must_use]
    pub fn uep1_tx_dma(&mut self) -> UEP1_TX_DMA_W<UEP1_TX_DMA_SPEC, 0> {
        UEP1_TX_DMA_W::new(self)
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
#[doc = "endpoint 1 DMA TX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep1_tx_dma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep1_tx_dma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UEP1_TX_DMA_SPEC;
impl crate::RegisterSpec for UEP1_TX_DMA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uep1_tx_dma::R`](R) reader structure"]
impl crate::Readable for UEP1_TX_DMA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uep1_tx_dma::W`](W) writer structure"]
impl crate::Writable for UEP1_TX_DMA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEP1_TX_DMA to value 0"]
impl crate::Resettable for UEP1_TX_DMA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
