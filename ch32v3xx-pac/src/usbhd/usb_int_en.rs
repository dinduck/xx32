#[doc = "Register `USB_INT_EN` reader"]
pub type R = crate::R<USB_INT_EN_SPEC>;
#[doc = "Register `USB_INT_EN` writer"]
pub type W = crate::W<USB_INT_EN_SPEC>;
#[doc = "Field `RB_UIE_BUS_RST__RB_UIE_DETECT` reader - enable interrupt for USB bus reset event for USB device mode;enable interrupt for USB device detected event for USB host mode"]
pub type RB_UIE_BUS_RST__RB_UIE_DETECT_R = crate::BitReader;
#[doc = "Field `RB_UIE_BUS_RST__RB_UIE_DETECT` writer - enable interrupt for USB bus reset event for USB device mode;enable interrupt for USB device detected event for USB host mode"]
pub type RB_UIE_BUS_RST__RB_UIE_DETECT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_UIE_TRANSFER` reader - enable interrupt for USB transfer completion"]
pub type RB_UIE_TRANSFER_R = crate::BitReader;
#[doc = "Field `RB_UIE_TRANSFER` writer - enable interrupt for USB transfer completion"]
pub type RB_UIE_TRANSFER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_UIE_SUSPEND` reader - enable interrupt for USB suspend or resume event"]
pub type RB_UIE_SUSPEND_R = crate::BitReader;
#[doc = "Field `RB_UIE_SUSPEND` writer - enable interrupt for USB suspend or resume event"]
pub type RB_UIE_SUSPEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_UIE_SOF_ACT` reader - indicate host SOF timer action status for USB host"]
pub type RB_UIE_SOF_ACT_R = crate::BitReader;
#[doc = "Field `RB_UIE_SOF_ACT` writer - indicate host SOF timer action status for USB host"]
pub type RB_UIE_SOF_ACT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_UIE_FIFO_OV` reader - enable interrupt for FIFO overflow"]
pub type RB_UIE_FIFO_OV_R = crate::BitReader;
#[doc = "Field `RB_UIE_FIFO_OV` writer - enable interrupt for FIFO overflow"]
pub type RB_UIE_FIFO_OV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_UIE_SETUP_ACT` reader - indicate host SETUP timer action status for USB host"]
pub type RB_UIE_SETUP_ACT_R = crate::BitReader;
#[doc = "Field `RB_UIE_SETUP_ACT` writer - indicate host SETUP timer action status for USB host"]
pub type RB_UIE_SETUP_ACT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_UIE_ISO_ACT` reader - enable interrupt for NAK responded for USB device mode"]
pub type RB_UIE_ISO_ACT_R = crate::BitReader;
#[doc = "Field `RB_UIE_ISO_ACT` writer - enable interrupt for NAK responded for USB device mode"]
pub type RB_UIE_ISO_ACT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_UIE_DEV_NAK` reader - enable interrupt for NAK responded for USB device mode"]
pub type RB_UIE_DEV_NAK_R = crate::BitReader;
#[doc = "Field `RB_UIE_DEV_NAK` writer - enable interrupt for NAK responded for USB device mode"]
pub type RB_UIE_DEV_NAK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - enable interrupt for USB bus reset event for USB device mode;enable interrupt for USB device detected event for USB host mode"]
    #[inline(always)]
    pub fn rb_uie_bus_rst__rb_uie_detect(&self) -> RB_UIE_BUS_RST__RB_UIE_DETECT_R {
        RB_UIE_BUS_RST__RB_UIE_DETECT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable interrupt for USB transfer completion"]
    #[inline(always)]
    pub fn rb_uie_transfer(&self) -> RB_UIE_TRANSFER_R {
        RB_UIE_TRANSFER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable interrupt for USB suspend or resume event"]
    #[inline(always)]
    pub fn rb_uie_suspend(&self) -> RB_UIE_SUSPEND_R {
        RB_UIE_SUSPEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - indicate host SOF timer action status for USB host"]
    #[inline(always)]
    pub fn rb_uie_sof_act(&self) -> RB_UIE_SOF_ACT_R {
        RB_UIE_SOF_ACT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable interrupt for FIFO overflow"]
    #[inline(always)]
    pub fn rb_uie_fifo_ov(&self) -> RB_UIE_FIFO_OV_R {
        RB_UIE_FIFO_OV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - indicate host SETUP timer action status for USB host"]
    #[inline(always)]
    pub fn rb_uie_setup_act(&self) -> RB_UIE_SETUP_ACT_R {
        RB_UIE_SETUP_ACT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - enable interrupt for NAK responded for USB device mode"]
    #[inline(always)]
    pub fn rb_uie_iso_act(&self) -> RB_UIE_ISO_ACT_R {
        RB_UIE_ISO_ACT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable interrupt for NAK responded for USB device mode"]
    #[inline(always)]
    pub fn rb_uie_dev_nak(&self) -> RB_UIE_DEV_NAK_R {
        RB_UIE_DEV_NAK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable interrupt for USB bus reset event for USB device mode;enable interrupt for USB device detected event for USB host mode"]
    #[inline(always)]
    #[must_use]
    pub fn rb_uie_bus_rst__rb_uie_detect(
        &mut self,
    ) -> RB_UIE_BUS_RST__RB_UIE_DETECT_W<USB_INT_EN_SPEC, 0> {
        RB_UIE_BUS_RST__RB_UIE_DETECT_W::new(self)
    }
    #[doc = "Bit 1 - enable interrupt for USB transfer completion"]
    #[inline(always)]
    #[must_use]
    pub fn rb_uie_transfer(&mut self) -> RB_UIE_TRANSFER_W<USB_INT_EN_SPEC, 1> {
        RB_UIE_TRANSFER_W::new(self)
    }
    #[doc = "Bit 2 - enable interrupt for USB suspend or resume event"]
    #[inline(always)]
    #[must_use]
    pub fn rb_uie_suspend(&mut self) -> RB_UIE_SUSPEND_W<USB_INT_EN_SPEC, 2> {
        RB_UIE_SUSPEND_W::new(self)
    }
    #[doc = "Bit 3 - indicate host SOF timer action status for USB host"]
    #[inline(always)]
    #[must_use]
    pub fn rb_uie_sof_act(&mut self) -> RB_UIE_SOF_ACT_W<USB_INT_EN_SPEC, 3> {
        RB_UIE_SOF_ACT_W::new(self)
    }
    #[doc = "Bit 4 - enable interrupt for FIFO overflow"]
    #[inline(always)]
    #[must_use]
    pub fn rb_uie_fifo_ov(&mut self) -> RB_UIE_FIFO_OV_W<USB_INT_EN_SPEC, 4> {
        RB_UIE_FIFO_OV_W::new(self)
    }
    #[doc = "Bit 5 - indicate host SETUP timer action status for USB host"]
    #[inline(always)]
    #[must_use]
    pub fn rb_uie_setup_act(&mut self) -> RB_UIE_SETUP_ACT_W<USB_INT_EN_SPEC, 5> {
        RB_UIE_SETUP_ACT_W::new(self)
    }
    #[doc = "Bit 6 - enable interrupt for NAK responded for USB device mode"]
    #[inline(always)]
    #[must_use]
    pub fn rb_uie_iso_act(&mut self) -> RB_UIE_ISO_ACT_W<USB_INT_EN_SPEC, 6> {
        RB_UIE_ISO_ACT_W::new(self)
    }
    #[doc = "Bit 7 - enable interrupt for NAK responded for USB device mode"]
    #[inline(always)]
    #[must_use]
    pub fn rb_uie_dev_nak(&mut self) -> RB_UIE_DEV_NAK_W<USB_INT_EN_SPEC, 7> {
        RB_UIE_DEV_NAK_W::new(self)
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
#[doc = "USB interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_int_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_int_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_INT_EN_SPEC;
impl crate::RegisterSpec for USB_INT_EN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usb_int_en::R`](R) reader structure"]
impl crate::Readable for USB_INT_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_int_en::W`](W) writer structure"]
impl crate::Writable for USB_INT_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB_INT_EN to value 0"]
impl crate::Resettable for USB_INT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
