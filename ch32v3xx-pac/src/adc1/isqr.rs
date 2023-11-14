#[doc = "Register `ISQR` reader"]
pub type R = crate::R<ISQR_SPEC>;
#[doc = "Register `ISQR` writer"]
pub type W = crate::W<ISQR_SPEC>;
#[doc = "Field `JSQ1` reader - 1st conversion in injected sequence"]
pub type JSQ1_R = crate::FieldReader;
#[doc = "Field `JSQ1` writer - 1st conversion in injected sequence"]
pub type JSQ1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `JSQ2` reader - 2nd conversion in injected sequence"]
pub type JSQ2_R = crate::FieldReader;
#[doc = "Field `JSQ2` writer - 2nd conversion in injected sequence"]
pub type JSQ2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `JSQ3` reader - 3rd conversion in injected sequence"]
pub type JSQ3_R = crate::FieldReader;
#[doc = "Field `JSQ3` writer - 3rd conversion in injected sequence"]
pub type JSQ3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `JSQ4` reader - 4th conversion in injected sequence"]
pub type JSQ4_R = crate::FieldReader;
#[doc = "Field `JSQ4` writer - 4th conversion in injected sequence"]
pub type JSQ4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `JL` reader - Injected sequence length"]
pub type JL_R = crate::FieldReader;
#[doc = "Field `JL` writer - Injected sequence length"]
pub type JL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:4 - 1st conversion in injected sequence"]
    #[inline(always)]
    pub fn jsq1(&self) -> JSQ1_R {
        JSQ1_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 2nd conversion in injected sequence"]
    #[inline(always)]
    pub fn jsq2(&self) -> JSQ2_R {
        JSQ2_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 3rd conversion in injected sequence"]
    #[inline(always)]
    pub fn jsq3(&self) -> JSQ3_R {
        JSQ3_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 4th conversion in injected sequence"]
    #[inline(always)]
    pub fn jsq4(&self) -> JSQ4_R {
        JSQ4_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:21 - Injected sequence length"]
    #[inline(always)]
    pub fn jl(&self) -> JL_R {
        JL_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 1st conversion in injected sequence"]
    #[inline(always)]
    #[must_use]
    pub fn jsq1(&mut self) -> JSQ1_W<ISQR_SPEC, 0> {
        JSQ1_W::new(self)
    }
    #[doc = "Bits 5:9 - 2nd conversion in injected sequence"]
    #[inline(always)]
    #[must_use]
    pub fn jsq2(&mut self) -> JSQ2_W<ISQR_SPEC, 5> {
        JSQ2_W::new(self)
    }
    #[doc = "Bits 10:14 - 3rd conversion in injected sequence"]
    #[inline(always)]
    #[must_use]
    pub fn jsq3(&mut self) -> JSQ3_W<ISQR_SPEC, 10> {
        JSQ3_W::new(self)
    }
    #[doc = "Bits 15:19 - 4th conversion in injected sequence"]
    #[inline(always)]
    #[must_use]
    pub fn jsq4(&mut self) -> JSQ4_W<ISQR_SPEC, 15> {
        JSQ4_W::new(self)
    }
    #[doc = "Bits 20:21 - Injected sequence length"]
    #[inline(always)]
    #[must_use]
    pub fn jl(&mut self) -> JL_W<ISQR_SPEC, 20> {
        JL_W::new(self)
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
#[doc = "injected sequence register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isqr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isqr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISQR_SPEC;
impl crate::RegisterSpec for ISQR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isqr::R`](R) reader structure"]
impl crate::Readable for ISQR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`isqr::W`](W) writer structure"]
impl crate::Writable for ISQR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISQR to value 0"]
impl crate::Resettable for ISQR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
