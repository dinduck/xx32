#[doc = "Register `UEP2_R_CTRL__UH_RX_CTRL` reader"]
pub type R = crate::R<UEP2_R_CTRL__UH_RX_CTRL_SPEC>;
#[doc = "Register `UEP2_R_CTRL__UH_RX_CTRL` writer"]
pub type W = crate::W<UEP2_R_CTRL__UH_RX_CTRL_SPEC>;
#[doc = "Field `MASK_UEP_R_RES__MASK_UH_R_RES` reader - endpoint 2 control of the accept response to OUT transactions"]
pub type MASK_UEP_R_RES__MASK_UH_R_RES_R = crate::FieldReader;
#[doc = "Field `MASK_UEP_R_RES__MASK_UH_R_RES` writer - endpoint 2 control of the accept response to OUT transactions"]
pub type MASK_UEP_R_RES__MASK_UH_R_RES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `bUH_R_RES_NO` reader - bUH_R_RES_NO"]
pub type B_UH_R_RES_NO_R = crate::BitReader;
#[doc = "Field `bUH_R_RES_NO` writer - bUH_R_RES_NO"]
pub type B_UH_R_RES_NO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MASK_UEP_R_TOG__MASK_UH_R_TOG` reader - endpoint 2 synchronous trigger bit for the accept to prepare"]
pub type MASK_UEP_R_TOG__MASK_UH_R_TOG_R = crate::FieldReader;
#[doc = "Field `MASK_UEP_R_TOG__MASK_UH_R_TOG` writer - endpoint 2 synchronous trigger bit for the accept to prepare"]
pub type MASK_UEP_R_TOG__MASK_UH_R_TOG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `bUEP_R_TOG_AUTO__bUH_R_AUTO_TOG` reader - endpoint 2 synchronous trigger bit automatic filp enables the control bit"]
pub type B_UEP_R_TOG_AUTO__B_UH_R_AUTO_TOG_R = crate::BitReader;
#[doc = "Field `bUH_R_DATA_NO` reader - bUH_R_DATA_NO"]
pub type B_UH_R_DATA_NO_R = crate::BitReader;
#[doc = "Field `bUH_R_DATA_NO` writer - bUH_R_DATA_NO"]
pub type B_UH_R_DATA_NO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - endpoint 2 control of the accept response to OUT transactions"]
    #[inline(always)]
    pub fn mask_uep_r_res__mask_uh_r_res(&self) -> MASK_UEP_R_RES__MASK_UH_R_RES_R {
        MASK_UEP_R_RES__MASK_UH_R_RES_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - bUH_R_RES_NO"]
    #[inline(always)]
    pub fn b_uh_r_res_no(&self) -> B_UH_R_RES_NO_R {
        B_UH_R_RES_NO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - endpoint 2 synchronous trigger bit for the accept to prepare"]
    #[inline(always)]
    pub fn mask_uep_r_tog__mask_uh_r_tog(&self) -> MASK_UEP_R_TOG__MASK_UH_R_TOG_R {
        MASK_UEP_R_TOG__MASK_UH_R_TOG_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 5 - endpoint 2 synchronous trigger bit automatic filp enables the control bit"]
    #[inline(always)]
    pub fn b_uep_r_tog_auto__b_uh_r_auto_tog(&self) -> B_UEP_R_TOG_AUTO__B_UH_R_AUTO_TOG_R {
        B_UEP_R_TOG_AUTO__B_UH_R_AUTO_TOG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - bUH_R_DATA_NO"]
    #[inline(always)]
    pub fn b_uh_r_data_no(&self) -> B_UH_R_DATA_NO_R {
        B_UH_R_DATA_NO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - endpoint 2 control of the accept response to OUT transactions"]
    #[inline(always)]
    #[must_use]
    pub fn mask_uep_r_res__mask_uh_r_res(
        &mut self,
    ) -> MASK_UEP_R_RES__MASK_UH_R_RES_W<UEP2_R_CTRL__UH_RX_CTRL_SPEC, 0> {
        MASK_UEP_R_RES__MASK_UH_R_RES_W::new(self)
    }
    #[doc = "Bit 2 - bUH_R_RES_NO"]
    #[inline(always)]
    #[must_use]
    pub fn b_uh_r_res_no(&mut self) -> B_UH_R_RES_NO_W<UEP2_R_CTRL__UH_RX_CTRL_SPEC, 2> {
        B_UH_R_RES_NO_W::new(self)
    }
    #[doc = "Bits 3:4 - endpoint 2 synchronous trigger bit for the accept to prepare"]
    #[inline(always)]
    #[must_use]
    pub fn mask_uep_r_tog__mask_uh_r_tog(
        &mut self,
    ) -> MASK_UEP_R_TOG__MASK_UH_R_TOG_W<UEP2_R_CTRL__UH_RX_CTRL_SPEC, 3> {
        MASK_UEP_R_TOG__MASK_UH_R_TOG_W::new(self)
    }
    #[doc = "Bit 6 - bUH_R_DATA_NO"]
    #[inline(always)]
    #[must_use]
    pub fn b_uh_r_data_no(&mut self) -> B_UH_R_DATA_NO_W<UEP2_R_CTRL__UH_RX_CTRL_SPEC, 6> {
        B_UH_R_DATA_NO_W::new(self)
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
#[doc = "endpoint 2 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep2_r_ctrl__uh_rx_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep2_r_ctrl__uh_rx_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UEP2_R_CTRL__UH_RX_CTRL_SPEC;
impl crate::RegisterSpec for UEP2_R_CTRL__UH_RX_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uep2_r_ctrl__uh_rx_ctrl::R`](R) reader structure"]
impl crate::Readable for UEP2_R_CTRL__UH_RX_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uep2_r_ctrl__uh_rx_ctrl::W`](W) writer structure"]
impl crate::Writable for UEP2_R_CTRL__UH_RX_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEP2_R_CTRL__UH_RX_CTRL to value 0"]
impl crate::Resettable for UEP2_R_CTRL__UH_RX_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
