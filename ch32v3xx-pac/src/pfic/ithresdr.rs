#[doc = "Register `ITHRESDR` reader"]
pub type R = crate::R<ITHRESDR_SPEC>;
#[doc = "Register `ITHRESDR` writer"]
pub type W = crate::W<ITHRESDR_SPEC>;
#[doc = "Field `THRESHOLD` reader - THRESHOLD"]
pub type THRESHOLD_R = crate::FieldReader;
#[doc = "Field `THRESHOLD` writer - THRESHOLD"]
pub type THRESHOLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - THRESHOLD"]
    #[inline(always)]
    pub fn threshold(&self) -> THRESHOLD_R {
        THRESHOLD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - THRESHOLD"]
    #[inline(always)]
    #[must_use]
    pub fn threshold(&mut self) -> THRESHOLD_W<ITHRESDR_SPEC, 0> {
        THRESHOLD_W::new(self)
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
#[doc = "Interrupt Priority Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ithresdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ithresdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITHRESDR_SPEC;
impl crate::RegisterSpec for ITHRESDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ithresdr::R`](R) reader structure"]
impl crate::Readable for ITHRESDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ithresdr::W`](W) writer structure"]
impl crate::Writable for ITHRESDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ITHRESDR to value 0"]
impl crate::Resettable for ITHRESDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
