#[doc = "Register `CTLR2` reader"]
pub type R = crate::R<CTLR2_SPEC>;
#[doc = "Register `CTLR2` writer"]
pub type W = crate::W<CTLR2_SPEC>;
#[doc = "Field `FREQ` reader - Peripheral clock frequency"]
pub type FREQ_R = crate::FieldReader;
#[doc = "Field `FREQ` writer - Peripheral clock frequency"]
pub type FREQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `ITERREN` reader - Error interrupt enable"]
pub type ITERREN_R = crate::BitReader;
#[doc = "Field `ITERREN` writer - Error interrupt enable"]
pub type ITERREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ITEVTEN` reader - Event interrupt enable"]
pub type ITEVTEN_R = crate::BitReader;
#[doc = "Field `ITEVTEN` writer - Event interrupt enable"]
pub type ITEVTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ITBUFEN` reader - Buffer interrupt enable"]
pub type ITBUFEN_R = crate::BitReader;
#[doc = "Field `ITBUFEN` writer - Buffer interrupt enable"]
pub type ITBUFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMAEN` reader - DMA requests enable"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA requests enable"]
pub type DMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LAST` reader - DMA last transfer"]
pub type LAST_R = crate::BitReader;
#[doc = "Field `LAST` writer - DMA last transfer"]
pub type LAST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:5 - Peripheral clock frequency"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    pub fn iterren(&self) -> ITERREN_R {
        ITERREN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    pub fn itevten(&self) -> ITEVTEN_R {
        ITEVTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer interrupt enable"]
    #[inline(always)]
    pub fn itbufen(&self) -> ITBUFEN_R {
        ITBUFEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA requests enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA last transfer"]
    #[inline(always)]
    pub fn last(&self) -> LAST_R {
        LAST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Peripheral clock frequency"]
    #[inline(always)]
    #[must_use]
    pub fn freq(&mut self) -> FREQ_W<CTLR2_SPEC, 0> {
        FREQ_W::new(self)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn iterren(&mut self) -> ITERREN_W<CTLR2_SPEC, 8> {
        ITERREN_W::new(self)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn itevten(&mut self) -> ITEVTEN_W<CTLR2_SPEC, 9> {
        ITEVTEN_W::new(self)
    }
    #[doc = "Bit 10 - Buffer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn itbufen(&mut self) -> ITBUFEN_W<CTLR2_SPEC, 10> {
        ITBUFEN_W::new(self)
    }
    #[doc = "Bit 11 - DMA requests enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<CTLR2_SPEC, 11> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 12 - DMA last transfer"]
    #[inline(always)]
    #[must_use]
    pub fn last(&mut self) -> LAST_W<CTLR2_SPEC, 12> {
        LAST_W::new(self)
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
#[doc = "Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTLR2_SPEC;
impl crate::RegisterSpec for CTLR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlr2::R`](R) reader structure"]
impl crate::Readable for CTLR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctlr2::W`](W) writer structure"]
impl crate::Writable for CTLR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTLR2 to value 0"]
impl crate::Resettable for CTLR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
