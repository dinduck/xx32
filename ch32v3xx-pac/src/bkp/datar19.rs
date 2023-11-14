#[doc = "Register `DATAR19` reader"]
pub type R = crate::R<DATAR19_SPEC>;
#[doc = "Register `DATAR19` writer"]
pub type W = crate::W<DATAR19_SPEC>;
#[doc = "Field `D19` reader - Backup data"]
pub type D19_R = crate::FieldReader<u16>;
#[doc = "Field `D19` writer - Backup data"]
pub type D19_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d19(&self) -> D19_R {
        D19_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d19(&mut self) -> D19_W<DATAR19_SPEC, 0> {
        D19_W::new(self)
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
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar19::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar19::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATAR19_SPEC;
impl crate::RegisterSpec for DATAR19_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datar19::R`](R) reader structure"]
impl crate::Readable for DATAR19_SPEC {}
#[doc = "`write(|w| ..)` method takes [`datar19::W`](W) writer structure"]
impl crate::Writable for DATAR19_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATAR19 to value 0"]
impl crate::Resettable for DATAR19_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
