#[doc = "Register `CNTR6` reader"]
pub type R = crate::R<CNTR6_SPEC>;
#[doc = "Register `CNTR6` writer"]
pub type W = crate::W<CNTR6_SPEC>;
#[doc = "Field `NDT` reader - Number of data to transfer"]
pub type NDT_R = crate::FieldReader<u16>;
#[doc = "Field `NDT` writer - Number of data to transfer"]
pub type NDT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    #[must_use]
    pub fn ndt(&mut self) -> NDT_W<CNTR6_SPEC, 0> {
        NDT_W::new(self)
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
#[doc = "DMA channel 6 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTR6_SPEC;
impl crate::RegisterSpec for CNTR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntr6::R`](R) reader structure"]
impl crate::Readable for CNTR6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cntr6::W`](W) writer structure"]
impl crate::Writable for CNTR6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNTR6 to value 0"]
impl crate::Resettable for CNTR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
