#[doc = "Register `STK_CMPHR` reader"]
pub type R = crate::R<STK_CMPHR_SPEC>;
#[doc = "Register `STK_CMPHR` writer"]
pub type W = crate::W<STK_CMPHR_SPEC>;
#[doc = "Field `CMPH` reader - CMPH"]
pub type CMPH_R = crate::FieldReader<u32>;
#[doc = "Field `CMPH` writer - CMPH"]
pub type CMPH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - CMPH"]
    #[inline(always)]
    pub fn cmph(&self) -> CMPH_R {
        CMPH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CMPH"]
    #[inline(always)]
    #[must_use]
    pub fn cmph(&mut self) -> CMPH_W<STK_CMPHR_SPEC, 0> {
        CMPH_W::new(self)
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
#[doc = "System compare high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stk_cmphr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stk_cmphr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STK_CMPHR_SPEC;
impl crate::RegisterSpec for STK_CMPHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stk_cmphr::R`](R) reader structure"]
impl crate::Readable for STK_CMPHR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stk_cmphr::W`](W) writer structure"]
impl crate::Writable for STK_CMPHR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STK_CMPHR to value 0"]
impl crate::Resettable for STK_CMPHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
