#[doc = "Register `STATR` reader"]
pub type R = crate::R<STATR_SPEC>;
#[doc = "Register `STATR` writer"]
pub type W = crate::W<STATR_SPEC>;
#[doc = "Field `WEIF` reader - Early Wakeup Interrupt Flag"]
pub type WEIF_R = crate::BitReader;
#[doc = "Field `WEIF` writer - Early Wakeup Interrupt Flag"]
pub type WEIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Early Wakeup Interrupt Flag"]
    #[inline(always)]
    pub fn weif(&self) -> WEIF_R {
        WEIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Early Wakeup Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn weif(&mut self) -> WEIF_W<STATR_SPEC, 0> {
        WEIF_W::new(self)
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
#[doc = "Status register (WWDG_SR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`statr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATR_SPEC;
impl crate::RegisterSpec for STATR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statr::R`](R) reader structure"]
impl crate::Readable for STATR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`statr::W`](W) writer structure"]
impl crate::Writable for STATR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATR to value 0"]
impl crate::Resettable for STATR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
