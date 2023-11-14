#[doc = "Register `R8_USB_INT_EN` reader"]
pub type R = crate::R<R8_USB_INT_EN_SPEC>;
#[doc = "Register `R8_USB_INT_EN` writer"]
pub type W = crate::W<R8_USB_INT_EN_SPEC>;
#[doc = "Field `USBHD_UIE_BUS_RST__USBHD_UIE_DETECT` reader - enable interrupt for USB bus reset event for USB device mode"]
pub type USBHD_UIE_BUS_RST__USBHD_UIE_DETECT_R = crate::BitReader;
#[doc = "Field `USBHD_UIE_BUS_RST__USBHD_UIE_DETECT` writer - enable interrupt for USB bus reset event for USB device mode"]
pub type USBHD_UIE_BUS_RST__USBHD_UIE_DETECT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBHD_UIE_TRANSFER` reader - enable interrupt for USB transfer completion"]
pub type USBHD_UIE_TRANSFER_R = crate::BitReader;
#[doc = "Field `USBHD_UIE_TRANSFER` writer - enable interrupt for USB transfer completion"]
pub type USBHD_UIE_TRANSFER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBHD_UIE_SUSPEND` reader - enable interrupt for USB suspend or resume event"]
pub type USBHD_UIE_SUSPEND_R = crate::BitReader;
#[doc = "Field `USBHD_UIE_SUSPEND` writer - enable interrupt for USB suspend or resume event"]
pub type USBHD_UIE_SUSPEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBHD_UIE_HST_SOF` reader - enable interrupt for host SOF timer action for USB host mode"]
pub type USBHD_UIE_HST_SOF_R = crate::BitReader;
#[doc = "Field `USBHD_UIE_HST_SOF` writer - enable interrupt for host SOF timer action for USB host mode"]
pub type USBHD_UIE_HST_SOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBHD_UIE_FIFO_OV` reader - enable interrupt for FIFO overflow"]
pub type USBHD_UIE_FIFO_OV_R = crate::BitReader;
#[doc = "Field `USBHD_UIE_FIFO_OV` writer - enable interrupt for FIFO overflow"]
pub type USBHD_UIE_FIFO_OV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBHD_UIE_DEV_NAK` reader - enable interrupt for NAK responded for USB device mode"]
pub type USBHD_UIE_DEV_NAK_R = crate::BitReader;
#[doc = "Field `USBHD_UIE_DEV_NAK` writer - enable interrupt for NAK responded for USB device mode"]
pub type USBHD_UIE_DEV_NAK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBHD_UIE_DEV_SOF` reader - enable interrupt for SOF received for USB device mode"]
pub type USBHD_UIE_DEV_SOF_R = crate::BitReader;
#[doc = "Field `USBHD_UIE_DEV_SOF` writer - enable interrupt for SOF received for USB device mode"]
pub type USBHD_UIE_DEV_SOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - enable interrupt for USB bus reset event for USB device mode"]
    #[inline(always)]
    pub fn usbhd_uie_bus_rst__usbhd_uie_detect(&self) -> USBHD_UIE_BUS_RST__USBHD_UIE_DETECT_R {
        USBHD_UIE_BUS_RST__USBHD_UIE_DETECT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable interrupt for USB transfer completion"]
    #[inline(always)]
    pub fn usbhd_uie_transfer(&self) -> USBHD_UIE_TRANSFER_R {
        USBHD_UIE_TRANSFER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable interrupt for USB suspend or resume event"]
    #[inline(always)]
    pub fn usbhd_uie_suspend(&self) -> USBHD_UIE_SUSPEND_R {
        USBHD_UIE_SUSPEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable interrupt for host SOF timer action for USB host mode"]
    #[inline(always)]
    pub fn usbhd_uie_hst_sof(&self) -> USBHD_UIE_HST_SOF_R {
        USBHD_UIE_HST_SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable interrupt for FIFO overflow"]
    #[inline(always)]
    pub fn usbhd_uie_fifo_ov(&self) -> USBHD_UIE_FIFO_OV_R {
        USBHD_UIE_FIFO_OV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - enable interrupt for NAK responded for USB device mode"]
    #[inline(always)]
    pub fn usbhd_uie_dev_nak(&self) -> USBHD_UIE_DEV_NAK_R {
        USBHD_UIE_DEV_NAK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable interrupt for SOF received for USB device mode"]
    #[inline(always)]
    pub fn usbhd_uie_dev_sof(&self) -> USBHD_UIE_DEV_SOF_R {
        USBHD_UIE_DEV_SOF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable interrupt for USB bus reset event for USB device mode"]
    #[inline(always)]
    #[must_use]
    pub fn usbhd_uie_bus_rst__usbhd_uie_detect(
        &mut self,
    ) -> USBHD_UIE_BUS_RST__USBHD_UIE_DETECT_W<R8_USB_INT_EN_SPEC, 0> {
        USBHD_UIE_BUS_RST__USBHD_UIE_DETECT_W::new(self)
    }
    #[doc = "Bit 1 - enable interrupt for USB transfer completion"]
    #[inline(always)]
    #[must_use]
    pub fn usbhd_uie_transfer(&mut self) -> USBHD_UIE_TRANSFER_W<R8_USB_INT_EN_SPEC, 1> {
        USBHD_UIE_TRANSFER_W::new(self)
    }
    #[doc = "Bit 2 - enable interrupt for USB suspend or resume event"]
    #[inline(always)]
    #[must_use]
    pub fn usbhd_uie_suspend(&mut self) -> USBHD_UIE_SUSPEND_W<R8_USB_INT_EN_SPEC, 2> {
        USBHD_UIE_SUSPEND_W::new(self)
    }
    #[doc = "Bit 3 - enable interrupt for host SOF timer action for USB host mode"]
    #[inline(always)]
    #[must_use]
    pub fn usbhd_uie_hst_sof(&mut self) -> USBHD_UIE_HST_SOF_W<R8_USB_INT_EN_SPEC, 3> {
        USBHD_UIE_HST_SOF_W::new(self)
    }
    #[doc = "Bit 4 - enable interrupt for FIFO overflow"]
    #[inline(always)]
    #[must_use]
    pub fn usbhd_uie_fifo_ov(&mut self) -> USBHD_UIE_FIFO_OV_W<R8_USB_INT_EN_SPEC, 4> {
        USBHD_UIE_FIFO_OV_W::new(self)
    }
    #[doc = "Bit 6 - enable interrupt for NAK responded for USB device mode"]
    #[inline(always)]
    #[must_use]
    pub fn usbhd_uie_dev_nak(&mut self) -> USBHD_UIE_DEV_NAK_W<R8_USB_INT_EN_SPEC, 6> {
        USBHD_UIE_DEV_NAK_W::new(self)
    }
    #[doc = "Bit 7 - enable interrupt for SOF received for USB device mode"]
    #[inline(always)]
    #[must_use]
    pub fn usbhd_uie_dev_sof(&mut self) -> USBHD_UIE_DEV_SOF_W<R8_USB_INT_EN_SPEC, 7> {
        USBHD_UIE_DEV_SOF_W::new(self)
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
#[doc = "USB interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_usb_int_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_usb_int_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8_USB_INT_EN_SPEC;
impl crate::RegisterSpec for R8_USB_INT_EN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_usb_int_en::R`](R) reader structure"]
impl crate::Readable for R8_USB_INT_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`r8_usb_int_en::W`](W) writer structure"]
impl crate::Writable for R8_USB_INT_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets R8_USB_INT_EN to value 0"]
impl crate::Resettable for R8_USB_INT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
