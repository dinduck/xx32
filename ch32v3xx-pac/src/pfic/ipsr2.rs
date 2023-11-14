#[doc = "Register `IPSR2` writer"]
pub type W = crate::W<IPSR2_SPEC>;
#[doc = "Field `PENDSET` writer - PENDSET"]
pub type PENDSET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - PENDSET"]
    #[inline(always)]
    #[must_use]
    pub fn pendset(&mut self) -> PENDSET_W<IPSR2_SPEC, 0> {
        PENDSET_W::new(self)
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
#[doc = "Interrupt Pending Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipsr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPSR2_SPEC;
impl crate::RegisterSpec for IPSR2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ipsr2::W`](W) writer structure"]
impl crate::Writable for IPSR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPSR2 to value 0"]
impl crate::Resettable for IPSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
