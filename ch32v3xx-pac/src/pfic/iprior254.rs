#[doc = "Register `IPRIOR254` reader"]
pub type R = crate::R<IPRIOR254_SPEC>;
#[doc = "Register `IPRIOR254` writer"]
pub type W = crate::W<IPRIOR254_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<IPRIOR254_SPEC> {
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
#[doc = "Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iprior254::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprior254::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPRIOR254_SPEC;
impl crate::RegisterSpec for IPRIOR254_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`iprior254::R`](R) reader structure"]
impl crate::Readable for IPRIOR254_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iprior254::W`](W) writer structure"]
impl crate::Writable for IPRIOR254_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPRIOR254 to value 0"]
impl crate::Resettable for IPRIOR254_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
