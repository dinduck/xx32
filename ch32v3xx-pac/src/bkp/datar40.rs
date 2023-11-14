#[doc = "Register `DATAR40` reader"]
pub type R = crate::R<DATAR40_SPEC>;
#[doc = "Register `DATAR40` writer"]
pub type W = crate::W<DATAR40_SPEC>;
#[doc = "Field `D40` reader - Backup data"]
pub type D40_R = crate::FieldReader<u16>;
#[doc = "Field `D40` writer - Backup data"]
pub type D40_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d40(&self) -> D40_R {
        D40_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d40(&mut self) -> D40_W<DATAR40_SPEC, 0> {
        D40_W::new(self)
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
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar40::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar40::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATAR40_SPEC;
impl crate::RegisterSpec for DATAR40_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datar40::R`](R) reader structure"]
impl crate::Readable for DATAR40_SPEC {}
#[doc = "`write(|w| ..)` method takes [`datar40::W`](W) writer structure"]
impl crate::Writable for DATAR40_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATAR40 to value 0"]
impl crate::Resettable for DATAR40_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
