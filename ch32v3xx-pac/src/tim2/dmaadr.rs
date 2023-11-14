#[doc = "Register `DMAADR` reader"]
pub type R = crate::R<DMAADR_SPEC>;
#[doc = "Register `DMAADR` writer"]
pub type W = crate::W<DMAADR_SPEC>;
#[doc = "Field `DMAADR` reader - DMA register for burst accesses"]
pub type DMAADR_R = crate::FieldReader<u16>;
#[doc = "Field `DMAADR` writer - DMA register for burst accesses"]
pub type DMAADR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - DMA register for burst accesses"]
    #[inline(always)]
    pub fn dmaadr(&self) -> DMAADR_R {
        DMAADR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DMA register for burst accesses"]
    #[inline(always)]
    #[must_use]
    pub fn dmaadr(&mut self) -> DMAADR_W<DMAADR_SPEC, 0> {
        DMAADR_W::new(self)
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
#[doc = "DMA address for full transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaadr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaadr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAADR_SPEC;
impl crate::RegisterSpec for DMAADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaadr::R`](R) reader structure"]
impl crate::Readable for DMAADR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmaadr::W`](W) writer structure"]
impl crate::Writable for DMAADR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAADR to value 0"]
impl crate::Resettable for DMAADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
