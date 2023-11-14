#[doc = "Register `UEP13_R_CTRL` reader"]
pub type R = crate::R<UEP13_R_CTRL_SPEC>;
#[doc = "Register `UEP13_R_CTRL` writer"]
pub type W = crate::W<UEP13_R_CTRL_SPEC>;
#[doc = "Field `MASK_UEP_R_RES` reader - endpoint 13 control of the accept response to OUT transactions"]
pub type MASK_UEP_R_RES_R = crate::FieldReader;
#[doc = "Field `MASK_UEP_R_RES` writer - endpoint 13 control of the accept response to OUT transactions"]
pub type MASK_UEP_R_RES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MASK_UEP_R_TOG` reader - endpoint 13 synchronous trigger bit for the accept to prepare"]
pub type MASK_UEP_R_TOG_R = crate::FieldReader;
#[doc = "Field `MASK_UEP_R_TOG` writer - endpoint 13 synchronous trigger bit for the accept to prepare"]
pub type MASK_UEP_R_TOG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `bUEP_R_TOG_AUTO` reader - endpoint 13 synchronous trigger bit automatic filp enables the control bit"]
pub type B_UEP_R_TOG_AUTO_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - endpoint 13 control of the accept response to OUT transactions"]
    #[inline(always)]
    pub fn mask_uep_r_res(&self) -> MASK_UEP_R_RES_R {
        MASK_UEP_R_RES_R::new(self.bits & 3)
    }
    #[doc = "Bits 3:4 - endpoint 13 synchronous trigger bit for the accept to prepare"]
    #[inline(always)]
    pub fn mask_uep_r_tog(&self) -> MASK_UEP_R_TOG_R {
        MASK_UEP_R_TOG_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 5 - endpoint 13 synchronous trigger bit automatic filp enables the control bit"]
    #[inline(always)]
    pub fn b_uep_r_tog_auto(&self) -> B_UEP_R_TOG_AUTO_R {
        B_UEP_R_TOG_AUTO_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - endpoint 13 control of the accept response to OUT transactions"]
    #[inline(always)]
    #[must_use]
    pub fn mask_uep_r_res(&mut self) -> MASK_UEP_R_RES_W<UEP13_R_CTRL_SPEC, 0> {
        MASK_UEP_R_RES_W::new(self)
    }
    #[doc = "Bits 3:4 - endpoint 13 synchronous trigger bit for the accept to prepare"]
    #[inline(always)]
    #[must_use]
    pub fn mask_uep_r_tog(&mut self) -> MASK_UEP_R_TOG_W<UEP13_R_CTRL_SPEC, 3> {
        MASK_UEP_R_TOG_W::new(self)
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
#[doc = "endpoint 13 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep13_r_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep13_r_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UEP13_R_CTRL_SPEC;
impl crate::RegisterSpec for UEP13_R_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uep13_r_ctrl::R`](R) reader structure"]
impl crate::Readable for UEP13_R_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uep13_r_ctrl::W`](W) writer structure"]
impl crate::Writable for UEP13_R_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEP13_R_CTRL to value 0"]
impl crate::Resettable for UEP13_R_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
