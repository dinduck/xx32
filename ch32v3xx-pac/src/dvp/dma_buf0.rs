#[doc = "Register `DMA_BUF0` reader"]
pub type R = crate::R<DMA_BUF0_SPEC>;
#[doc = "Register `DMA_BUF0` writer"]
pub type W = crate::W<DMA_BUF0_SPEC>;
#[doc = "Field `RB_DVP_DMA_BUF0` reader - DMA receive address 0"]
pub type RB_DVP_DMA_BUF0_R = crate::FieldReader<u32>;
#[doc = "Field `RB_DVP_DMA_BUF0` writer - DMA receive address 0"]
pub type RB_DVP_DMA_BUF0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 17, O, u32>;
impl R {
    #[doc = "Bits 0:16 - DMA receive address 0"]
    #[inline(always)]
    pub fn rb_dvp_dma_buf0(&self) -> RB_DVP_DMA_BUF0_R {
        RB_DVP_DMA_BUF0_R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - DMA receive address 0"]
    #[inline(always)]
    #[must_use]
    pub fn rb_dvp_dma_buf0(&mut self) -> RB_DVP_DMA_BUF0_W<DMA_BUF0_SPEC, 0> {
        RB_DVP_DMA_BUF0_W::new(self)
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
#[doc = "Digital Video DMA address register (DVP_DMA_BUF0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_buf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_buf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_BUF0_SPEC;
impl crate::RegisterSpec for DMA_BUF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_buf0::R`](R) reader structure"]
impl crate::Readable for DMA_BUF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_buf0::W`](W) writer structure"]
impl crate::Writable for DMA_BUF0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_BUF0 to value 0"]
impl crate::Resettable for DMA_BUF0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
