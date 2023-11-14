#[doc = "Register `RLDR` reader"]
pub type R = crate::R<RLDR_SPEC>;
#[doc = "Register `RLDR` writer"]
pub type W = crate::W<RLDR_SPEC>;
#[doc = "Field `RL` reader - Watchdog counter reload value"]
pub type RL_R = crate::FieldReader<u16>;
#[doc = "Field `RL` writer - Watchdog counter reload value"]
pub type RL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Watchdog counter reload value"]
    #[inline(always)]
    pub fn rl(&self) -> RL_R {
        RL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog counter reload value"]
    #[inline(always)]
    #[must_use]
    pub fn rl(&mut self) -> RL_W<RLDR_SPEC, 0> {
        RL_W::new(self)
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
#[doc = "Reload register (IWDG_RLDR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rldr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rldr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RLDR_SPEC;
impl crate::RegisterSpec for RLDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rldr::R`](R) reader structure"]
impl crate::Readable for RLDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rldr::W`](W) writer structure"]
impl crate::Writable for RLDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RLDR to value 0x0fff"]
impl crate::Resettable for RLDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}
