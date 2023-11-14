#[doc = "Register `EXTEN_INTFCR` reader"]
pub type R = crate::R<EXTEN_INTFCR_SPEC>;
#[doc = "Register `EXTEN_INTFCR` writer"]
pub type W = crate::W<EXTEN_INTFCR_SPEC>;
#[doc = "Field `CGIF8` reader - Channel 8 Global interrupt clear"]
pub type CGIF8_R = crate::BitReader;
#[doc = "Field `CGIF8` writer - Channel 8 Global interrupt clear"]
pub type CGIF8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTCIF8` reader - Channel 8 Global interrupt clear"]
pub type CTCIF8_R = crate::BitReader;
#[doc = "Field `CTCIF8` writer - Channel 8 Global interrupt clear"]
pub type CTCIF8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHTIF8` reader - Channel 8 Global interrupt clear"]
pub type CHTIF8_R = crate::BitReader;
#[doc = "Field `CHTIF8` writer - Channel 8 Global interrupt clear"]
pub type CHTIF8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTEIF8` reader - Channel 8 Global interrupt clear"]
pub type CTEIF8_R = crate::BitReader;
#[doc = "Field `CTEIF8` writer - Channel 8 Global interrupt clear"]
pub type CTEIF8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CGIF9` reader - Channel 9 Global interrupt clear"]
pub type CGIF9_R = crate::BitReader;
#[doc = "Field `CGIF9` writer - Channel 9 Global interrupt clear"]
pub type CGIF9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTCIF9` reader - Channel 9 Global interrupt clear"]
pub type CTCIF9_R = crate::BitReader;
#[doc = "Field `CTCIF9` writer - Channel 9 Global interrupt clear"]
pub type CTCIF9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHTIF9` reader - Channel 9 Global interrupt clear"]
pub type CHTIF9_R = crate::BitReader;
#[doc = "Field `CHTIF9` writer - Channel 9 Global interrupt clear"]
pub type CHTIF9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTEIF9` reader - Channel 9 Global interrupt clear"]
pub type CTEIF9_R = crate::BitReader;
#[doc = "Field `CTEIF9` writer - Channel 9 Global interrupt clear"]
pub type CTEIF9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CGIF10` reader - Channel 10 Global interrupt clear"]
pub type CGIF10_R = crate::BitReader;
#[doc = "Field `CGIF10` writer - Channel 10 Global interrupt clear"]
pub type CGIF10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTCIF10` reader - Channel 10 Global interrupt clear"]
pub type CTCIF10_R = crate::BitReader;
#[doc = "Field `CTCIF10` writer - Channel 10 Global interrupt clear"]
pub type CTCIF10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHTIF10` reader - Channel 10 Global interrupt clear"]
pub type CHTIF10_R = crate::BitReader;
#[doc = "Field `CHTIF10` writer - Channel 10 Global interrupt clear"]
pub type CHTIF10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTEIF10` reader - Channel 10 Global interrupt clear"]
pub type CTEIF10_R = crate::BitReader;
#[doc = "Field `CTEIF10` writer - Channel 10 Global interrupt clear"]
pub type CTEIF10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CGIF11` reader - Channel 11 Global interrupt clear"]
pub type CGIF11_R = crate::BitReader;
#[doc = "Field `CGIF11` writer - Channel 11 Global interrupt clear"]
pub type CGIF11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTCIF11` reader - Channel 11 Global interrupt clear"]
pub type CTCIF11_R = crate::BitReader;
#[doc = "Field `CTCIF11` writer - Channel 11 Global interrupt clear"]
pub type CTCIF11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHTIF11` reader - Channel 11 Global interrupt clear"]
pub type CHTIF11_R = crate::BitReader;
#[doc = "Field `CHTIF11` writer - Channel 11 Global interrupt clear"]
pub type CHTIF11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTEIF11` reader - Channel 11 Global interrupt clear"]
pub type CTEIF11_R = crate::BitReader;
#[doc = "Field `CTEIF11` writer - Channel 11 Global interrupt clear"]
pub type CTEIF11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Channel 8 Global interrupt clear"]
    #[inline(always)]
    pub fn cgif8(&self) -> CGIF8_R {
        CGIF8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 8 Global interrupt clear"]
    #[inline(always)]
    pub fn ctcif8(&self) -> CTCIF8_R {
        CTCIF8_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 8 Global interrupt clear"]
    #[inline(always)]
    pub fn chtif8(&self) -> CHTIF8_R {
        CHTIF8_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 8 Global interrupt clear"]
    #[inline(always)]
    pub fn cteif8(&self) -> CTEIF8_R {
        CTEIF8_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 9 Global interrupt clear"]
    #[inline(always)]
    pub fn cgif9(&self) -> CGIF9_R {
        CGIF9_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 9 Global interrupt clear"]
    #[inline(always)]
    pub fn ctcif9(&self) -> CTCIF9_R {
        CTCIF9_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 9 Global interrupt clear"]
    #[inline(always)]
    pub fn chtif9(&self) -> CHTIF9_R {
        CHTIF9_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 9 Global interrupt clear"]
    #[inline(always)]
    pub fn cteif9(&self) -> CTEIF9_R {
        CTEIF9_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 10 Global interrupt clear"]
    #[inline(always)]
    pub fn cgif10(&self) -> CGIF10_R {
        CGIF10_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 10 Global interrupt clear"]
    #[inline(always)]
    pub fn ctcif10(&self) -> CTCIF10_R {
        CTCIF10_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Global interrupt clear"]
    #[inline(always)]
    pub fn chtif10(&self) -> CHTIF10_R {
        CHTIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 10 Global interrupt clear"]
    #[inline(always)]
    pub fn cteif10(&self) -> CTEIF10_R {
        CTEIF10_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 11 Global interrupt clear"]
    #[inline(always)]
    pub fn cgif11(&self) -> CGIF11_R {
        CGIF11_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 11 Global interrupt clear"]
    #[inline(always)]
    pub fn ctcif11(&self) -> CTCIF11_R {
        CTCIF11_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 11 Global interrupt clear"]
    #[inline(always)]
    pub fn chtif11(&self) -> CHTIF11_R {
        CHTIF11_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 11 Global interrupt clear"]
    #[inline(always)]
    pub fn cteif11(&self) -> CTEIF11_R {
        CTEIF11_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 8 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif8(&mut self) -> CGIF8_W<EXTEN_INTFCR_SPEC, 0> {
        CGIF8_W::new(self)
    }
    #[doc = "Bit 1 - Channel 8 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif8(&mut self) -> CTCIF8_W<EXTEN_INTFCR_SPEC, 1> {
        CTCIF8_W::new(self)
    }
    #[doc = "Bit 2 - Channel 8 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif8(&mut self) -> CHTIF8_W<EXTEN_INTFCR_SPEC, 2> {
        CHTIF8_W::new(self)
    }
    #[doc = "Bit 3 - Channel 8 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif8(&mut self) -> CTEIF8_W<EXTEN_INTFCR_SPEC, 3> {
        CTEIF8_W::new(self)
    }
    #[doc = "Bit 4 - Channel 9 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif9(&mut self) -> CGIF9_W<EXTEN_INTFCR_SPEC, 4> {
        CGIF9_W::new(self)
    }
    #[doc = "Bit 5 - Channel 9 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif9(&mut self) -> CTCIF9_W<EXTEN_INTFCR_SPEC, 5> {
        CTCIF9_W::new(self)
    }
    #[doc = "Bit 6 - Channel 9 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif9(&mut self) -> CHTIF9_W<EXTEN_INTFCR_SPEC, 6> {
        CHTIF9_W::new(self)
    }
    #[doc = "Bit 7 - Channel 9 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif9(&mut self) -> CTEIF9_W<EXTEN_INTFCR_SPEC, 7> {
        CTEIF9_W::new(self)
    }
    #[doc = "Bit 8 - Channel 10 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif10(&mut self) -> CGIF10_W<EXTEN_INTFCR_SPEC, 8> {
        CGIF10_W::new(self)
    }
    #[doc = "Bit 9 - Channel 10 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif10(&mut self) -> CTCIF10_W<EXTEN_INTFCR_SPEC, 9> {
        CTCIF10_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif10(&mut self) -> CHTIF10_W<EXTEN_INTFCR_SPEC, 10> {
        CHTIF10_W::new(self)
    }
    #[doc = "Bit 11 - Channel 10 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif10(&mut self) -> CTEIF10_W<EXTEN_INTFCR_SPEC, 11> {
        CTEIF10_W::new(self)
    }
    #[doc = "Bit 12 - Channel 11 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cgif11(&mut self) -> CGIF11_W<EXTEN_INTFCR_SPEC, 12> {
        CGIF11_W::new(self)
    }
    #[doc = "Bit 13 - Channel 11 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif11(&mut self) -> CTCIF11_W<EXTEN_INTFCR_SPEC, 13> {
        CTCIF11_W::new(self)
    }
    #[doc = "Bit 14 - Channel 11 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn chtif11(&mut self) -> CHTIF11_W<EXTEN_INTFCR_SPEC, 14> {
        CHTIF11_W::new(self)
    }
    #[doc = "Bit 15 - Channel 11 Global interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cteif11(&mut self) -> CTEIF11_W<EXTEN_INTFCR_SPEC, 15> {
        CTEIF11_W::new(self)
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
#[doc = "DMA2 EXTEN interrupt flag clear register (DMA_INTFCR)used in ch32v30x_D8/D8C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exten_intfcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exten_intfcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTEN_INTFCR_SPEC;
impl crate::RegisterSpec for EXTEN_INTFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exten_intfcr::R`](R) reader structure"]
impl crate::Readable for EXTEN_INTFCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exten_intfcr::W`](W) writer structure"]
impl crate::Writable for EXTEN_INTFCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTEN_INTFCR to value 0"]
impl crate::Resettable for EXTEN_INTFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
