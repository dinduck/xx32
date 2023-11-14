#[doc = "Register `FCTLR` reader"]
pub type R = crate::R<FCTLR_SPEC>;
#[doc = "Register `FCTLR` writer"]
pub type W = crate::W<FCTLR_SPEC>;
#[doc = "Field `FINIT` reader - Filter init mode"]
pub type FINIT_R = crate::BitReader;
#[doc = "Field `FINIT` writer - Filter init mode"]
pub type FINIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN2SB` reader - CAN2 start bank"]
pub type CAN2SB_R = crate::FieldReader;
#[doc = "Field `CAN2SB` writer - CAN2 start bank"]
pub type CAN2SB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bit 0 - Filter init mode"]
    #[inline(always)]
    pub fn finit(&self) -> FINIT_R {
        FINIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:13 - CAN2 start bank"]
    #[inline(always)]
    pub fn can2sb(&self) -> CAN2SB_R {
        CAN2SB_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Filter init mode"]
    #[inline(always)]
    #[must_use]
    pub fn finit(&mut self) -> FINIT_W<FCTLR_SPEC, 0> {
        FINIT_W::new(self)
    }
    #[doc = "Bits 8:13 - CAN2 start bank"]
    #[inline(always)]
    #[must_use]
    pub fn can2sb(&mut self) -> CAN2SB_W<FCTLR_SPEC, 8> {
        CAN2SB_W::new(self)
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
#[doc = "CAN filter master register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCTLR_SPEC;
impl crate::RegisterSpec for FCTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fctlr::R`](R) reader structure"]
impl crate::Readable for FCTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fctlr::W`](W) writer structure"]
impl crate::Writable for FCTLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCTLR to value 0x2a1c_0e01"]
impl crate::Resettable for FCTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2a1c_0e01;
}
