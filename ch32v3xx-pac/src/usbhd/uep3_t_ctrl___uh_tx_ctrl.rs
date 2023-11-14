#[doc = "Register `UEP3_T_CTRL___UH_TX_CTRL` reader"]
pub type R = crate::R<UEP3_T_CTRL___UH_TX_CTRL_SPEC>;
#[doc = "Register `UEP3_T_CTRL___UH_TX_CTRL` writer"]
pub type W = crate::W<UEP3_T_CTRL___UH_TX_CTRL_SPEC>;
#[doc = "Field `MASK_UEP_T_RES_____MASK_UH_T_RES` reader - endpoint 3 control of the send response to IN transactions"]
pub type MASK_UEP_T_RES_____MASK_UH_T_RES_R = crate::FieldReader;
#[doc = "Field `MASK_UEP_T_RES_____MASK_UH_T_RES` writer - endpoint 3 control of the send response to IN transactions"]
pub type MASK_UEP_T_RES_____MASK_UH_T_RES_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `bUH_T_RES_NO` reader - bUH_T_RES_NO"]
pub type B_UH_T_RES_NO_R = crate::BitReader;
#[doc = "Field `bUH_T_RES_NO` writer - bUH_T_RES_NO"]
pub type B_UH_T_RES_NO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MASK_UEP_T_TOG____MASK_UH_T_TOG` reader - endpoint 3 synchronous trigger bit for the sender to prepare"]
pub type MASK_UEP_T_TOG____MASK_UH_T_TOG_R = crate::FieldReader;
#[doc = "Field `MASK_UEP_T_TOG____MASK_UH_T_TOG` writer - endpoint 3 synchronous trigger bit for the sender to prepare"]
pub type MASK_UEP_T_TOG____MASK_UH_T_TOG_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `bUEP_T_TOG_AUTO____bUH_T_AUTO_TOG` reader - endpoint 3 synchronous trigger bit automatic filp enables the control bit"]
pub type B_UEP_T_TOG_AUTO____B_UH_T_AUTO_TOG_R = crate::BitReader;
#[doc = "Field `bUEP_T_TOG_AUTO____bUH_T_AUTO_TOG` writer - endpoint 3 synchronous trigger bit automatic filp enables the control bit"]
pub type B_UEP_T_TOG_AUTO____B_UH_T_AUTO_TOG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `bUH_T_DATA_NO` reader - bUH_T_DATA_NO"]
pub type B_UH_T_DATA_NO_R = crate::BitReader;
#[doc = "Field `bUH_T_DATA_NO` writer - bUH_T_DATA_NO"]
pub type B_UH_T_DATA_NO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - endpoint 3 control of the send response to IN transactions"]
    #[inline(always)]
    pub fn mask_uep_t_res_____mask_uh_t_res(&self) -> MASK_UEP_T_RES_____MASK_UH_T_RES_R {
        MASK_UEP_T_RES_____MASK_UH_T_RES_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - bUH_T_RES_NO"]
    #[inline(always)]
    pub fn b_uh_t_res_no(&self) -> B_UH_T_RES_NO_R {
        B_UH_T_RES_NO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - endpoint 3 synchronous trigger bit for the sender to prepare"]
    #[inline(always)]
    pub fn mask_uep_t_tog____mask_uh_t_tog(&self) -> MASK_UEP_T_TOG____MASK_UH_T_TOG_R {
        MASK_UEP_T_TOG____MASK_UH_T_TOG_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 5 - endpoint 3 synchronous trigger bit automatic filp enables the control bit"]
    #[inline(always)]
    pub fn b_uep_t_tog_auto____b_uh_t_auto_tog(&self) -> B_UEP_T_TOG_AUTO____B_UH_T_AUTO_TOG_R {
        B_UEP_T_TOG_AUTO____B_UH_T_AUTO_TOG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - bUH_T_DATA_NO"]
    #[inline(always)]
    pub fn b_uh_t_data_no(&self) -> B_UH_T_DATA_NO_R {
        B_UH_T_DATA_NO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - endpoint 3 control of the send response to IN transactions"]
    #[inline(always)]
    #[must_use]
    pub fn mask_uep_t_res_____mask_uh_t_res(
        &mut self,
    ) -> MASK_UEP_T_RES_____MASK_UH_T_RES_W<UEP3_T_CTRL___UH_TX_CTRL_SPEC, 0> {
        MASK_UEP_T_RES_____MASK_UH_T_RES_W::new(self)
    }
    #[doc = "Bit 2 - bUH_T_RES_NO"]
    #[inline(always)]
    #[must_use]
    pub fn b_uh_t_res_no(&mut self) -> B_UH_T_RES_NO_W<UEP3_T_CTRL___UH_TX_CTRL_SPEC, 2> {
        B_UH_T_RES_NO_W::new(self)
    }
    #[doc = "Bits 3:4 - endpoint 3 synchronous trigger bit for the sender to prepare"]
    #[inline(always)]
    #[must_use]
    pub fn mask_uep_t_tog____mask_uh_t_tog(
        &mut self,
    ) -> MASK_UEP_T_TOG____MASK_UH_T_TOG_W<UEP3_T_CTRL___UH_TX_CTRL_SPEC, 3> {
        MASK_UEP_T_TOG____MASK_UH_T_TOG_W::new(self)
    }
    #[doc = "Bit 5 - endpoint 3 synchronous trigger bit automatic filp enables the control bit"]
    #[inline(always)]
    #[must_use]
    pub fn b_uep_t_tog_auto____b_uh_t_auto_tog(
        &mut self,
    ) -> B_UEP_T_TOG_AUTO____B_UH_T_AUTO_TOG_W<UEP3_T_CTRL___UH_TX_CTRL_SPEC, 5> {
        B_UEP_T_TOG_AUTO____B_UH_T_AUTO_TOG_W::new(self)
    }
    #[doc = "Bit 6 - bUH_T_DATA_NO"]
    #[inline(always)]
    #[must_use]
    pub fn b_uh_t_data_no(&mut self) -> B_UH_T_DATA_NO_W<UEP3_T_CTRL___UH_TX_CTRL_SPEC, 6> {
        B_UH_T_DATA_NO_W::new(self)
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
#[doc = "endpoint 3 send control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep3_t_ctrl___uh_tx_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep3_t_ctrl___uh_tx_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UEP3_T_CTRL___UH_TX_CTRL_SPEC;
impl crate::RegisterSpec for UEP3_T_CTRL___UH_TX_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uep3_t_ctrl___uh_tx_ctrl::R`](R) reader structure"]
impl crate::Readable for UEP3_T_CTRL___UH_TX_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uep3_t_ctrl___uh_tx_ctrl::W`](W) writer structure"]
impl crate::Writable for UEP3_T_CTRL___UH_TX_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEP3_T_CTRL___UH_TX_CTRL to value 0"]
impl crate::Resettable for UEP3_T_CTRL___UH_TX_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
