#[doc = "Register `MADDR9` reader"]
pub type R = crate::R<MADDR9_SPEC>;
#[doc = "Register `MADDR9` writer"]
pub type W = crate::W<MADDR9_SPEC>;
#[doc = "Field `MA` reader - Memory address"]
pub type MA_R = crate::FieldReader<u32>;
#[doc = "Field `MA` writer - Memory address"]
pub type MA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MA_W<MADDR9_SPEC, 0> {
        MA_W::new(self)
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
#[doc = "DMA channel 9 memory address register used in ch32v30x_D8/D8C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maddr9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maddr9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MADDR9_SPEC;
impl crate::RegisterSpec for MADDR9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maddr9::R`](R) reader structure"]
impl crate::Readable for MADDR9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maddr9::W`](W) writer structure"]
impl crate::Writable for MADDR9_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MADDR9 to value 0"]
impl crate::Resettable for MADDR9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
