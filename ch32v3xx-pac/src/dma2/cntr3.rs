#[doc = "Register `CNTR3` reader"]
pub type R = crate::R<CNTR3_SPEC>;
#[doc = "Register `CNTR3` writer"]
pub type W = crate::W<CNTR3_SPEC>;
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
    pub fn ndt(&mut self) -> NDT_W<CNTR3_SPEC, 0> {
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
#[doc = "DMA channel 3 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTR3_SPEC;
impl crate::RegisterSpec for CNTR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntr3::R`](R) reader structure"]
impl crate::Readable for CNTR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cntr3::W`](W) writer structure"]
impl crate::Writable for CNTR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNTR3 to value 0"]
impl crate::Resettable for CNTR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
