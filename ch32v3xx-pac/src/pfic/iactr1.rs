#[doc = "Register `IACTR1` writer"]
pub type W = crate::W<IACTR1_SPEC>;
#[doc = "Field `IACTS2_3` writer - IACTS"]
pub type IACTS2_3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IACTS12_31` writer - IACTS"]
pub type IACTS12_31_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 20, O, u32>;
impl W {
    #[doc = "Bits 2:3 - IACTS"]
    #[inline(always)]
    #[must_use]
    pub fn iacts2_3(&mut self) -> IACTS2_3_W<IACTR1_SPEC, 2> {
        IACTS2_3_W::new(self)
    }
    #[doc = "Bits 12:31 - IACTS"]
    #[inline(always)]
    #[must_use]
    pub fn iacts12_31(&mut self) -> IACTS12_31_W<IACTR1_SPEC, 12> {
        IACTS12_31_W::new(self)
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
#[doc = "Interrupt ACTIVE Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iactr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IACTR1_SPEC;
impl crate::RegisterSpec for IACTR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iactr1::W`](W) writer structure"]
impl crate::Writable for IACTR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IACTR1 to value 0"]
impl crate::Resettable for IACTR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
