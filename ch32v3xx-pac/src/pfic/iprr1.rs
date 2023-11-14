#[doc = "Register `IPRR1` writer"]
pub type W = crate::W<IPRR1_SPEC>;
#[doc = "Field `PENDRESET2_3` writer - PENDRESET"]
pub type PENDRESET2_3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PENDRESET12_31` writer - PENDRESET"]
pub type PENDRESET12_31_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 20, O, u32>;
impl W {
    #[doc = "Bits 2:3 - PENDRESET"]
    #[inline(always)]
    #[must_use]
    pub fn pendreset2_3(&mut self) -> PENDRESET2_3_W<IPRR1_SPEC, 2> {
        PENDRESET2_3_W::new(self)
    }
    #[doc = "Bits 12:31 - PENDRESET"]
    #[inline(always)]
    #[must_use]
    pub fn pendreset12_31(&mut self) -> PENDRESET12_31_W<IPRR1_SPEC, 12> {
        PENDRESET12_31_W::new(self)
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
#[doc = "Interrupt Pending Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iprr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPRR1_SPEC;
impl crate::RegisterSpec for IPRR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iprr1::W`](W) writer structure"]
impl crate::Writable for IPRR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPRR1 to value 0"]
impl crate::Resettable for IPRR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
