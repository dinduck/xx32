#[doc = "Register `RTR` reader"]
pub type R = crate::R<RTR_SPEC>;
#[doc = "Register `RTR` writer"]
pub type W = crate::W<RTR_SPEC>;
#[doc = "Field `TRISE` reader - Maximum rise time in Fast/Standard mode (Master mode)"]
pub type TRISE_R = crate::FieldReader;
#[doc = "Field `TRISE` writer - Maximum rise time in Fast/Standard mode (Master mode)"]
pub type TRISE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Maximum rise time in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    pub fn trise(&self) -> TRISE_R {
        TRISE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Maximum rise time in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn trise(&mut self) -> TRISE_W<RTR_SPEC, 0> {
        TRISE_W::new(self)
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
#[doc = "Raise time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTR_SPEC;
impl crate::RegisterSpec for RTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtr::R`](R) reader structure"]
impl crate::Readable for RTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtr::W`](W) writer structure"]
impl crate::Writable for RTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTR to value 0x02"]
impl crate::Resettable for RTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
