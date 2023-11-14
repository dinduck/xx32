#[doc = "Register `UEP3_T_LEN___UH_TX_LEN_H` reader"]
pub type R = crate::R<UEP3_T_LEN___UH_TX_LEN_H_SPEC>;
#[doc = "Register `UEP3_T_LEN___UH_TX_LEN_H` writer"]
pub type W = crate::W<UEP3_T_LEN___UH_TX_LEN_H_SPEC>;
#[doc = "Field `UEP3_T_LEN___UH_TX_LEN_H` reader - endpoint 3 send the length"]
pub type UEP3_T_LEN___UH_TX_LEN_H_R = crate::FieldReader<u16>;
#[doc = "Field `UEP3_T_LEN___UH_TX_LEN_H` writer - endpoint 3 send the length"]
pub type UEP3_T_LEN___UH_TX_LEN_H_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bits 0:10 - endpoint 3 send the length"]
    #[inline(always)]
    pub fn uep3_t_len___uh_tx_len_h(&self) -> UEP3_T_LEN___UH_TX_LEN_H_R {
        UEP3_T_LEN___UH_TX_LEN_H_R::new(self.bits & 0x07ff)
    }
}
impl W {
    #[doc = "Bits 0:10 - endpoint 3 send the length"]
    #[inline(always)]
    #[must_use]
    pub fn uep3_t_len___uh_tx_len_h(
        &mut self,
    ) -> UEP3_T_LEN___UH_TX_LEN_H_W<UEP3_T_LEN___UH_TX_LEN_H_SPEC, 0> {
        UEP3_T_LEN___UH_TX_LEN_H_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "endpoint 3 send the length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep3_t_len___uh_tx_len_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep3_t_len___uh_tx_len_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UEP3_T_LEN___UH_TX_LEN_H_SPEC;
impl crate::RegisterSpec for UEP3_T_LEN___UH_TX_LEN_H_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uep3_t_len___uh_tx_len_h::R`](R) reader structure"]
impl crate::Readable for UEP3_T_LEN___UH_TX_LEN_H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uep3_t_len___uh_tx_len_h::W`](W) writer structure"]
impl crate::Writable for UEP3_T_LEN___UH_TX_LEN_H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEP3_T_LEN___UH_TX_LEN_H to value 0"]
impl crate::Resettable for UEP3_T_LEN___UH_TX_LEN_H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
