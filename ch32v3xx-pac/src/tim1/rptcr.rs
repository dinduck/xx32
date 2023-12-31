#[doc = "Register `RPTCR` reader"]
pub type R = crate::R<RPTCR_SPEC>;
#[doc = "Register `RPTCR` writer"]
pub type W = crate::W<RPTCR_SPEC>;
#[doc = "Field `RPTCR` reader - Repetition counter value"]
pub type RPTCR_R = crate::FieldReader;
#[doc = "Field `RPTCR` writer - Repetition counter value"]
pub type RPTCR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Repetition counter value"]
    #[inline(always)]
    pub fn rptcr(&self) -> RPTCR_R {
        RPTCR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Repetition counter value"]
    #[inline(always)]
    #[must_use]
    pub fn rptcr(&mut self) -> RPTCR_W<RPTCR_SPEC, 0> {
        RPTCR_W::new(self)
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
#[doc = "repetition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rptcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rptcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RPTCR_SPEC;
impl crate::RegisterSpec for RPTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rptcr::R`](R) reader structure"]
impl crate::Readable for RPTCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rptcr::W`](W) writer structure"]
impl crate::Writable for RPTCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RPTCR to value 0"]
impl crate::Resettable for RPTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
