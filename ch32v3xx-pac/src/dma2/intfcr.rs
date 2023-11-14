#[doc = "Register `INTFCR` writer"]
pub type W = crate::W<INTFCR_SPEC>;
#[doc = "Field `CGIF1` writer - Channel 1 Global interrupt clear"]
pub type CGIF1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTCIF1` writer - Channel 1 Transfer Complete clear"]
pub type CTCIF1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHTIF1` writer - Channel 1 Half Transfer clear"]
pub type CHTIF1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTEIF1` writer - Channel 1 Transfer Error clear"]
pub type CTEIF1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CGIF2` writer - Channel 2 Global interrupt clear"]
pub type CGIF2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTCIF2` writer - Channel 2 Transfer Complete clear"]
pub type CTCIF2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHTIF2` writer - Channel 2 Half Transfer clear"]
pub type CHTIF2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTEIF2` writer - Channel 2 Transfer Error clear"]
pub type CTEIF2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CGIF3` writer - Channel 3 Global interrupt clear"]
pub type CGIF3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTCIF3` writer - Channel 3 Transfer Complete clear"]
pub type CTCIF3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHTIF3` writer - Channel 3 Half Transfer clear"]
pub type CHTIF3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTEIF3` writer - Channel 3 Transfer Error clear"]
pub type CTEIF3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CGIF4` writer - Channel 4 Global interrupt clear"]
pub type CGIF4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTCIF4` writer - Channel 4 Transfer Complete clear"]
pub type CTCIF4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHTIF4` writer - Channel 4 Half Transfer clear"]
pub type CHTIF4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTEIF4` writer - Channel 4 Transfer Error clear"]
pub type CTEIF4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CGIF5` writer - Channel 5 Global interrupt clear"]
pub type CGIF5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTCIF5` writer - Channel 5 Transfer Complete clear"]
pub type CTCIF5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHTIF5` writer - Channel 5 Half Transfer clear"]
pub type CHTIF5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTEIF5` writer - Channel 5 Transfer Error clear"]
pub type CTEIF5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CGIF6` writer - Channel 6 Global interrupt clear"]
pub type CGIF6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTCIF6` writer - Channel 6 Transfer Complete clear"]
pub type CTCIF6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHTIF6` writer - Channel 6 Half Transfer clear"]
pub type CHTIF6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTEIF6` writer - Channel 6 Transfer Error clear"]
pub type CTEIF6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CGIF7` writer - Channel 7 Global interrupt clear"]
pub type CGIF7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTCIF7` writer - Channel 7 Transfer Complete clear"]
pub type CTCIF7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHTIF7` writer - Channel 7 Half Transfer clear"]
pub type CHTIF7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTEIF7` writer - Channel 7 Transfer Error clear"]
pub type CTEIF7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Channel 1 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif1(&mut self) -> CGIF1_W<INTFCR_SPEC, 0> {
        CGIF1_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Transfer Complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif1(&mut self) -> CTCIF1_W<INTFCR_SPEC, 1> {
        CTCIF1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 1 Half Transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif1(&mut self) -> CHTIF1_W<INTFCR_SPEC, 2> {
        CHTIF1_W::new(self)
    }
    #[doc = "Bit 3 - Channel 1 Transfer Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif1(&mut self) -> CTEIF1_W<INTFCR_SPEC, 3> {
        CTEIF1_W::new(self)
    }
    #[doc = "Bit 4 - Channel 2 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif2(&mut self) -> CGIF2_W<INTFCR_SPEC, 4> {
        CGIF2_W::new(self)
    }
    #[doc = "Bit 5 - Channel 2 Transfer Complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif2(&mut self) -> CTCIF2_W<INTFCR_SPEC, 5> {
        CTCIF2_W::new(self)
    }
    #[doc = "Bit 6 - Channel 2 Half Transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif2(&mut self) -> CHTIF2_W<INTFCR_SPEC, 6> {
        CHTIF2_W::new(self)
    }
    #[doc = "Bit 7 - Channel 2 Transfer Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif2(&mut self) -> CTEIF2_W<INTFCR_SPEC, 7> {
        CTEIF2_W::new(self)
    }
    #[doc = "Bit 8 - Channel 3 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif3(&mut self) -> CGIF3_W<INTFCR_SPEC, 8> {
        CGIF3_W::new(self)
    }
    #[doc = "Bit 9 - Channel 3 Transfer Complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif3(&mut self) -> CTCIF3_W<INTFCR_SPEC, 9> {
        CTCIF3_W::new(self)
    }
    #[doc = "Bit 10 - Channel 3 Half Transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif3(&mut self) -> CHTIF3_W<INTFCR_SPEC, 10> {
        CHTIF3_W::new(self)
    }
    #[doc = "Bit 11 - Channel 3 Transfer Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif3(&mut self) -> CTEIF3_W<INTFCR_SPEC, 11> {
        CTEIF3_W::new(self)
    }
    #[doc = "Bit 12 - Channel 4 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif4(&mut self) -> CGIF4_W<INTFCR_SPEC, 12> {
        CGIF4_W::new(self)
    }
    #[doc = "Bit 13 - Channel 4 Transfer Complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif4(&mut self) -> CTCIF4_W<INTFCR_SPEC, 13> {
        CTCIF4_W::new(self)
    }
    #[doc = "Bit 14 - Channel 4 Half Transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif4(&mut self) -> CHTIF4_W<INTFCR_SPEC, 14> {
        CHTIF4_W::new(self)
    }
    #[doc = "Bit 15 - Channel 4 Transfer Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif4(&mut self) -> CTEIF4_W<INTFCR_SPEC, 15> {
        CTEIF4_W::new(self)
    }
    #[doc = "Bit 16 - Channel 5 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif5(&mut self) -> CGIF5_W<INTFCR_SPEC, 16> {
        CGIF5_W::new(self)
    }
    #[doc = "Bit 17 - Channel 5 Transfer Complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif5(&mut self) -> CTCIF5_W<INTFCR_SPEC, 17> {
        CTCIF5_W::new(self)
    }
    #[doc = "Bit 18 - Channel 5 Half Transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif5(&mut self) -> CHTIF5_W<INTFCR_SPEC, 18> {
        CHTIF5_W::new(self)
    }
    #[doc = "Bit 19 - Channel 5 Transfer Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif5(&mut self) -> CTEIF5_W<INTFCR_SPEC, 19> {
        CTEIF5_W::new(self)
    }
    #[doc = "Bit 20 - Channel 6 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif6(&mut self) -> CGIF6_W<INTFCR_SPEC, 20> {
        CGIF6_W::new(self)
    }
    #[doc = "Bit 21 - Channel 6 Transfer Complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif6(&mut self) -> CTCIF6_W<INTFCR_SPEC, 21> {
        CTCIF6_W::new(self)
    }
    #[doc = "Bit 22 - Channel 6 Half Transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif6(&mut self) -> CHTIF6_W<INTFCR_SPEC, 22> {
        CHTIF6_W::new(self)
    }
    #[doc = "Bit 23 - Channel 6 Transfer Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif6(&mut self) -> CTEIF6_W<INTFCR_SPEC, 23> {
        CTEIF6_W::new(self)
    }
    #[doc = "Bit 24 - Channel 7 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif7(&mut self) -> CGIF7_W<INTFCR_SPEC, 24> {
        CGIF7_W::new(self)
    }
    #[doc = "Bit 25 - Channel 7 Transfer Complete clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif7(&mut self) -> CTCIF7_W<INTFCR_SPEC, 25> {
        CTCIF7_W::new(self)
    }
    #[doc = "Bit 26 - Channel 7 Half Transfer clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif7(&mut self) -> CHTIF7_W<INTFCR_SPEC, 26> {
        CHTIF7_W::new(self)
    }
    #[doc = "Bit 27 - Channel 7 Transfer Error clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif7(&mut self) -> CTEIF7_W<INTFCR_SPEC, 27> {
        CTEIF7_W::new(self)
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
#[doc = "DMA interrupt flag clear register (DMA_INTFCR)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intfcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFCR_SPEC;
impl crate::RegisterSpec for INTFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intfcr::W`](W) writer structure"]
impl crate::Writable for INTFCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFCR to value 0"]
impl crate::Resettable for INTFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
