#[doc = "Register `STK_CMPLR` reader"]
pub type R = crate::R<STK_CMPLR_SPEC>;
#[doc = "Register `STK_CMPLR` writer"]
pub type W = crate::W<STK_CMPLR_SPEC>;
#[doc = "Field `CMPL` reader - CMPL"]
pub type CMPL_R = crate::FieldReader<u32>;
#[doc = "Field `CMPL` writer - CMPL"]
pub type CMPL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - CMPL"]
    #[inline(always)]
    pub fn cmpl(&self) -> CMPL_R {
        CMPL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CMPL"]
    #[inline(always)]
    #[must_use]
    pub fn cmpl(&mut self) -> CMPL_W<STK_CMPLR_SPEC, 0> {
        CMPL_W::new(self)
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
#[doc = "System compare low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stk_cmplr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stk_cmplr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STK_CMPLR_SPEC;
impl crate::RegisterSpec for STK_CMPLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stk_cmplr::R`](R) reader structure"]
impl crate::Readable for STK_CMPLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stk_cmplr::W`](W) writer structure"]
impl crate::Writable for STK_CMPLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STK_CMPLR to value 0"]
impl crate::Resettable for STK_CMPLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
