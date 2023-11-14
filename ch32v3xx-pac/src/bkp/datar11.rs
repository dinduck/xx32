#[doc = "Register `DATAR11` reader"]
pub type R = crate::R<DATAR11_SPEC>;
#[doc = "Register `DATAR11` writer"]
pub type W = crate::W<DATAR11_SPEC>;
#[doc = "Field `DR11` reader - Backup data"]
pub type DR11_R = crate::FieldReader<u16>;
#[doc = "Field `DR11` writer - Backup data"]
pub type DR11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn dr11(&self) -> DR11_R {
        DR11_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn dr11(&mut self) -> DR11_W<DATAR11_SPEC, 0> {
        DR11_W::new(self)
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
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATAR11_SPEC;
impl crate::RegisterSpec for DATAR11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datar11::R`](R) reader structure"]
impl crate::Readable for DATAR11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`datar11::W`](W) writer structure"]
impl crate::Writable for DATAR11_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATAR11 to value 0"]
impl crate::Resettable for DATAR11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
