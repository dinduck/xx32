#[doc = "Register `IPSR1` writer"]
pub type W = crate::W<IPSR1_SPEC>;
#[doc = "Field `PENDSET2_3` writer - PENDSET"]
pub type PENDSET2_3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PENDSET12_31` writer - PENDSET"]
pub type PENDSET12_31_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 20, O, u32>;
impl W {
    #[doc = "Bits 2:3 - PENDSET"]
    #[inline(always)]
    #[must_use]
    pub fn pendset2_3(&mut self) -> PENDSET2_3_W<IPSR1_SPEC, 2> {
        PENDSET2_3_W::new(self)
    }
    #[doc = "Bits 12:31 - PENDSET"]
    #[inline(always)]
    #[must_use]
    pub fn pendset12_31(&mut self) -> PENDSET12_31_W<IPSR1_SPEC, 12> {
        PENDSET12_31_W::new(self)
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
#[doc = "Interrupt Pending Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipsr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPSR1_SPEC;
impl crate::RegisterSpec for IPSR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ipsr1::W`](W) writer structure"]
impl crate::Writable for IPSR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPSR1 to value 0"]
impl crate::Resettable for IPSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
