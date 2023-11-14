#[doc = "Register `IOFR4` reader"]
pub type R = crate::R<IOFR4_SPEC>;
#[doc = "Register `IOFR4` writer"]
pub type W = crate::W<IOFR4_SPEC>;
#[doc = "Field `JOFFSET4` reader - Data offset for injected channel x"]
pub type JOFFSET4_R = crate::FieldReader<u16>;
#[doc = "Field `JOFFSET4` writer - Data offset for injected channel x"]
pub type JOFFSET4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn joffset4(&self) -> JOFFSET4_R {
        JOFFSET4_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    #[must_use]
    pub fn joffset4(&mut self) -> JOFFSET4_W<IOFR4_SPEC, 0> {
        JOFFSET4_W::new(self)
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
#[doc = "injected channel data offset register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iofr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iofr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOFR4_SPEC;
impl crate::RegisterSpec for IOFR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iofr4::R`](R) reader structure"]
impl crate::Readable for IOFR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iofr4::W`](W) writer structure"]
impl crate::Writable for IOFR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOFR4 to value 0"]
impl crate::Resettable for IOFR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
