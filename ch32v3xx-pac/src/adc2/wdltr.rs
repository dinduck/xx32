#[doc = "Register `WDLTR` reader"]
pub type R = crate::R<WDLTR_SPEC>;
#[doc = "Register `WDLTR` writer"]
pub type W = crate::W<WDLTR_SPEC>;
#[doc = "Field `LT` reader - Analog watchdog lower threshold"]
pub type LT_R = crate::FieldReader<u16>;
#[doc = "Field `LT` writer - Analog watchdog lower threshold"]
pub type LT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Analog watchdog lower threshold"]
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog lower threshold"]
    #[inline(always)]
    #[must_use]
    pub fn lt(&mut self) -> LT_W<WDLTR_SPEC, 0> {
        LT_W::new(self)
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
#[doc = "watchdog lower threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdltr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdltr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDLTR_SPEC;
impl crate::RegisterSpec for WDLTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdltr::R`](R) reader structure"]
impl crate::Readable for WDLTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdltr::W`](W) writer structure"]
impl crate::Writable for WDLTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDLTR to value 0"]
impl crate::Resettable for WDLTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
