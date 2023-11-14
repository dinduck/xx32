#[doc = "Register `CTLR` reader"]
pub type R = crate::R<CTLR_SPEC>;
#[doc = "Register `CTLR` writer"]
pub type W = crate::W<CTLR_SPEC>;
#[doc = "Field `T` reader - 7-bit counter (MSB to LSB)"]
pub type T_R = crate::FieldReader;
#[doc = "Field `T` writer - 7-bit counter (MSB to LSB)"]
pub type T_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `WDGA` reader - Activation bit"]
pub type WDGA_R = crate::BitReader;
#[doc = "Field `WDGA` writer - Activation bit"]
pub type WDGA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - 7-bit counter (MSB to LSB)"]
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Activation bit"]
    #[inline(always)]
    pub fn wdga(&self) -> WDGA_R {
        WDGA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit counter (MSB to LSB)"]
    #[inline(always)]
    #[must_use]
    pub fn t(&mut self) -> T_W<CTLR_SPEC, 0> {
        T_W::new(self)
    }
    #[doc = "Bit 7 - Activation bit"]
    #[inline(always)]
    #[must_use]
    pub fn wdga(&mut self) -> WDGA_W<CTLR_SPEC, 7> {
        WDGA_W::new(self)
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
#[doc = "Control register (WWDG_CR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTLR_SPEC;
impl crate::RegisterSpec for CTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlr::R`](R) reader structure"]
impl crate::Readable for CTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctlr::W`](W) writer structure"]
impl crate::Writable for CTLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTLR to value 0x7f"]
impl crate::Resettable for CTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
