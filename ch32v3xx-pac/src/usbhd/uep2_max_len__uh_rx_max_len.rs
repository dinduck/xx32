#[doc = "Register `UEP2_MAX_LEN__UH_RX_MAX_LEN` reader"]
pub type R = crate::R<UEP2_MAX_LEN__UH_RX_MAX_LEN_SPEC>;
#[doc = "Register `UEP2_MAX_LEN__UH_RX_MAX_LEN` writer"]
pub type W = crate::W<UEP2_MAX_LEN__UH_RX_MAX_LEN_SPEC>;
#[doc = "Field `UEP2_MAX_LEN__UH_RX_MAX_LEN` reader - endpoint 2 max acceptable length"]
pub type UEP2_MAX_LEN__UH_RX_MAX_LEN_R = crate::FieldReader<u16>;
#[doc = "Field `UEP2_MAX_LEN__UH_RX_MAX_LEN` writer - endpoint 2 max acceptable length"]
pub type UEP2_MAX_LEN__UH_RX_MAX_LEN_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bits 0:10 - endpoint 2 max acceptable length"]
    #[inline(always)]
    pub fn uep2_max_len__uh_rx_max_len(&self) -> UEP2_MAX_LEN__UH_RX_MAX_LEN_R {
        UEP2_MAX_LEN__UH_RX_MAX_LEN_R::new(self.bits & 0x07ff)
    }
}
impl W {
    #[doc = "Bits 0:10 - endpoint 2 max acceptable length"]
    #[inline(always)]
    #[must_use]
    pub fn uep2_max_len__uh_rx_max_len(
        &mut self,
    ) -> UEP2_MAX_LEN__UH_RX_MAX_LEN_W<UEP2_MAX_LEN__UH_RX_MAX_LEN_SPEC, 0> {
        UEP2_MAX_LEN__UH_RX_MAX_LEN_W::new(self)
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
#[doc = "endpoint 2 max acceptable length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep2_max_len__uh_rx_max_len::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep2_max_len__uh_rx_max_len::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UEP2_MAX_LEN__UH_RX_MAX_LEN_SPEC;
impl crate::RegisterSpec for UEP2_MAX_LEN__UH_RX_MAX_LEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uep2_max_len__uh_rx_max_len::R`](R) reader structure"]
impl crate::Readable for UEP2_MAX_LEN__UH_RX_MAX_LEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uep2_max_len__uh_rx_max_len::W`](W) writer structure"]
impl crate::Writable for UEP2_MAX_LEN__UH_RX_MAX_LEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEP2_MAX_LEN__UH_RX_MAX_LEN to value 0"]
impl crate::Resettable for UEP2_MAX_LEN__UH_RX_MAX_LEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
