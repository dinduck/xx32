#[doc = "Register `IOFR2` reader"]
pub type R = crate::R<IOFR2_SPEC>;
#[doc = "Register `IOFR2` writer"]
pub type W = crate::W<IOFR2_SPEC>;
#[doc = "Field `JOFFSET2` reader - Data offset for injected channel x"]
pub type JOFFSET2_R = crate::FieldReader<u16>;
#[doc = "Field `JOFFSET2` writer - Data offset for injected channel x"]
pub type JOFFSET2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn joffset2(&self) -> JOFFSET2_R {
        JOFFSET2_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    #[must_use]
    pub fn joffset2(&mut self) -> JOFFSET2_W<IOFR2_SPEC, 0> {
        JOFFSET2_W::new(self)
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
#[doc = "injected channel data offset register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iofr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iofr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOFR2_SPEC;
impl crate::RegisterSpec for IOFR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iofr2::R`](R) reader structure"]
impl crate::Readable for IOFR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iofr2::W`](W) writer structure"]
impl crate::Writable for IOFR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOFR2 to value 0"]
impl crate::Resettable for IOFR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
