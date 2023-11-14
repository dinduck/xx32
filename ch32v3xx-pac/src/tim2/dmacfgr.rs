#[doc = "Register `DMACFGR` reader"]
pub type R = crate::R<DMACFGR_SPEC>;
#[doc = "Register `DMACFGR` writer"]
pub type W = crate::W<DMACFGR_SPEC>;
#[doc = "Field `DBA` reader - DMA base address"]
pub type DBA_R = crate::FieldReader;
#[doc = "Field `DBA` writer - DMA base address"]
pub type DBA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `DBL` reader - DMA burst length"]
pub type DBL_R = crate::FieldReader;
#[doc = "Field `DBL` writer - DMA burst length"]
pub type DBL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - DMA base address"]
    #[inline(always)]
    pub fn dba(&self) -> DBA_R {
        DBA_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DMA burst length"]
    #[inline(always)]
    pub fn dbl(&self) -> DBL_R {
        DBL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA base address"]
    #[inline(always)]
    #[must_use]
    pub fn dba(&mut self) -> DBA_W<DMACFGR_SPEC, 0> {
        DBA_W::new(self)
    }
    #[doc = "Bits 8:12 - DMA burst length"]
    #[inline(always)]
    #[must_use]
    pub fn dbl(&mut self) -> DBL_W<DMACFGR_SPEC, 8> {
        DBL_W::new(self)
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
#[doc = "DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACFGR_SPEC;
impl crate::RegisterSpec for DMACFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacfgr::R`](R) reader structure"]
impl crate::Readable for DMACFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmacfgr::W`](W) writer structure"]
impl crate::Writable for DMACFGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACFGR to value 0"]
impl crate::Resettable for DMACFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
