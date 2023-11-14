#[doc = "Register `UEP12_TX_DMA____UH_SPLIT_DATA` reader"]
pub type R = crate::R<UEP12_TX_DMA____UH_SPLIT_DATA_SPEC>;
#[doc = "Register `UEP12_TX_DMA____UH_SPLIT_DATA` writer"]
pub type W = crate::W<UEP12_TX_DMA____UH_SPLIT_DATA_SPEC>;
#[doc = "Field `UEP12_TX_DMA___UH_SPLIT_DATA` reader - endpoint 12 DMA buffer address"]
pub type UEP12_TX_DMA___UH_SPLIT_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `UEP12_TX_DMA___UH_SPLIT_DATA` writer - endpoint 12 DMA buffer address"]
pub type UEP12_TX_DMA___UH_SPLIT_DATA_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - endpoint 12 DMA buffer address"]
    #[inline(always)]
    pub fn uep12_tx_dma___uh_split_data(&self) -> UEP12_TX_DMA___UH_SPLIT_DATA_R {
        UEP12_TX_DMA___UH_SPLIT_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - endpoint 12 DMA buffer address"]
    #[inline(always)]
    #[must_use]
    pub fn uep12_tx_dma___uh_split_data(
        &mut self,
    ) -> UEP12_TX_DMA___UH_SPLIT_DATA_W<UEP12_TX_DMA____UH_SPLIT_DATA_SPEC, 0> {
        UEP12_TX_DMA___UH_SPLIT_DATA_W::new(self)
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
#[doc = "endpoint 12 DMA TX buffer address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep12_tx_dma____uh_split_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep12_tx_dma____uh_split_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UEP12_TX_DMA____UH_SPLIT_DATA_SPEC;
impl crate::RegisterSpec for UEP12_TX_DMA____UH_SPLIT_DATA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uep12_tx_dma____uh_split_data::R`](R) reader structure"]
impl crate::Readable for UEP12_TX_DMA____UH_SPLIT_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uep12_tx_dma____uh_split_data::W`](W) writer structure"]
impl crate::Writable for UEP12_TX_DMA____UH_SPLIT_DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEP12_TX_DMA____UH_SPLIT_DATA to value 0"]
impl crate::Resettable for UEP12_TX_DMA____UH_SPLIT_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
