#[doc = "Register `R8_UEP5_T_LEN` reader"]
pub type R = crate::R<R8_UEP5_T_LEN_SPEC>;
#[doc = "Register `R8_UEP5_T_LEN` writer"]
pub type W = crate::W<R8_UEP5_T_LEN_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<R8_UEP5_T_LEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
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
#[doc = "endpoint 5 transmittal length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep5_t_len::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep5_t_len::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8_UEP5_T_LEN_SPEC;
impl crate::RegisterSpec for R8_UEP5_T_LEN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uep5_t_len::R`](R) reader structure"]
impl crate::Readable for R8_UEP5_T_LEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`r8_uep5_t_len::W`](W) writer structure"]
impl crate::Writable for R8_UEP5_T_LEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets R8_UEP5_T_LEN to value 0"]
impl crate::Resettable for R8_UEP5_T_LEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
