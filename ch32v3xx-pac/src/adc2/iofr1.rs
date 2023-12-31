#[doc = "Register `IOFR1` reader"]
pub type R = crate::R<IOFR1_SPEC>;
#[doc = "Register `IOFR1` writer"]
pub type W = crate::W<IOFR1_SPEC>;
#[doc = "Field `JOFFSET1` reader - Data offset for injected channel x"]
pub type JOFFSET1_R = crate::FieldReader<u16>;
#[doc = "Field `JOFFSET1` writer - Data offset for injected channel x"]
pub type JOFFSET1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn joffset1(&self) -> JOFFSET1_R {
        JOFFSET1_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    #[must_use]
    pub fn joffset1(&mut self) -> JOFFSET1_W<IOFR1_SPEC, 0> {
        JOFFSET1_W::new(self)
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
#[doc = "injected channel data offset register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iofr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iofr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOFR1_SPEC;
impl crate::RegisterSpec for IOFR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iofr1::R`](R) reader structure"]
impl crate::Readable for IOFR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iofr1::W`](W) writer structure"]
impl crate::Writable for IOFR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOFR1 to value 0"]
impl crate::Resettable for IOFR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
