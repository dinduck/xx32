#[doc = "Register `R16_USB_RX_LEN` reader"]
pub type R = crate::R<R16_USB_RX_LEN_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<R16_USB_RX_LEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "USB receiving length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r16_usb_rx_len::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R16_USB_RX_LEN_SPEC;
impl crate::RegisterSpec for R16_USB_RX_LEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`r16_usb_rx_len::R`](R) reader structure"]
impl crate::Readable for R16_USB_RX_LEN_SPEC {}
#[doc = "`reset()` method sets R16_USB_RX_LEN to value 0"]
impl crate::Resettable for R16_USB_RX_LEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
