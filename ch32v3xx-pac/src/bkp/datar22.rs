#[doc = "Register `DATAR22` reader"]
pub type R = crate::R<DATAR22_SPEC>;
#[doc = "Register `DATAR22` writer"]
pub type W = crate::W<DATAR22_SPEC>;
#[doc = "Field `D22` reader - Backup data"]
pub type D22_R = crate::FieldReader<u16>;
#[doc = "Field `D22` writer - Backup data"]
pub type D22_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d22(&self) -> D22_R {
        D22_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d22(&mut self) -> D22_W<DATAR22_SPEC, 0> {
        D22_W::new(self)
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
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar22::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar22::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATAR22_SPEC;
impl crate::RegisterSpec for DATAR22_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datar22::R`](R) reader structure"]
impl crate::Readable for DATAR22_SPEC {}
#[doc = "`write(|w| ..)` method takes [`datar22::W`](W) writer structure"]
impl crate::Writable for DATAR22_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATAR22 to value 0"]
impl crate::Resettable for DATAR22_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
