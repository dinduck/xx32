#[doc = "Register `STK_CNTL` reader"]
pub type R = crate::R<STK_CNTL_SPEC>;
#[doc = "Register `STK_CNTL` writer"]
pub type W = crate::W<STK_CNTL_SPEC>;
#[doc = "Field `CNTL` reader - CNTL"]
pub type CNTL_R = crate::FieldReader<u32>;
#[doc = "Field `CNTL` writer - CNTL"]
pub type CNTL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - CNTL"]
    #[inline(always)]
    pub fn cntl(&self) -> CNTL_R {
        CNTL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CNTL"]
    #[inline(always)]
    #[must_use]
    pub fn cntl(&mut self) -> CNTL_W<STK_CNTL_SPEC, 0> {
        CNTL_W::new(self)
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
#[doc = "System counter low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stk_cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stk_cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STK_CNTL_SPEC;
impl crate::RegisterSpec for STK_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stk_cntl::R`](R) reader structure"]
impl crate::Readable for STK_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stk_cntl::W`](W) writer structure"]
impl crate::Writable for STK_CNTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STK_CNTL to value 0"]
impl crate::Resettable for STK_CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
