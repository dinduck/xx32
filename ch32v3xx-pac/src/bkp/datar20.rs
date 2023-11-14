#[doc = "Register `DATAR20` reader"]
pub type R = crate::R<DATAR20_SPEC>;
#[doc = "Register `DATAR20` writer"]
pub type W = crate::W<DATAR20_SPEC>;
#[doc = "Field `D20` reader - Backup data"]
pub type D20_R = crate::FieldReader<u16>;
#[doc = "Field `D20` writer - Backup data"]
pub type D20_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d20(&self) -> D20_R {
        D20_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d20(&mut self) -> D20_W<DATAR20_SPEC, 0> {
        D20_W::new(self)
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
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATAR20_SPEC;
impl crate::RegisterSpec for DATAR20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datar20::R`](R) reader structure"]
impl crate::Readable for DATAR20_SPEC {}
#[doc = "`write(|w| ..)` method takes [`datar20::W`](W) writer structure"]
impl crate::Writable for DATAR20_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATAR20 to value 0"]
impl crate::Resettable for DATAR20_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}