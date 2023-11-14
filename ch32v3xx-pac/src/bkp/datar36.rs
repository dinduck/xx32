#[doc = "Register `DATAR36` reader"]
pub type R = crate::R<DATAR36_SPEC>;
#[doc = "Register `DATAR36` writer"]
pub type W = crate::W<DATAR36_SPEC>;
#[doc = "Field `D36` reader - Backup data"]
pub type D36_R = crate::FieldReader<u16>;
#[doc = "Field `D36` writer - Backup data"]
pub type D36_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d36(&self) -> D36_R {
        D36_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d36(&mut self) -> D36_W<DATAR36_SPEC, 0> {
        D36_W::new(self)
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
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar36::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar36::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATAR36_SPEC;
impl crate::RegisterSpec for DATAR36_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datar36::R`](R) reader structure"]
impl crate::Readable for DATAR36_SPEC {}
#[doc = "`write(|w| ..)` method takes [`datar36::W`](W) writer structure"]
impl crate::Writable for DATAR36_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATAR36 to value 0"]
impl crate::Resettable for DATAR36_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
