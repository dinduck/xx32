#[doc = "Register `UEP7_T_CTRL` reader"]
pub type R = crate::R<UEP7_T_CTRL_SPEC>;
#[doc = "Register `UEP7_T_CTRL` writer"]
pub type W = crate::W<UEP7_T_CTRL_SPEC>;
#[doc = "Field `MASK_UEP_T_RES` reader - endpoint 7 control of the send response to IN transactions"]
pub type MASK_UEP_T_RES_R = crate::FieldReader;
#[doc = "Field `MASK_UEP_T_RES` writer - endpoint 7 control of the send response to IN transactions"]
pub type MASK_UEP_T_RES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MASK_UEP_T_TOG` reader - endpoint 7 synchronous trigger bit for the sender to prepare"]
pub type MASK_UEP_T_TOG_R = crate::FieldReader;
#[doc = "Field `MASK_UEP_T_TOG` writer - endpoint 7 synchronous trigger bit for the sender to prepare"]
pub type MASK_UEP_T_TOG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `bUEP_T_TOG_AUTO` reader - endpoint 7 synchronous trigger bit automatic filp enables the control bit"]
pub type B_UEP_T_TOG_AUTO_R = crate::BitReader;
#[doc = "Field `bUEP_T_TOG_AUTO` writer - endpoint 7 synchronous trigger bit automatic filp enables the control bit"]
pub type B_UEP_T_TOG_AUTO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - endpoint 7 control of the send response to IN transactions"]
    #[inline(always)]
    pub fn mask_uep_t_res(&self) -> MASK_UEP_T_RES_R {
        MASK_UEP_T_RES_R::new(self.bits & 3)
    }
    #[doc = "Bits 3:4 - endpoint 7 synchronous trigger bit for the sender to prepare"]
    #[inline(always)]
    pub fn mask_uep_t_tog(&self) -> MASK_UEP_T_TOG_R {
        MASK_UEP_T_TOG_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 5 - endpoint 7 synchronous trigger bit automatic filp enables the control bit"]
    #[inline(always)]
    pub fn b_uep_t_tog_auto(&self) -> B_UEP_T_TOG_AUTO_R {
        B_UEP_T_TOG_AUTO_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - endpoint 7 control of the send response to IN transactions"]
    #[inline(always)]
    #[must_use]
    pub fn mask_uep_t_res(&mut self) -> MASK_UEP_T_RES_W<UEP7_T_CTRL_SPEC, 0> {
        MASK_UEP_T_RES_W::new(self)
    }
    #[doc = "Bits 3:4 - endpoint 7 synchronous trigger bit for the sender to prepare"]
    #[inline(always)]
    #[must_use]
    pub fn mask_uep_t_tog(&mut self) -> MASK_UEP_T_TOG_W<UEP7_T_CTRL_SPEC, 3> {
        MASK_UEP_T_TOG_W::new(self)
    }
    #[doc = "Bit 5 - endpoint 7 synchronous trigger bit automatic filp enables the control bit"]
    #[inline(always)]
    #[must_use]
    pub fn b_uep_t_tog_auto(&mut self) -> B_UEP_T_TOG_AUTO_W<UEP7_T_CTRL_SPEC, 5> {
        B_UEP_T_TOG_AUTO_W::new(self)
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
#[doc = "endpoint 7 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep7_t_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep7_t_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UEP7_T_CTRL_SPEC;
impl crate::RegisterSpec for UEP7_T_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uep7_t_ctrl::R`](R) reader structure"]
impl crate::Readable for UEP7_T_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uep7_t_ctrl::W`](W) writer structure"]
impl crate::Writable for UEP7_T_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEP7_T_CTRL to value 0"]
impl crate::Resettable for UEP7_T_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
