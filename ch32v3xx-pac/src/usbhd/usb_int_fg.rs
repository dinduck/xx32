#[doc = "Register `USB_INT_FG` reader"]
pub type R = crate::R<USB_INT_FG_SPEC>;
#[doc = "Register `USB_INT_FG` writer"]
pub type W = crate::W<USB_INT_FG_SPEC>;
#[doc = "Field `RB_UIF_BUS_RST` reader - RB_UIF_BUS_RST"]
pub type RB_UIF_BUS_RST_R = crate::BitReader;
#[doc = "Field `RB_UIF_BUS_RST` writer - RB_UIF_BUS_RST"]
pub type RB_UIF_BUS_RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_UIF_TRANSFER` reader - USB transfer completion interrupt flag, direct bit address clear or write 1 to clear"]
pub type RB_UIF_TRANSFER_R = crate::BitReader;
#[doc = "Field `RB_UIF_TRANSFER` writer - USB transfer completion interrupt flag, direct bit address clear or write 1 to clear"]
pub type RB_UIF_TRANSFER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_UIF_SUSPEND` reader - USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear"]
pub type RB_UIF_SUSPEND_R = crate::BitReader;
#[doc = "Field `RB_UIF_SUSPEND` writer - USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear"]
pub type RB_UIF_SUSPEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_UIF_HST_SOF` reader - host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear"]
pub type RB_UIF_HST_SOF_R = crate::BitReader;
#[doc = "Field `RB_UIF_HST_SOF` writer - host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear"]
pub type RB_UIF_HST_SOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_UIF_FIFO_OV` reader - FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear"]
pub type RB_UIF_FIFO_OV_R = crate::BitReader;
#[doc = "Field `RB_UIF_FIFO_OV` writer - FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear"]
pub type RB_UIF_FIFO_OV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_U_SETUP_ACT` reader - USB_SETUP_ACT"]
pub type RB_U_SETUP_ACT_R = crate::BitReader;
#[doc = "Field `UIF_ISO_ACT` reader - UIF_ISO_ACT"]
pub type UIF_ISO_ACT_R = crate::BitReader;
#[doc = "Field `RB_U_IS_NAK` reader - RO, indicate current USB transfer is NAK received"]
pub type RB_U_IS_NAK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RB_UIF_BUS_RST"]
    #[inline(always)]
    pub fn rb_uif_bus_rst(&self) -> RB_UIF_BUS_RST_R {
        RB_UIF_BUS_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB transfer completion interrupt flag, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_uif_transfer(&self) -> RB_UIF_TRANSFER_R {
        RB_UIF_TRANSFER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_uif_suspend(&self) -> RB_UIF_SUSPEND_R {
        RB_UIF_SUSPEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_uif_hst_sof(&self) -> RB_UIF_HST_SOF_R {
        RB_UIF_HST_SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_uif_fifo_ov(&self) -> RB_UIF_FIFO_OV_R {
        RB_UIF_FIFO_OV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB_SETUP_ACT"]
    #[inline(always)]
    pub fn rb_u_setup_act(&self) -> RB_U_SETUP_ACT_R {
        RB_U_SETUP_ACT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UIF_ISO_ACT"]
    #[inline(always)]
    pub fn uif_iso_act(&self) -> UIF_ISO_ACT_R {
        UIF_ISO_ACT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RO, indicate current USB transfer is NAK received"]
    #[inline(always)]
    pub fn rb_u_is_nak(&self) -> RB_U_IS_NAK_R {
        RB_U_IS_NAK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RB_UIF_BUS_RST"]
    #[inline(always)]
    #[must_use]
    pub fn rb_uif_bus_rst(&mut self) -> RB_UIF_BUS_RST_W<USB_INT_FG_SPEC, 0> {
        RB_UIF_BUS_RST_W::new(self)
    }
    #[doc = "Bit 1 - USB transfer completion interrupt flag, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    #[must_use]
    pub fn rb_uif_transfer(&mut self) -> RB_UIF_TRANSFER_W<USB_INT_FG_SPEC, 1> {
        RB_UIF_TRANSFER_W::new(self)
    }
    #[doc = "Bit 2 - USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    #[must_use]
    pub fn rb_uif_suspend(&mut self) -> RB_UIF_SUSPEND_W<USB_INT_FG_SPEC, 2> {
        RB_UIF_SUSPEND_W::new(self)
    }
    #[doc = "Bit 3 - host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    #[must_use]
    pub fn rb_uif_hst_sof(&mut self) -> RB_UIF_HST_SOF_W<USB_INT_FG_SPEC, 3> {
        RB_UIF_HST_SOF_W::new(self)
    }
    #[doc = "Bit 4 - FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    #[must_use]
    pub fn rb_uif_fifo_ov(&mut self) -> RB_UIF_FIFO_OV_W<USB_INT_FG_SPEC, 4> {
        RB_UIF_FIFO_OV_W::new(self)
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
#[doc = "USB interrupt flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_int_fg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_int_fg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_INT_FG_SPEC;
impl crate::RegisterSpec for USB_INT_FG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usb_int_fg::R`](R) reader structure"]
impl crate::Readable for USB_INT_FG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_int_fg::W`](W) writer structure"]
impl crate::Writable for USB_INT_FG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB_INT_FG to value 0x20"]
impl crate::Resettable for USB_INT_FG_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
