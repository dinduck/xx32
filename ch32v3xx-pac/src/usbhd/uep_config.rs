#[doc = "Register `UEP_CONFIG` reader"]
pub type R = crate::R<UEP_CONFIG_SPEC>;
#[doc = "Register `UEP_CONFIG` writer"]
pub type W = crate::W<UEP_CONFIG_SPEC>;
#[doc = "Field `bUEP_T_EN_bUH_TX_EN` reader - endpoint TX enable/bUH_TX_EN"]
pub type B_UEP_T_EN_B_UH_TX_EN_R = crate::FieldReader<u16>;
#[doc = "Field `bUEP_T_EN_bUH_TX_EN` writer - endpoint TX enable/bUH_TX_EN"]
pub type B_UEP_T_EN_B_UH_TX_EN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 15, O, u16>;
#[doc = "Field `bUEP_R_EN__UH_EP_MOD` reader - endpoint RX enable/bUH_TX_EN"]
pub type B_UEP_R_EN__UH_EP_MOD_R = crate::FieldReader<u16>;
#[doc = "Field `bUEP_R_EN__UH_EP_MOD` writer - endpoint RX enable/bUH_TX_EN"]
pub type B_UEP_R_EN__UH_EP_MOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 15, O, u16>;
impl R {
    #[doc = "Bits 1:15 - endpoint TX enable/bUH_TX_EN"]
    #[inline(always)]
    pub fn b_uep_t_en_b_uh_tx_en(&self) -> B_UEP_T_EN_B_UH_TX_EN_R {
        B_UEP_T_EN_B_UH_TX_EN_R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
    #[doc = "Bits 17:31 - endpoint RX enable/bUH_TX_EN"]
    #[inline(always)]
    pub fn b_uep_r_en__uh_ep_mod(&self) -> B_UEP_R_EN__UH_EP_MOD_R {
        B_UEP_R_EN__UH_EP_MOD_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:15 - endpoint TX enable/bUH_TX_EN"]
    #[inline(always)]
    #[must_use]
    pub fn b_uep_t_en_b_uh_tx_en(&mut self) -> B_UEP_T_EN_B_UH_TX_EN_W<UEP_CONFIG_SPEC, 1> {
        B_UEP_T_EN_B_UH_TX_EN_W::new(self)
    }
    #[doc = "Bits 17:31 - endpoint RX enable/bUH_TX_EN"]
    #[inline(always)]
    #[must_use]
    pub fn b_uep_r_en__uh_ep_mod(&mut self) -> B_UEP_R_EN__UH_EP_MOD_W<UEP_CONFIG_SPEC, 17> {
        B_UEP_R_EN__UH_EP_MOD_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USB endpoint configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UEP_CONFIG_SPEC;
impl crate::RegisterSpec for UEP_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uep_config::R`](R) reader structure"]
impl crate::Readable for UEP_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uep_config::W`](W) writer structure"]
impl crate::Writable for UEP_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEP_CONFIG to value 0"]
impl crate::Resettable for UEP_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
