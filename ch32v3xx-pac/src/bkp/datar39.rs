#[doc = "Register `DATAR39` reader"]
pub type R = crate::R<DATAR39_SPEC>;
#[doc = "Register `DATAR39` writer"]
pub type W = crate::W<DATAR39_SPEC>;
#[doc = "Field `D39` reader - Backup data"]
pub type D39_R = crate::FieldReader<u16>;
#[doc = "Field `D39` writer - Backup data"]
pub type D39_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d39(&self) -> D39_R {
        D39_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d39(&mut self) -> D39_W<DATAR39_SPEC, 0> {
        D39_W::new(self)
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
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar39::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar39::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATAR39_SPEC;
impl crate::RegisterSpec for DATAR39_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datar39::R`](R) reader structure"]
impl crate::Readable for DATAR39_SPEC {}
#[doc = "`write(|w| ..)` method takes [`datar39::W`](W) writer structure"]
impl crate::Writable for DATAR39_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATAR39 to value 0"]
impl crate::Resettable for DATAR39_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
