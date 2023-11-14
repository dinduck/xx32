#[doc = "Register `USB_RX_LEN` reader"]
pub type R = crate::R<USB_RX_LEN_SPEC>;
#[doc = "Field `R16_USB_RX_LEN` reader - length of received bytes"]
pub type R16_USB_RX_LEN_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - length of received bytes"]
    #[inline(always)]
    pub fn r16_usb_rx_len(&self) -> R16_USB_RX_LEN_R {
        R16_USB_RX_LEN_R::new(self.bits)
    }
}
#[doc = "USB receiving length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_rx_len::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_RX_LEN_SPEC;
impl crate::RegisterSpec for USB_RX_LEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`usb_rx_len::R`](R) reader structure"]
impl crate::Readable for USB_RX_LEN_SPEC {}
#[doc = "`reset()` method sets USB_RX_LEN to value 0"]
impl crate::Resettable for USB_RX_LEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
