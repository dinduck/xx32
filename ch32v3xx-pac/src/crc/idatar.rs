#[doc = "Register `IDATAR` reader"]
pub type R = crate::R<IDATAR_SPEC>;
#[doc = "Register `IDATAR` writer"]
pub type W = crate::W<IDATAR_SPEC>;
#[doc = "Field `IDR` reader - Independent Data register"]
pub type IDR_R = crate::FieldReader;
#[doc = "Field `IDR` writer - Independent Data register"]
pub type IDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Independent Data register"]
    #[inline(always)]
    pub fn idr(&self) -> IDR_R {
        IDR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Independent Data register"]
    #[inline(always)]
    #[must_use]
    pub fn idr(&mut self) -> IDR_W<IDATAR_SPEC, 0> {
        IDR_W::new(self)
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
#[doc = "Independent Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idatar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idatar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDATAR_SPEC;
impl crate::RegisterSpec for IDATAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idatar::R`](R) reader structure"]
impl crate::Readable for IDATAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`idatar::W`](W) writer structure"]
impl crate::Writable for IDATAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDATAR to value 0"]
impl crate::Resettable for IDATAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
