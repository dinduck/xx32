#[doc = "Register `CH4CVR` reader"]
pub type R = crate::R<CH4CVR_SPEC>;
#[doc = "Register `CH4CVR` writer"]
pub type W = crate::W<CH4CVR_SPEC>;
#[doc = "Field `CH4CVR` reader - Capture/Compare value"]
pub type CH4CVR_R = crate::FieldReader<u16>;
#[doc = "Field `CH4CVR` writer - Capture/Compare value"]
pub type CH4CVR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare value"]
    #[inline(always)]
    pub fn ch4cvr(&self) -> CH4CVR_R {
        CH4CVR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare value"]
    #[inline(always)]
    #[must_use]
    pub fn ch4cvr(&mut self) -> CH4CVR_W<CH4CVR_SPEC, 0> {
        CH4CVR_W::new(self)
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
#[doc = "capture/compare register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4cvr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4cvr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH4CVR_SPEC;
impl crate::RegisterSpec for CH4CVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4cvr::R`](R) reader structure"]
impl crate::Readable for CH4CVR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch4cvr::W`](W) writer structure"]
impl crate::Writable for CH4CVR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH4CVR to value 0"]
impl crate::Resettable for CH4CVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
