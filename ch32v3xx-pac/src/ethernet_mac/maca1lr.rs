#[doc = "Register `MACA1LR` reader"]
pub type R = crate::R<MACA1LR_SPEC>;
#[doc = "Register `MACA1LR` writer"]
pub type W = crate::W<MACA1LR_SPEC>;
#[doc = "Field `MACA1L` reader - MAC address1 low"]
pub type MACA1L_R = crate::FieldReader<u32>;
#[doc = "Field `MACA1L` writer - MAC address1 low"]
pub type MACA1L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC address1 low"]
    #[inline(always)]
    pub fn maca1l(&self) -> MACA1L_R {
        MACA1L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC address1 low"]
    #[inline(always)]
    #[must_use]
    pub fn maca1l(&mut self) -> MACA1L_W<MACA1LR_SPEC, 0> {
        MACA1L_W::new(self)
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
#[doc = "Ethernet MAC address1 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca1lr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca1lr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA1LR_SPEC;
impl crate::RegisterSpec for MACA1LR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca1lr::R`](R) reader structure"]
impl crate::Readable for MACA1LR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maca1lr::W`](W) writer structure"]
impl crate::Writable for MACA1LR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACA1LR to value 0xffff_ffff"]
impl crate::Resettable for MACA1LR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
