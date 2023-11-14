#[doc = "Register `CTLR1` reader"]
pub type R = crate::R<CTLR1_SPEC>;
#[doc = "Register `CTLR1` writer"]
pub type W = crate::W<CTLR1_SPEC>;
#[doc = "Field `CEN` reader - Counter enable"]
pub type CEN_R = crate::BitReader;
#[doc = "Field `CEN` writer - Counter enable"]
pub type CEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UDIS` reader - Update disable"]
pub type UDIS_R = crate::BitReader;
#[doc = "Field `UDIS` writer - Update disable"]
pub type UDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `URS` reader - Update request source"]
pub type URS_R = crate::BitReader;
#[doc = "Field `URS` writer - Update request source"]
pub type URS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OPM` reader - One-pulse mode"]
pub type OPM_R = crate::BitReader;
#[doc = "Field `OPM` writer - One-pulse mode"]
pub type OPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ARPE` reader - Auto-reload preload enable"]
pub type ARPE_R = crate::BitReader;
#[doc = "Field `ARPE` writer - Auto-reload preload enable"]
pub type ARPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn udis(&self) -> UDIS_R {
        UDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline(always)]
    pub fn urs(&self) -> URS_R {
        URS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline(always)]
    pub fn opm(&self) -> OPM_R {
        OPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    pub fn arpe(&self) -> ARPE_R {
        ARPE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<CTLR1_SPEC, 0> {
        CEN_W::new(self)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    #[must_use]
    pub fn udis(&mut self) -> UDIS_W<CTLR1_SPEC, 1> {
        UDIS_W::new(self)
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline(always)]
    #[must_use]
    pub fn urs(&mut self) -> URS_W<CTLR1_SPEC, 2> {
        URS_W::new(self)
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline(always)]
    #[must_use]
    pub fn opm(&mut self) -> OPM_W<CTLR1_SPEC, 3> {
        OPM_W::new(self)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn arpe(&mut self) -> ARPE_W<CTLR1_SPEC, 7> {
        ARPE_W::new(self)
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
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTLR1_SPEC;
impl crate::RegisterSpec for CTLR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlr1::R`](R) reader structure"]
impl crate::Readable for CTLR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctlr1::W`](W) writer structure"]
impl crate::Writable for CTLR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTLR1 to value 0"]
impl crate::Resettable for CTLR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
