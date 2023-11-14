#[doc = "Register `DATAR18` reader"]
pub type R = crate::R<DATAR18_SPEC>;
#[doc = "Register `DATAR18` writer"]
pub type W = crate::W<DATAR18_SPEC>;
#[doc = "Field `D18` reader - Backup data"]
pub type D18_R = crate::FieldReader<u16>;
#[doc = "Field `D18` writer - Backup data"]
pub type D18_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d18(&self) -> D18_R {
        D18_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d18(&mut self) -> D18_W<DATAR18_SPEC, 0> {
        D18_W::new(self)
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
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar18::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar18::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATAR18_SPEC;
impl crate::RegisterSpec for DATAR18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datar18::R`](R) reader structure"]
impl crate::Readable for DATAR18_SPEC {}
#[doc = "`write(|w| ..)` method takes [`datar18::W`](W) writer structure"]
impl crate::Writable for DATAR18_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATAR18 to value 0"]
impl crate::Resettable for DATAR18_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
