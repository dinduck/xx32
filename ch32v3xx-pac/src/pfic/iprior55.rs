#[doc = "Register `IPRIOR55` reader"]
pub type R = crate::R<IPRIOR55_SPEC>;
#[doc = "Register `IPRIOR55` writer"]
pub type W = crate::W<IPRIOR55_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<IPRIOR55_SPEC> {
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
#[doc = "Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior55::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior55::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPRIOR55_SPEC;
impl crate::RegisterSpec for IPRIOR55_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`iprior55::R`](R) reader structure"]
impl crate::Readable for IPRIOR55_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iprior55::W`](W) writer structure"]
impl crate::Writable for IPRIOR55_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPRIOR55 to value 0"]
impl crate::Resettable for IPRIOR55_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
