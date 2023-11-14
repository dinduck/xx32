#[doc = "Register `TPCTLR` reader"]
pub type R = crate::R<TPCTLR_SPEC>;
#[doc = "Register `TPCTLR` writer"]
pub type W = crate::W<TPCTLR_SPEC>;
#[doc = "Field `TPE` reader - Tamper pin enable"]
pub type TPE_R = crate::BitReader;
#[doc = "Field `TPE` writer - Tamper pin enable"]
pub type TPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TPAL` reader - Tamper pin active level"]
pub type TPAL_R = crate::BitReader;
#[doc = "Field `TPAL` writer - Tamper pin active level"]
pub type TPAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Tamper pin enable"]
    #[inline(always)]
    pub fn tpe(&self) -> TPE_R {
        TPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper pin active level"]
    #[inline(always)]
    pub fn tpal(&self) -> TPAL_R {
        TPAL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpe(&mut self) -> TPE_W<TPCTLR_SPEC, 0> {
        TPE_W::new(self)
    }
    #[doc = "Bit 1 - Tamper pin active level"]
    #[inline(always)]
    #[must_use]
    pub fn tpal(&mut self) -> TPAL_W<TPCTLR_SPEC, 1> {
        TPAL_W::new(self)
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
#[doc = "Backup control register (BKP_TPCTLR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TPCTLR_SPEC;
impl crate::RegisterSpec for TPCTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpctlr::R`](R) reader structure"]
impl crate::Readable for TPCTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tpctlr::W`](W) writer structure"]
impl crate::Writable for TPCTLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TPCTLR to value 0"]
impl crate::Resettable for TPCTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
