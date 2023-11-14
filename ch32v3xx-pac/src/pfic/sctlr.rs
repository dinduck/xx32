#[doc = "Register `SCTLR` reader"]
pub type R = crate::R<SCTLR_SPEC>;
#[doc = "Register `SCTLR` writer"]
pub type W = crate::W<SCTLR_SPEC>;
#[doc = "Field `SLEEPONEXIT` reader - SLEEPONEXIT"]
pub type SLEEPONEXIT_R = crate::BitReader;
#[doc = "Field `SLEEPONEXIT` writer - SLEEPONEXIT"]
pub type SLEEPONEXIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLEEPDEEP` reader - SLEEPDEEP"]
pub type SLEEPDEEP_R = crate::BitReader;
#[doc = "Field `SLEEPDEEP` writer - SLEEPDEEP"]
pub type SLEEPDEEP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WFITOWFE` reader - WFITOWFE"]
pub type WFITOWFE_R = crate::BitReader;
#[doc = "Field `WFITOWFE` writer - WFITOWFE"]
pub type WFITOWFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SEVONPEND` reader - SEVONPEND"]
pub type SEVONPEND_R = crate::BitReader;
#[doc = "Field `SEVONPEND` writer - SEVONPEND"]
pub type SEVONPEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SETEVENT` reader - SETEVENT"]
pub type SETEVENT_R = crate::BitReader;
#[doc = "Field `SETEVENT` writer - SETEVENT"]
pub type SETEVENT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSRESET` reader - SYSRESET"]
pub type SYSRESET_R = crate::BitReader;
#[doc = "Field `SYSRESET` writer - SYSRESET"]
pub type SYSRESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - SLEEPONEXIT"]
    #[inline(always)]
    pub fn sleeponexit(&self) -> SLEEPONEXIT_R {
        SLEEPONEXIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SLEEPDEEP"]
    #[inline(always)]
    pub fn sleepdeep(&self) -> SLEEPDEEP_R {
        SLEEPDEEP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WFITOWFE"]
    #[inline(always)]
    pub fn wfitowfe(&self) -> WFITOWFE_R {
        WFITOWFE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SEVONPEND"]
    #[inline(always)]
    pub fn sevonpend(&self) -> SEVONPEND_R {
        SEVONPEND_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SETEVENT"]
    #[inline(always)]
    pub fn setevent(&self) -> SETEVENT_R {
        SETEVENT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 31 - SYSRESET"]
    #[inline(always)]
    pub fn sysreset(&self) -> SYSRESET_R {
        SYSRESET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SLEEPONEXIT"]
    #[inline(always)]
    #[must_use]
    pub fn sleeponexit(&mut self) -> SLEEPONEXIT_W<SCTLR_SPEC, 1> {
        SLEEPONEXIT_W::new(self)
    }
    #[doc = "Bit 2 - SLEEPDEEP"]
    #[inline(always)]
    #[must_use]
    pub fn sleepdeep(&mut self) -> SLEEPDEEP_W<SCTLR_SPEC, 2> {
        SLEEPDEEP_W::new(self)
    }
    #[doc = "Bit 3 - WFITOWFE"]
    #[inline(always)]
    #[must_use]
    pub fn wfitowfe(&mut self) -> WFITOWFE_W<SCTLR_SPEC, 3> {
        WFITOWFE_W::new(self)
    }
    #[doc = "Bit 4 - SEVONPEND"]
    #[inline(always)]
    #[must_use]
    pub fn sevonpend(&mut self) -> SEVONPEND_W<SCTLR_SPEC, 4> {
        SEVONPEND_W::new(self)
    }
    #[doc = "Bit 5 - SETEVENT"]
    #[inline(always)]
    #[must_use]
    pub fn setevent(&mut self) -> SETEVENT_W<SCTLR_SPEC, 5> {
        SETEVENT_W::new(self)
    }
    #[doc = "Bit 31 - SYSRESET"]
    #[inline(always)]
    #[must_use]
    pub fn sysreset(&mut self) -> SYSRESET_W<SCTLR_SPEC, 31> {
        SYSRESET_W::new(self)
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
#[doc = "System Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCTLR_SPEC;
impl crate::RegisterSpec for SCTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sctlr::R`](R) reader structure"]
impl crate::Readable for SCTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sctlr::W`](W) writer structure"]
impl crate::Writable for SCTLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCTLR to value 0"]
impl crate::Resettable for SCTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
