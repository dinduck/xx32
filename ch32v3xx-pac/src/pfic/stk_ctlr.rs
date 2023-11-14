#[doc = "Register `STK_CTLR` reader"]
pub type R = crate::R<STK_CTLR_SPEC>;
#[doc = "Register `STK_CTLR` writer"]
pub type W = crate::W<STK_CTLR_SPEC>;
#[doc = "Field `STE` reader - System counter enable"]
pub type STE_R = crate::BitReader;
#[doc = "Field `STE` writer - System counter enable"]
pub type STE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STIE` reader - System counter interrupt enable"]
pub type STIE_R = crate::BitReader;
#[doc = "Field `STIE` writer - System counter interrupt enable"]
pub type STIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STCLK` reader - System selects the clock source"]
pub type STCLK_R = crate::BitReader;
#[doc = "Field `STCLK` writer - System selects the clock source"]
pub type STCLK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STRE` reader - System reload register"]
pub type STRE_R = crate::BitReader;
#[doc = "Field `STRE` writer - System reload register"]
pub type STRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MODE` reader - System Mode"]
pub type MODE_R = crate::BitReader;
#[doc = "Field `MODE` writer - System Mode"]
pub type MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INIT` reader - System Initialization update"]
pub type INIT_R = crate::BitReader;
#[doc = "Field `INIT` writer - System Initialization update"]
pub type INIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWIE` reader - System software triggered interrupts enable"]
pub type SWIE_R = crate::BitReader;
#[doc = "Field `SWIE` writer - System software triggered interrupts enable"]
pub type SWIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - System counter enable"]
    #[inline(always)]
    pub fn ste(&self) -> STE_R {
        STE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - System counter interrupt enable"]
    #[inline(always)]
    pub fn stie(&self) -> STIE_R {
        STIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System selects the clock source"]
    #[inline(always)]
    pub fn stclk(&self) -> STCLK_R {
        STCLK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - System reload register"]
    #[inline(always)]
    pub fn stre(&self) -> STRE_R {
        STRE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - System Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - System Initialization update"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 31 - System software triggered interrupts enable"]
    #[inline(always)]
    pub fn swie(&self) -> SWIE_R {
        SWIE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn ste(&mut self) -> STE_W<STK_CTLR_SPEC, 0> {
        STE_W::new(self)
    }
    #[doc = "Bit 1 - System counter interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stie(&mut self) -> STIE_W<STK_CTLR_SPEC, 1> {
        STIE_W::new(self)
    }
    #[doc = "Bit 2 - System selects the clock source"]
    #[inline(always)]
    #[must_use]
    pub fn stclk(&mut self) -> STCLK_W<STK_CTLR_SPEC, 2> {
        STCLK_W::new(self)
    }
    #[doc = "Bit 3 - System reload register"]
    #[inline(always)]
    #[must_use]
    pub fn stre(&mut self) -> STRE_W<STK_CTLR_SPEC, 3> {
        STRE_W::new(self)
    }
    #[doc = "Bit 4 - System Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<STK_CTLR_SPEC, 4> {
        MODE_W::new(self)
    }
    #[doc = "Bit 5 - System Initialization update"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<STK_CTLR_SPEC, 5> {
        INIT_W::new(self)
    }
    #[doc = "Bit 31 - System software triggered interrupts enable"]
    #[inline(always)]
    #[must_use]
    pub fn swie(&mut self) -> SWIE_W<STK_CTLR_SPEC, 31> {
        SWIE_W::new(self)
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
#[doc = "System counter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stk_ctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stk_ctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STK_CTLR_SPEC;
impl crate::RegisterSpec for STK_CTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stk_ctlr::R`](R) reader structure"]
impl crate::Readable for STK_CTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stk_ctlr::W`](W) writer structure"]
impl crate::Writable for STK_CTLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STK_CTLR to value 0"]
impl crate::Resettable for STK_CTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
