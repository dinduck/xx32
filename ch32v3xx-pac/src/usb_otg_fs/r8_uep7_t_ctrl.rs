#[doc = "Register `R8_UEP7_T_CTRL` reader"]
pub type R = crate::R<R8_UEP7_T_CTRL_SPEC>;
#[doc = "Register `R8_UEP7_T_CTRL` writer"]
pub type W = crate::W<R8_UEP7_T_CTRL_SPEC>;
#[doc = "Field `MASK_UEP_T_RES` reader - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
pub type MASK_UEP_T_RES_R = crate::FieldReader;
#[doc = "Field `MASK_UEP_T_RES` writer - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
pub type MASK_UEP_T_RES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `USBHD_UEP_T_TOG___USBHD_UH_T_TOG` reader - prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1"]
pub type USBHD_UEP_T_TOG___USBHD_UH_T_TOG_R = crate::BitReader;
#[doc = "Field `USBHD_UEP_T_TOG___USBHD_UH_T_TOG` writer - prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1"]
pub type USBHD_UEP_T_TOG___USBHD_UH_T_TOG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBHD_UEP_AUTO_TOG__USBHD_UH_T_AUTO_TOG` reader - enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle"]
pub type USBHD_UEP_AUTO_TOG__USBHD_UH_T_AUTO_TOG_R = crate::BitReader;
#[doc = "Field `USBHD_UEP_AUTO_TOG__USBHD_UH_T_AUTO_TOG` writer - enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle"]
pub type USBHD_UEP_AUTO_TOG__USBHD_UH_T_AUTO_TOG_W<'a, REG, const O: u8> =
    crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
    #[inline(always)]
    pub fn mask_uep_t_res(&self) -> MASK_UEP_T_RES_R {
        MASK_UEP_T_RES_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1"]
    #[inline(always)]
    pub fn usbhd_uep_t_tog___usbhd_uh_t_tog(&self) -> USBHD_UEP_T_TOG___USBHD_UH_T_TOG_R {
        USBHD_UEP_T_TOG___USBHD_UH_T_TOG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle"]
    #[inline(always)]
    pub fn usbhd_uep_auto_tog__usbhd_uh_t_auto_tog(
        &self,
    ) -> USBHD_UEP_AUTO_TOG__USBHD_UH_T_AUTO_TOG_R {
        USBHD_UEP_AUTO_TOG__USBHD_UH_T_AUTO_TOG_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
    #[inline(always)]
    #[must_use]
    pub fn mask_uep_t_res(&mut self) -> MASK_UEP_T_RES_W<R8_UEP7_T_CTRL_SPEC, 0> {
        MASK_UEP_T_RES_W::new(self)
    }
    #[doc = "Bit 2 - prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1"]
    #[inline(always)]
    #[must_use]
    pub fn usbhd_uep_t_tog___usbhd_uh_t_tog(
        &mut self,
    ) -> USBHD_UEP_T_TOG___USBHD_UH_T_TOG_W<R8_UEP7_T_CTRL_SPEC, 2> {
        USBHD_UEP_T_TOG___USBHD_UH_T_TOG_W::new(self)
    }
    #[doc = "Bit 3 - enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle"]
    #[inline(always)]
    #[must_use]
    pub fn usbhd_uep_auto_tog__usbhd_uh_t_auto_tog(
        &mut self,
    ) -> USBHD_UEP_AUTO_TOG__USBHD_UH_T_AUTO_TOG_W<R8_UEP7_T_CTRL_SPEC, 3> {
        USBHD_UEP_AUTO_TOG__USBHD_UH_T_AUTO_TOG_W::new(self)
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
#[doc = "endpoint 7 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep7_t_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep7_t_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8_UEP7_T_CTRL_SPEC;
impl crate::RegisterSpec for R8_UEP7_T_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uep7_t_ctrl::R`](R) reader structure"]
impl crate::Readable for R8_UEP7_T_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`r8_uep7_t_ctrl::W`](W) writer structure"]
impl crate::Writable for R8_UEP7_T_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets R8_UEP7_T_CTRL to value 0"]
impl crate::Resettable for R8_UEP7_T_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
