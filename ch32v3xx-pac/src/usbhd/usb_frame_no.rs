#[doc = "Register `USB_FRAME_NO` reader"]
pub type R = crate::R<USB_FRAME_NO_SPEC>;
#[doc = "Field `USB_FRAME_NO` reader - USB_FRAME_NO"]
pub type USB_FRAME_NO_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - USB_FRAME_NO"]
    #[inline(always)]
    pub fn usb_frame_no(&self) -> USB_FRAME_NO_R {
        USB_FRAME_NO_R::new(self.bits)
    }
}
#[doc = "USB_FRAME_NO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_frame_no::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_FRAME_NO_SPEC;
impl crate::RegisterSpec for USB_FRAME_NO_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`usb_frame_no::R`](R) reader structure"]
impl crate::Readable for USB_FRAME_NO_SPEC {}
#[doc = "`reset()` method sets USB_FRAME_NO to value 0"]
impl crate::Resettable for USB_FRAME_NO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
