#[doc = "Register `DATAR` reader"]
pub type R = crate::R<DATAR_SPEC>;
#[doc = "Register `DATAR` writer"]
pub type W = crate::W<DATAR_SPEC>;
#[doc = "Field `DR` reader - Data Register"]
pub type DR_R = crate::FieldReader<u32>;
#[doc = "Field `DR` writer - Data Register"]
pub type DR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Register"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Register"]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DR_W<DATAR_SPEC, 0> {
        DR_W::new(self)
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
#[doc = "Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATAR_SPEC;
impl crate::RegisterSpec for DATAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datar::R`](R) reader structure"]
impl crate::Readable for DATAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`datar::W`](W) writer structure"]
impl crate::Writable for DATAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATAR to value 0xffff_ffff"]
impl crate::Resettable for DATAR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
