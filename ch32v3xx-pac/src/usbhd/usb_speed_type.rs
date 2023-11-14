#[doc = "Register `USB_SPEED_TYPE` reader"]
pub type R = crate::R<USB_SPEED_TYPE_SPEC>;
#[doc = "Field `USB_SPEED_TYPE` reader - USB_SPEED_TYPE"]
pub type USB_SPEED_TYPE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - USB_SPEED_TYPE"]
    #[inline(always)]
    pub fn usb_speed_type(&self) -> USB_SPEED_TYPE_R {
        USB_SPEED_TYPE_R::new(self.bits & 3)
    }
}
#[doc = "USB_SPEED_TYPE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_speed_type::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_SPEED_TYPE_SPEC;
impl crate::RegisterSpec for USB_SPEED_TYPE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usb_speed_type::R`](R) reader structure"]
impl crate::Readable for USB_SPEED_TYPE_SPEC {}
#[doc = "`reset()` method sets USB_SPEED_TYPE to value 0"]
impl crate::Resettable for USB_SPEED_TYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
