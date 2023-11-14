#[doc = "Register `UEP5_T_LEN` reader"]
pub type R = crate::R<UEP5_T_LEN_SPEC>;
#[doc = "Register `UEP5_T_LEN` writer"]
pub type W = crate::W<UEP5_T_LEN_SPEC>;
#[doc = "Field `UEP5_T_LEN` reader - endpoint 5 send the length"]
pub type UEP5_T_LEN_R = crate::FieldReader<u16>;
#[doc = "Field `UEP5_T_LEN` writer - endpoint 5 send the length"]
pub type UEP5_T_LEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bits 0:10 - endpoint 5 send the length"]
    #[inline(always)]
    pub fn uep5_t_len(&self) -> UEP5_T_LEN_R {
        UEP5_T_LEN_R::new(self.bits & 0x07ff)
    }
}
impl W {
    #[doc = "Bits 0:10 - endpoint 5 send the length"]
    #[inline(always)]
    #[must_use]
    pub fn uep5_t_len(&mut self) -> UEP5_T_LEN_W<UEP5_T_LEN_SPEC, 0> {
        UEP5_T_LEN_W::new(self)
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
#[doc = "endpoint 5 send the length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uep5_t_len::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uep5_t_len::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UEP5_T_LEN_SPEC;
impl crate::RegisterSpec for UEP5_T_LEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uep5_t_len::R`](R) reader structure"]
impl crate::Readable for UEP5_T_LEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uep5_t_len::W`](W) writer structure"]
impl crate::Writable for UEP5_T_LEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEP5_T_LEN to value 0"]
impl crate::Resettable for UEP5_T_LEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
