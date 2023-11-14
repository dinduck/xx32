#[doc = "Register `USB_DEV_AD` reader"]
pub type R = crate::R<USB_DEV_AD_SPEC>;
#[doc = "Register `USB_DEV_AD` writer"]
pub type W = crate::W<USB_DEV_AD_SPEC>;
#[doc = "Field `MASK_USB_ADDR` reader - bit mask for USB device address"]
pub type MASK_USB_ADDR_R = crate::FieldReader;
#[doc = "Field `MASK_USB_ADDR` writer - bit mask for USB device address"]
pub type MASK_USB_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `RB_UDA_GP_BIT` reader - general purpose bit"]
pub type RB_UDA_GP_BIT_R = crate::BitReader;
#[doc = "Field `RB_UDA_GP_BIT` writer - general purpose bit"]
pub type RB_UDA_GP_BIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - bit mask for USB device address"]
    #[inline(always)]
    pub fn mask_usb_addr(&self) -> MASK_USB_ADDR_R {
        MASK_USB_ADDR_R::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - general purpose bit"]
    #[inline(always)]
    pub fn rb_uda_gp_bit(&self) -> RB_UDA_GP_BIT_R {
        RB_UDA_GP_BIT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - bit mask for USB device address"]
    #[inline(always)]
    #[must_use]
    pub fn mask_usb_addr(&mut self) -> MASK_USB_ADDR_W<USB_DEV_AD_SPEC, 0> {
        MASK_USB_ADDR_W::new(self)
    }
    #[doc = "Bit 7 - general purpose bit"]
    #[inline(always)]
    #[must_use]
    pub fn rb_uda_gp_bit(&mut self) -> RB_UDA_GP_BIT_W<USB_DEV_AD_SPEC, 7> {
        RB_UDA_GP_BIT_W::new(self)
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
#[doc = "USB device address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_dev_ad::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_dev_ad::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_DEV_AD_SPEC;
impl crate::RegisterSpec for USB_DEV_AD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usb_dev_ad::R`](R) reader structure"]
impl crate::Readable for USB_DEV_AD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_dev_ad::W`](W) writer structure"]
impl crate::Writable for USB_DEV_AD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB_DEV_AD to value 0"]
impl crate::Resettable for USB_DEV_AD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
