#[doc = "Register `HSCR` reader"]
pub type R = crate::R<HSCR_SPEC>;
#[doc = "Register `HSCR` writer"]
pub type W = crate::W<HSCR_SPEC>;
#[doc = "Field `HSRXEN` reader - High speed mode read enable"]
pub type HSRXEN_R = crate::BitReader;
#[doc = "Field `HSRXEN` writer - High speed mode read enable"]
pub type HSRXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - High speed mode read enable"]
    #[inline(always)]
    pub fn hsrxen(&self) -> HSRXEN_R {
        HSRXEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - High speed mode read enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsrxen(&mut self) -> HSRXEN_W<HSCR_SPEC, 0> {
        HSRXEN_W::new(self)
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
#[doc = "high speed control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSCR_SPEC;
impl crate::RegisterSpec for HSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hscr::R`](R) reader structure"]
impl crate::Readable for HSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hscr::W`](W) writer structure"]
impl crate::Writable for HSCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSCR to value 0"]
impl crate::Resettable for HSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
