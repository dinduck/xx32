#[doc = "Register `IOFR3` reader"]
pub type R = crate::R<IOFR3_SPEC>;
#[doc = "Register `IOFR3` writer"]
pub type W = crate::W<IOFR3_SPEC>;
#[doc = "Field `JOFFSET3` reader - Data offset for injected channel x"]
pub type JOFFSET3_R = crate::FieldReader<u16>;
#[doc = "Field `JOFFSET3` writer - Data offset for injected channel x"]
pub type JOFFSET3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn joffset3(&self) -> JOFFSET3_R {
        JOFFSET3_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    #[must_use]
    pub fn joffset3(&mut self) -> JOFFSET3_W<IOFR3_SPEC, 0> {
        JOFFSET3_W::new(self)
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
#[doc = "injected channel data offset register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iofr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iofr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOFR3_SPEC;
impl crate::RegisterSpec for IOFR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iofr3::R`](R) reader structure"]
impl crate::Readable for IOFR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iofr3::W`](W) writer structure"]
impl crate::Writable for IOFR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOFR3 to value 0"]
impl crate::Resettable for IOFR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
