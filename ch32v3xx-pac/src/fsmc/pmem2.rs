#[doc = "Register `PMEM2` reader"]
pub type R = crate::R<PMEM2_SPEC>;
#[doc = "Register `PMEM2` writer"]
pub type W = crate::W<PMEM2_SPEC>;
#[doc = "Field `MEMSETx` reader - Common memory x setup time"]
pub type MEMSETX_R = crate::FieldReader;
#[doc = "Field `MEMSETx` writer - Common memory x setup time"]
pub type MEMSETX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `MEMWAITx` reader - Common memory x wait time"]
pub type MEMWAITX_R = crate::FieldReader;
#[doc = "Field `MEMWAITx` writer - Common memory x wait time"]
pub type MEMWAITX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `MEMHOLDx` reader - Common memory x hold time"]
pub type MEMHOLDX_R = crate::FieldReader;
#[doc = "Field `MEMHOLDx` writer - Common memory x hold time"]
pub type MEMHOLDX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `MEMHIZx` reader - Common memory x databus HiZ time"]
pub type MEMHIZX_R = crate::FieldReader;
#[doc = "Field `MEMHIZx` writer - Common memory x databus HiZ time"]
pub type MEMHIZX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Common memory x setup time"]
    #[inline(always)]
    pub fn memsetx(&self) -> MEMSETX_R {
        MEMSETX_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Common memory x wait time"]
    #[inline(always)]
    pub fn memwaitx(&self) -> MEMWAITX_R {
        MEMWAITX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Common memory x hold time"]
    #[inline(always)]
    pub fn memholdx(&self) -> MEMHOLDX_R {
        MEMHOLDX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Common memory x databus HiZ time"]
    #[inline(always)]
    pub fn memhizx(&self) -> MEMHIZX_R {
        MEMHIZX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Common memory x setup time"]
    #[inline(always)]
    #[must_use]
    pub fn memsetx(&mut self) -> MEMSETX_W<PMEM2_SPEC, 0> {
        MEMSETX_W::new(self)
    }
    #[doc = "Bits 8:15 - Common memory x wait time"]
    #[inline(always)]
    #[must_use]
    pub fn memwaitx(&mut self) -> MEMWAITX_W<PMEM2_SPEC, 8> {
        MEMWAITX_W::new(self)
    }
    #[doc = "Bits 16:23 - Common memory x hold time"]
    #[inline(always)]
    #[must_use]
    pub fn memholdx(&mut self) -> MEMHOLDX_W<PMEM2_SPEC, 16> {
        MEMHOLDX_W::new(self)
    }
    #[doc = "Bits 24:31 - Common memory x databus HiZ time"]
    #[inline(always)]
    #[must_use]
    pub fn memhizx(&mut self) -> MEMHIZX_W<PMEM2_SPEC, 24> {
        MEMHIZX_W::new(self)
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
#[doc = "Common memory space timing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmem2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmem2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMEM2_SPEC;
impl crate::RegisterSpec for PMEM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmem2::R`](R) reader structure"]
impl crate::Readable for PMEM2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmem2::W`](W) writer structure"]
impl crate::Writable for PMEM2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMEM2 to value 0xfcfc_fcfc"]
impl crate::Resettable for PMEM2_SPEC {
    const RESET_VALUE: Self::Ux = 0xfcfc_fcfc;
}
