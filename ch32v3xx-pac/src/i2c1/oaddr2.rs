#[doc = "Register `OADDR2` reader"]
pub type R = crate::R<OADDR2_SPEC>;
#[doc = "Register `OADDR2` writer"]
pub type W = crate::W<OADDR2_SPEC>;
#[doc = "Field `ENDUAL` reader - Dual addressing mode enable"]
pub type ENDUAL_R = crate::BitReader;
#[doc = "Field `ENDUAL` writer - Dual addressing mode enable"]
pub type ENDUAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADD2` reader - Interface address"]
pub type ADD2_R = crate::FieldReader;
#[doc = "Field `ADD2` writer - Interface address"]
pub type ADD2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bit 0 - Dual addressing mode enable"]
    #[inline(always)]
    pub fn endual(&self) -> ENDUAL_R {
        ENDUAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn add2(&self) -> ADD2_R {
        ADD2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Dual addressing mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn endual(&mut self) -> ENDUAL_W<OADDR2_SPEC, 0> {
        ENDUAL_W::new(self)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    #[must_use]
    pub fn add2(&mut self) -> ADD2_W<OADDR2_SPEC, 1> {
        ADD2_W::new(self)
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
#[doc = "Own address register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oaddr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oaddr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OADDR2_SPEC;
impl crate::RegisterSpec for OADDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oaddr2::R`](R) reader structure"]
impl crate::Readable for OADDR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oaddr2::W`](W) writer structure"]
impl crate::Writable for OADDR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OADDR2 to value 0"]
impl crate::Resettable for OADDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
