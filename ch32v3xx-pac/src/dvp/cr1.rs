#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1_SPEC>;
#[doc = "Field `RB_DVP_DMA_EN` reader - DVP dma enable"]
pub type RB_DVP_DMA_EN_R = crate::BitReader;
#[doc = "Field `RB_DVP_DMA_EN` writer - DVP dma enable"]
pub type RB_DVP_DMA_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_DVP_ALL_CLR` reader - DVP all clear"]
pub type RB_DVP_ALL_CLR_R = crate::BitReader;
#[doc = "Field `RB_DVP_ALL_CLR` writer - DVP all clear"]
pub type RB_DVP_ALL_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_DVP_RCV_CLR` reader - DVP receive logic clear"]
pub type RB_DVP_RCV_CLR_R = crate::BitReader;
#[doc = "Field `RB_DVP_RCV_CLR` writer - DVP receive logic clear"]
pub type RB_DVP_RCV_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_DVP_BUF_TOG` reader - DVP bug toggle by software"]
pub type RB_DVP_BUF_TOG_R = crate::BitReader;
#[doc = "Field `RB_DVP_BUF_TOG` writer - DVP bug toggle by software"]
pub type RB_DVP_BUF_TOG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_DVP_CM` reader - DVP capture mode"]
pub type RB_DVP_CM_R = crate::BitReader;
#[doc = "Field `RB_DVP_CM` writer - DVP capture mode"]
pub type RB_DVP_CM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_DVP_CROP` reader - DVP Crop feature enable"]
pub type RB_DVP_CROP_R = crate::BitReader;
#[doc = "Field `RB_DVP_CROP` writer - DVP Crop feature enable"]
pub type RB_DVP_CROP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_DVP_FCRC` reader - DVP frame capture rate control"]
pub type RB_DVP_FCRC_R = crate::FieldReader;
#[doc = "Field `RB_DVP_FCRC` writer - DVP frame capture rate control"]
pub type RB_DVP_FCRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bit 0 - DVP dma enable"]
    #[inline(always)]
    pub fn rb_dvp_dma_en(&self) -> RB_DVP_DMA_EN_R {
        RB_DVP_DMA_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DVP all clear"]
    #[inline(always)]
    pub fn rb_dvp_all_clr(&self) -> RB_DVP_ALL_CLR_R {
        RB_DVP_ALL_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DVP receive logic clear"]
    #[inline(always)]
    pub fn rb_dvp_rcv_clr(&self) -> RB_DVP_RCV_CLR_R {
        RB_DVP_RCV_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DVP bug toggle by software"]
    #[inline(always)]
    pub fn rb_dvp_buf_tog(&self) -> RB_DVP_BUF_TOG_R {
        RB_DVP_BUF_TOG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DVP capture mode"]
    #[inline(always)]
    pub fn rb_dvp_cm(&self) -> RB_DVP_CM_R {
        RB_DVP_CM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DVP Crop feature enable"]
    #[inline(always)]
    pub fn rb_dvp_crop(&self) -> RB_DVP_CROP_R {
        RB_DVP_CROP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - DVP frame capture rate control"]
    #[inline(always)]
    pub fn rb_dvp_fcrc(&self) -> RB_DVP_FCRC_R {
        RB_DVP_FCRC_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - DVP dma enable"]
    #[inline(always)]
    #[must_use]
    pub fn rb_dvp_dma_en(&mut self) -> RB_DVP_DMA_EN_W<CR1_SPEC, 0> {
        RB_DVP_DMA_EN_W::new(self)
    }
    #[doc = "Bit 1 - DVP all clear"]
    #[inline(always)]
    #[must_use]
    pub fn rb_dvp_all_clr(&mut self) -> RB_DVP_ALL_CLR_W<CR1_SPEC, 1> {
        RB_DVP_ALL_CLR_W::new(self)
    }
    #[doc = "Bit 2 - DVP receive logic clear"]
    #[inline(always)]
    #[must_use]
    pub fn rb_dvp_rcv_clr(&mut self) -> RB_DVP_RCV_CLR_W<CR1_SPEC, 2> {
        RB_DVP_RCV_CLR_W::new(self)
    }
    #[doc = "Bit 3 - DVP bug toggle by software"]
    #[inline(always)]
    #[must_use]
    pub fn rb_dvp_buf_tog(&mut self) -> RB_DVP_BUF_TOG_W<CR1_SPEC, 3> {
        RB_DVP_BUF_TOG_W::new(self)
    }
    #[doc = "Bit 4 - DVP capture mode"]
    #[inline(always)]
    #[must_use]
    pub fn rb_dvp_cm(&mut self) -> RB_DVP_CM_W<CR1_SPEC, 4> {
        RB_DVP_CM_W::new(self)
    }
    #[doc = "Bit 5 - DVP Crop feature enable"]
    #[inline(always)]
    #[must_use]
    pub fn rb_dvp_crop(&mut self) -> RB_DVP_CROP_W<CR1_SPEC, 5> {
        RB_DVP_CROP_W::new(self)
    }
    #[doc = "Bits 6:7 - DVP frame capture rate control"]
    #[inline(always)]
    #[must_use]
    pub fn rb_dvp_fcrc(&mut self) -> RB_DVP_FCRC_W<CR1_SPEC, 6> {
        RB_DVP_FCRC_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Digital Video control register (DVP_CR1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0x06"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x06;
}
