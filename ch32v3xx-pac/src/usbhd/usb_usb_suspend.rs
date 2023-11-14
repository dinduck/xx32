#[doc = "Register `USB_USB_SUSPEND` reader"]
pub type R = crate::R<USB_USB_SUSPEND_SPEC>;
#[doc = "Register `USB_USB_SUSPEND` writer"]
pub type W = crate::W<USB_USB_SUSPEND_SPEC>;
#[doc = "Field `USB_SYS_MOD` reader - USB_SYS_MOD"]
pub type USB_SYS_MOD_R = crate::FieldReader;
#[doc = "Field `USB_SYS_MOD` writer - USB_SYS_MOD"]
pub type USB_SYS_MOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `USB_WAKEUP` reader - remote resume"]
pub type USB_WAKEUP_R = crate::BitReader;
#[doc = "Field `USB_WAKEUP` writer - remote resume"]
pub type USB_WAKEUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_LINESTATE` reader - USB_LINESTATE"]
pub type USB_LINESTATE_R = crate::FieldReader;
#[doc = "Field `USB_LINESTATE` writer - USB_LINESTATE"]
pub type USB_LINESTATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - USB_SYS_MOD"]
    #[inline(always)]
    pub fn usb_sys_mod(&self) -> USB_SYS_MOD_R {
        USB_SYS_MOD_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - remote resume"]
    #[inline(always)]
    pub fn usb_wakeup(&self) -> USB_WAKEUP_R {
        USB_WAKEUP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - USB_LINESTATE"]
    #[inline(always)]
    pub fn usb_linestate(&self) -> USB_LINESTATE_R {
        USB_LINESTATE_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - USB_SYS_MOD"]
    #[inline(always)]
    #[must_use]
    pub fn usb_sys_mod(&mut self) -> USB_SYS_MOD_W<USB_USB_SUSPEND_SPEC, 0> {
        USB_SYS_MOD_W::new(self)
    }
    #[doc = "Bit 2 - remote resume"]
    #[inline(always)]
    #[must_use]
    pub fn usb_wakeup(&mut self) -> USB_WAKEUP_W<USB_USB_SUSPEND_SPEC, 2> {
        USB_WAKEUP_W::new(self)
    }
    #[doc = "Bits 4:5 - USB_LINESTATE"]
    #[inline(always)]
    #[must_use]
    pub fn usb_linestate(&mut self) -> USB_LINESTATE_W<USB_USB_SUSPEND_SPEC, 4> {
        USB_LINESTATE_W::new(self)
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
#[doc = "indicate USB suspend status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_usb_suspend::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_usb_suspend::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_USB_SUSPEND_SPEC;
impl crate::RegisterSpec for USB_USB_SUSPEND_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usb_usb_suspend::R`](R) reader structure"]
impl crate::Readable for USB_USB_SUSPEND_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_usb_suspend::W`](W) writer structure"]
impl crate::Writable for USB_USB_SUSPEND_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB_USB_SUSPEND to value 0"]
impl crate::Resettable for USB_USB_SUSPEND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
