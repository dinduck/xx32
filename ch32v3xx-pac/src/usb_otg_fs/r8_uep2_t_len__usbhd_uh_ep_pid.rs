#[doc = "Register `R8_UEP2_T_LEN__USBHD_UH_EP_PID` reader"]
pub type R = crate::R<R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC>;
#[doc = "Register `R8_UEP2_T_LEN__USBHD_UH_EP_PID` writer"]
pub type W = crate::W<R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC>;
#[doc = "Field `USBHD_UH_ENDP_MASK` reader - bit mask of endpoint number for USB host transfer"]
pub type USBHD_UH_ENDP_MASK_R = crate::FieldReader;
#[doc = "Field `USBHD_UH_ENDP_MASK` writer - bit mask of endpoint number for USB host transfer"]
pub type USBHD_UH_ENDP_MASK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `USBHD_UH_TOKEN_MASK` reader - bit mask of token PID for USB host transfer"]
pub type USBHD_UH_TOKEN_MASK_R = crate::FieldReader;
#[doc = "Field `USBHD_UH_TOKEN_MASK` writer - bit mask of token PID for USB host transfer"]
pub type USBHD_UH_TOKEN_MASK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - bit mask of endpoint number for USB host transfer"]
    #[inline(always)]
    pub fn usbhd_uh_endp_mask(&self) -> USBHD_UH_ENDP_MASK_R {
        USBHD_UH_ENDP_MASK_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - bit mask of token PID for USB host transfer"]
    #[inline(always)]
    pub fn usbhd_uh_token_mask(&self) -> USBHD_UH_TOKEN_MASK_R {
        USBHD_UH_TOKEN_MASK_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - bit mask of endpoint number for USB host transfer"]
    #[inline(always)]
    #[must_use]
    pub fn usbhd_uh_endp_mask(
        &mut self,
    ) -> USBHD_UH_ENDP_MASK_W<R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC, 0> {
        USBHD_UH_ENDP_MASK_W::new(self)
    }
    #[doc = "Bits 4:7 - bit mask of token PID for USB host transfer"]
    #[inline(always)]
    #[must_use]
    pub fn usbhd_uh_token_mask(
        &mut self,
    ) -> USBHD_UH_TOKEN_MASK_W<R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC, 4> {
        USBHD_UH_TOKEN_MASK_W::new(self)
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
#[doc = "endpoint 2 transmittal length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep2_t_len__usbhd_uh_ep_pid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep2_t_len__usbhd_uh_ep_pid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC;
impl crate::RegisterSpec for R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uep2_t_len__usbhd_uh_ep_pid::R`](R) reader structure"]
impl crate::Readable for R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`r8_uep2_t_len__usbhd_uh_ep_pid::W`](W) writer structure"]
impl crate::Writable for R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets R8_UEP2_T_LEN__USBHD_UH_EP_PID to value 0"]
impl crate::Resettable for R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
