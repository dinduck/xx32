#[doc = "Register `USB_OTG_SR` reader"]
pub type R = crate::R<USB_OTG_SR_SPEC>;
#[doc = "Register `USB_OTG_SR` writer"]
pub type W = crate::W<USB_OTG_SR_SPEC>;
#[doc = "Field `USB_OTG_SR_VBUS_VLD` reader - usb otg status"]
pub type USB_OTG_SR_VBUS_VLD_R = crate::BitReader;
#[doc = "Field `USB_OTG_SR_VBUS_VLD` writer - usb otg status"]
pub type USB_OTG_SR_VBUS_VLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_OTG_SR_SESS_VLD` reader - usb otg status"]
pub type USB_OTG_SR_SESS_VLD_R = crate::BitReader;
#[doc = "Field `USB_OTG_SR_SESS_VLD` writer - usb otg status"]
pub type USB_OTG_SR_SESS_VLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_OTG_SR_SESS_END` reader - usb otg status"]
pub type USB_OTG_SR_SESS_END_R = crate::BitReader;
#[doc = "Field `USB_OTG_SR_SESS_END` writer - usb otg status"]
pub type USB_OTG_SR_SESS_END_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_OTG_SR_ID_DIG` reader - usb otg status"]
pub type USB_OTG_SR_ID_DIG_R = crate::BitReader;
#[doc = "Field `USB_OTG_SR_ID_DIG` writer - usb otg status"]
pub type USB_OTG_SR_ID_DIG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - usb otg status"]
    #[inline(always)]
    pub fn usb_otg_sr_vbus_vld(&self) -> USB_OTG_SR_VBUS_VLD_R {
        USB_OTG_SR_VBUS_VLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - usb otg status"]
    #[inline(always)]
    pub fn usb_otg_sr_sess_vld(&self) -> USB_OTG_SR_SESS_VLD_R {
        USB_OTG_SR_SESS_VLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - usb otg status"]
    #[inline(always)]
    pub fn usb_otg_sr_sess_end(&self) -> USB_OTG_SR_SESS_END_R {
        USB_OTG_SR_SESS_END_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - usb otg status"]
    #[inline(always)]
    pub fn usb_otg_sr_id_dig(&self) -> USB_OTG_SR_ID_DIG_R {
        USB_OTG_SR_ID_DIG_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - usb otg status"]
    #[inline(always)]
    #[must_use]
    pub fn usb_otg_sr_vbus_vld(&mut self) -> USB_OTG_SR_VBUS_VLD_W<USB_OTG_SR_SPEC, 0> {
        USB_OTG_SR_VBUS_VLD_W::new(self)
    }
    #[doc = "Bit 1 - usb otg status"]
    #[inline(always)]
    #[must_use]
    pub fn usb_otg_sr_sess_vld(&mut self) -> USB_OTG_SR_SESS_VLD_W<USB_OTG_SR_SPEC, 1> {
        USB_OTG_SR_SESS_VLD_W::new(self)
    }
    #[doc = "Bit 2 - usb otg status"]
    #[inline(always)]
    #[must_use]
    pub fn usb_otg_sr_sess_end(&mut self) -> USB_OTG_SR_SESS_END_W<USB_OTG_SR_SPEC, 2> {
        USB_OTG_SR_SESS_END_W::new(self)
    }
    #[doc = "Bit 3 - usb otg status"]
    #[inline(always)]
    #[must_use]
    pub fn usb_otg_sr_id_dig(&mut self) -> USB_OTG_SR_ID_DIG_W<USB_OTG_SR_SPEC, 3> {
        USB_OTG_SR_ID_DIG_W::new(self)
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
#[doc = "usb otg status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_otg_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_otg_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_OTG_SR_SPEC;
impl crate::RegisterSpec for USB_OTG_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_otg_sr::R`](R) reader structure"]
impl crate::Readable for USB_OTG_SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_otg_sr::W`](W) writer structure"]
impl crate::Writable for USB_OTG_SR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB_OTG_SR to value 0"]
impl crate::Resettable for USB_OTG_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
