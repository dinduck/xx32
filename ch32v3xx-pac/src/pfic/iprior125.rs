#[doc = "Register `IPRIOR125` reader"]
pub type R = crate::R<IPRIOR125_SPEC>;
#[doc = "Register `IPRIOR125` writer"]
pub type W = crate::W<IPRIOR125_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<IPRIOR125_SPEC> {
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
#[doc = "Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior125::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior125::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPRIOR125_SPEC;
impl crate::RegisterSpec for IPRIOR125_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`iprior125::R`](R) reader structure"]
impl crate::Readable for IPRIOR125_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iprior125::W`](W) writer structure"]
impl crate::Writable for IPRIOR125_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPRIOR125 to value 0"]
impl crate::Resettable for IPRIOR125_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
