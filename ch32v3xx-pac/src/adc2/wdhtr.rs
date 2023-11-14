#[doc = "Register `WDHTR` reader"]
pub type R = crate::R<WDHTR_SPEC>;
#[doc = "Register `WDHTR` writer"]
pub type W = crate::W<WDHTR_SPEC>;
#[doc = "Field `HT` reader - Analog watchdog higher threshold"]
pub type HT_R = crate::FieldReader<u16>;
#[doc = "Field `HT` writer - Analog watchdog higher threshold"]
pub type HT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Analog watchdog higher threshold"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog higher threshold"]
    #[inline(always)]
    #[must_use]
    pub fn ht(&mut self) -> HT_W<WDHTR_SPEC, 0> {
        HT_W::new(self)
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
#[doc = "watchdog higher threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdhtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdhtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDHTR_SPEC;
impl crate::RegisterSpec for WDHTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdhtr::R`](R) reader structure"]
impl crate::Readable for WDHTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdhtr::W`](W) writer structure"]
impl crate::Writable for WDHTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDHTR to value 0"]
impl crate::Resettable for WDHTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
