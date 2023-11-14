#[doc = "Register `USBHD_BASE_CTRL` reader"]
pub type R = crate::R<USBHD_BASE_CTRL_SPEC>;
#[doc = "Register `USBHD_BASE_CTRL` writer"]
pub type W = crate::W<USBHD_BASE_CTRL_SPEC>;
#[doc = "Field `USBHD_UC_DMA_EN` reader - DMA enable and DMA interrupt enable for USB"]
pub type USBHD_UC_DMA_EN_R = crate::BitReader;
#[doc = "Field `USBHD_UC_DMA_EN` writer - DMA enable and DMA interrupt enable for USB"]
pub type USBHD_UC_DMA_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBHD_UC_CLR_ALL` reader - force clear FIFO and count of USB"]
pub type USBHD_UC_CLR_ALL_R = crate::BitReader;
#[doc = "Field `USBHD_UC_CLR_ALL` writer - force clear FIFO and count of USB"]
pub type USBHD_UC_CLR_ALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBHD_UC_RESET_SIE` reader - force reset USB SIE, need software clear"]
pub type USBHD_UC_RESET_SIE_R = crate::BitReader;
#[doc = "Field `USBHD_UC_RESET_SIE` writer - force reset USB SIE, need software clear"]
pub type USBHD_UC_RESET_SIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBHD_UC_INT_BUSY` reader - enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid"]
pub type USBHD_UC_INT_BUSY_R = crate::BitReader;
#[doc = "Field `USBHD_UC_INT_BUSY` writer - enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid"]
pub type USBHD_UC_INT_BUSY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBHD_UC_SYS_CTRL_MASK` reader - USB device enable and internal pullup resistance enable"]
pub type USBHD_UC_SYS_CTRL_MASK_R = crate::FieldReader;
#[doc = "Field `USBHD_UC_SYS_CTRL_MASK` writer - USB device enable and internal pullup resistance enable"]
pub type USBHD_UC_SYS_CTRL_MASK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `USBHD_UC_LOW_SPEED` reader - enable USB low speed: 0=12Mbps, 1=1.5Mbps"]
pub type USBHD_UC_LOW_SPEED_R = crate::BitReader;
#[doc = "Field `USBHD_UC_LOW_SPEED` writer - enable USB low speed: 0=12Mbps, 1=1.5Mbps"]
pub type USBHD_UC_LOW_SPEED_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_UC_HOST_MODE` reader - enable USB host mode: 0=device mode, 1=host mode"]
pub type RB_UC_HOST_MODE_R = crate::BitReader;
#[doc = "Field `RB_UC_HOST_MODE` writer - enable USB host mode: 0=device mode, 1=host mode"]
pub type RB_UC_HOST_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - DMA enable and DMA interrupt enable for USB"]
    #[inline(always)]
    pub fn usbhd_uc_dma_en(&self) -> USBHD_UC_DMA_EN_R {
        USBHD_UC_DMA_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - force clear FIFO and count of USB"]
    #[inline(always)]
    pub fn usbhd_uc_clr_all(&self) -> USBHD_UC_CLR_ALL_R {
        USBHD_UC_CLR_ALL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - force reset USB SIE, need software clear"]
    #[inline(always)]
    pub fn usbhd_uc_reset_sie(&self) -> USBHD_UC_RESET_SIE_R {
        USBHD_UC_RESET_SIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid"]
    #[inline(always)]
    pub fn usbhd_uc_int_busy(&self) -> USBHD_UC_INT_BUSY_R {
        USBHD_UC_INT_BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - USB device enable and internal pullup resistance enable"]
    #[inline(always)]
    pub fn usbhd_uc_sys_ctrl_mask(&self) -> USBHD_UC_SYS_CTRL_MASK_R {
        USBHD_UC_SYS_CTRL_MASK_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - enable USB low speed: 0=12Mbps, 1=1.5Mbps"]
    #[inline(always)]
    pub fn usbhd_uc_low_speed(&self) -> USBHD_UC_LOW_SPEED_R {
        USBHD_UC_LOW_SPEED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable USB host mode: 0=device mode, 1=host mode"]
    #[inline(always)]
    pub fn rb_uc_host_mode(&self) -> RB_UC_HOST_MODE_R {
        RB_UC_HOST_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA enable and DMA interrupt enable for USB"]
    #[inline(always)]
    #[must_use]
    pub fn usbhd_uc_dma_en(&mut self) -> USBHD_UC_DMA_EN_W<USBHD_BASE_CTRL_SPEC, 0> {
        USBHD_UC_DMA_EN_W::new(self)
    }
    #[doc = "Bit 1 - force clear FIFO and count of USB"]
    #[inline(always)]
    #[must_use]
    pub fn usbhd_uc_clr_all(&mut self) -> USBHD_UC_CLR_ALL_W<USBHD_BASE_CTRL_SPEC, 1> {
        USBHD_UC_CLR_ALL_W::new(self)
    }
    #[doc = "Bit 2 - force reset USB SIE, need software clear"]
    #[inline(always)]
    #[must_use]
    pub fn usbhd_uc_reset_sie(&mut self) -> USBHD_UC_RESET_SIE_W<USBHD_BASE_CTRL_SPEC, 2> {
        USBHD_UC_RESET_SIE_W::new(self)
    }
    #[doc = "Bit 3 - enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid"]
    #[inline(always)]
    #[must_use]
    pub fn usbhd_uc_int_busy(&mut self) -> USBHD_UC_INT_BUSY_W<USBHD_BASE_CTRL_SPEC, 3> {
        USBHD_UC_INT_BUSY_W::new(self)
    }
    #[doc = "Bits 4:5 - USB device enable and internal pullup resistance enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbhd_uc_sys_ctrl_mask(&mut self) -> USBHD_UC_SYS_CTRL_MASK_W<USBHD_BASE_CTRL_SPEC, 4> {
        USBHD_UC_SYS_CTRL_MASK_W::new(self)
    }
    #[doc = "Bit 6 - enable USB low speed: 0=12Mbps, 1=1.5Mbps"]
    #[inline(always)]
    #[must_use]
    pub fn usbhd_uc_low_speed(&mut self) -> USBHD_UC_LOW_SPEED_W<USBHD_BASE_CTRL_SPEC, 6> {
        USBHD_UC_LOW_SPEED_W::new(self)
    }
    #[doc = "Bit 7 - enable USB host mode: 0=device mode, 1=host mode"]
    #[inline(always)]
    #[must_use]
    pub fn rb_uc_host_mode(&mut self) -> RB_UC_HOST_MODE_W<USBHD_BASE_CTRL_SPEC, 7> {
        RB_UC_HOST_MODE_W::new(self)
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
#[doc = "USB base control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbhd_base_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbhd_base_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBHD_BASE_CTRL_SPEC;
impl crate::RegisterSpec for USBHD_BASE_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usbhd_base_ctrl::R`](R) reader structure"]
impl crate::Readable for USBHD_BASE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbhd_base_ctrl::W`](W) writer structure"]
impl crate::Writable for USBHD_BASE_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBHD_BASE_CTRL to value 0"]
impl crate::Resettable for USBHD_BASE_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
