#[doc = "Register `USB_INT_ST` reader"]
pub type R = crate::R<USB_INT_ST_SPEC>;
#[doc = "Field `MASK_UIS_H_RES__MASK_UIS_ENDP` reader - RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received;RO, bit mask of current transfer endpoint number for USB device mode"]
pub type MASK_UIS_H_RES__MASK_UIS_ENDP_R = crate::FieldReader;
#[doc = "Field `MASK_UIS_TOKEN` reader - RO, bit mask of current token PID code received for USB device mode"]
pub type MASK_UIS_TOKEN_R = crate::FieldReader;
#[doc = "Field `RB_UIS_TOG_OK` reader - RO, indicate current USB transfer toggle is OK"]
pub type RB_UIS_TOG_OK_R = crate::BitReader;
#[doc = "Field `RB_UIS_IS_NAK` reader - RO, indicate current USB transfer is NAK received for USB device mode"]
pub type RB_UIS_IS_NAK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received;RO, bit mask of current transfer endpoint number for USB device mode"]
    #[inline(always)]
    pub fn mask_uis_h_res__mask_uis_endp(&self) -> MASK_UIS_H_RES__MASK_UIS_ENDP_R {
        MASK_UIS_H_RES__MASK_UIS_ENDP_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - RO, bit mask of current token PID code received for USB device mode"]
    #[inline(always)]
    pub fn mask_uis_token(&self) -> MASK_UIS_TOKEN_R {
        MASK_UIS_TOKEN_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - RO, indicate current USB transfer toggle is OK"]
    #[inline(always)]
    pub fn rb_uis_tog_ok(&self) -> RB_UIS_TOG_OK_R {
        RB_UIS_TOG_OK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RO, indicate current USB transfer is NAK received for USB device mode"]
    #[inline(always)]
    pub fn rb_uis_is_nak(&self) -> RB_UIS_IS_NAK_R {
        RB_UIS_IS_NAK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "USB interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_INT_ST_SPEC;
impl crate::RegisterSpec for USB_INT_ST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usb_int_st::R`](R) reader structure"]
impl crate::Readable for USB_INT_ST_SPEC {}
#[doc = "`reset()` method sets USB_INT_ST to value 0"]
impl crate::Resettable for USB_INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
