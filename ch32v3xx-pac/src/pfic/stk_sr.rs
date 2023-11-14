#[doc = "Register `STK_SR` reader"]
pub type R = crate::R<STK_SR_SPEC>;
#[doc = "Register `STK_SR` writer"]
pub type W = crate::W<STK_SR_SPEC>;
#[doc = "Field `CNTIF` reader - CNTIF"]
pub type CNTIF_R = crate::BitReader;
#[doc = "Field `CNTIF` writer - CNTIF"]
pub type CNTIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - CNTIF"]
    #[inline(always)]
    pub fn cntif(&self) -> CNTIF_R {
        CNTIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CNTIF"]
    #[inline(always)]
    #[must_use]
    pub fn cntif(&mut self) -> CNTIF_W<STK_SR_SPEC, 0> {
        CNTIF_W::new(self)
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
#[doc = "System START\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stk_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stk_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STK_SR_SPEC;
impl crate::RegisterSpec for STK_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stk_sr::R`](R) reader structure"]
impl crate::Readable for STK_SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stk_sr::W`](W) writer structure"]
impl crate::Writable for STK_SR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STK_SR to value 0"]
impl crate::Resettable for STK_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
