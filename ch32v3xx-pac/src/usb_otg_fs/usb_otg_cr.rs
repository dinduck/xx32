#[doc = "Register `USB_OTG_CR` reader"]
pub type R = crate::R<USB_OTG_CR_SPEC>;
#[doc = "Register `USB_OTG_CR` writer"]
pub type W = crate::W<USB_OTG_CR_SPEC>;
#[doc = "Field `USB_OTG_CR_DISCHARGEVBUS` reader - usb otg control"]
pub type USB_OTG_CR_DISCHARGEVBUS_R = crate::BitReader;
#[doc = "Field `USB_OTG_CR_DISCHARGEVBUS` writer - usb otg control"]
pub type USB_OTG_CR_DISCHARGEVBUS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_OTG_CR_CHARGEVBUS` reader - usb otg control"]
pub type USB_OTG_CR_CHARGEVBUS_R = crate::BitReader;
#[doc = "Field `USB_OTG_CR_CHARGEVBUS` writer - usb otg control"]
pub type USB_OTG_CR_CHARGEVBUS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_OTG_CR_IDPU` reader - usb otg control"]
pub type USB_OTG_CR_IDPU_R = crate::BitReader;
#[doc = "Field `USB_OTG_CR_IDPU` writer - usb otg control"]
pub type USB_OTG_CR_IDPU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_OTG_CR_OTG_EN` reader - usb otg control"]
pub type USB_OTG_CR_OTG_EN_R = crate::BitReader;
#[doc = "Field `USB_OTG_CR_OTG_EN` writer - usb otg control"]
pub type USB_OTG_CR_OTG_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_OTG_CR_VBUS` reader - usb otg control"]
pub type USB_OTG_CR_VBUS_R = crate::BitReader;
#[doc = "Field `USB_OTG_CR_VBUS` writer - usb otg control"]
pub type USB_OTG_CR_VBUS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_OTG_CR_SESS` reader - usb otg control"]
pub type USB_OTG_CR_SESS_R = crate::BitReader;
#[doc = "Field `USB_OTG_CR_SESS` writer - usb otg control"]
pub type USB_OTG_CR_SESS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - usb otg control"]
    #[inline(always)]
    pub fn usb_otg_cr_dischargevbus(&self) -> USB_OTG_CR_DISCHARGEVBUS_R {
        USB_OTG_CR_DISCHARGEVBUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - usb otg control"]
    #[inline(always)]
    pub fn usb_otg_cr_chargevbus(&self) -> USB_OTG_CR_CHARGEVBUS_R {
        USB_OTG_CR_CHARGEVBUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - usb otg control"]
    #[inline(always)]
    pub fn usb_otg_cr_idpu(&self) -> USB_OTG_CR_IDPU_R {
        USB_OTG_CR_IDPU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - usb otg control"]
    #[inline(always)]
    pub fn usb_otg_cr_otg_en(&self) -> USB_OTG_CR_OTG_EN_R {
        USB_OTG_CR_OTG_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - usb otg control"]
    #[inline(always)]
    pub fn usb_otg_cr_vbus(&self) -> USB_OTG_CR_VBUS_R {
        USB_OTG_CR_VBUS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - usb otg control"]
    #[inline(always)]
    pub fn usb_otg_cr_sess(&self) -> USB_OTG_CR_SESS_R {
        USB_OTG_CR_SESS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - usb otg control"]
    #[inline(always)]
    #[must_use]
    pub fn usb_otg_cr_dischargevbus(&mut self) -> USB_OTG_CR_DISCHARGEVBUS_W<USB_OTG_CR_SPEC, 0> {
        USB_OTG_CR_DISCHARGEVBUS_W::new(self)
    }
    #[doc = "Bit 1 - usb otg control"]
    #[inline(always)]
    #[must_use]
    pub fn usb_otg_cr_chargevbus(&mut self) -> USB_OTG_CR_CHARGEVBUS_W<USB_OTG_CR_SPEC, 1> {
        USB_OTG_CR_CHARGEVBUS_W::new(self)
    }
    #[doc = "Bit 2 - usb otg control"]
    #[inline(always)]
    #[must_use]
    pub fn usb_otg_cr_idpu(&mut self) -> USB_OTG_CR_IDPU_W<USB_OTG_CR_SPEC, 2> {
        USB_OTG_CR_IDPU_W::new(self)
    }
    #[doc = "Bit 3 - usb otg control"]
    #[inline(always)]
    #[must_use]
    pub fn usb_otg_cr_otg_en(&mut self) -> USB_OTG_CR_OTG_EN_W<USB_OTG_CR_SPEC, 3> {
        USB_OTG_CR_OTG_EN_W::new(self)
    }
    #[doc = "Bit 4 - usb otg control"]
    #[inline(always)]
    #[must_use]
    pub fn usb_otg_cr_vbus(&mut self) -> USB_OTG_CR_VBUS_W<USB_OTG_CR_SPEC, 4> {
        USB_OTG_CR_VBUS_W::new(self)
    }
    #[doc = "Bit 5 - usb otg control"]
    #[inline(always)]
    #[must_use]
    pub fn usb_otg_cr_sess(&mut self) -> USB_OTG_CR_SESS_W<USB_OTG_CR_SPEC, 5> {
        USB_OTG_CR_SESS_W::new(self)
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
#[doc = "usb otg control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_otg_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_otg_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_OTG_CR_SPEC;
impl crate::RegisterSpec for USB_OTG_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_otg_cr::R`](R) reader structure"]
impl crate::Readable for USB_OTG_CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_otg_cr::W`](W) writer structure"]
impl crate::Writable for USB_OTG_CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB_OTG_CR to value 0"]
impl crate::Resettable for USB_OTG_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
