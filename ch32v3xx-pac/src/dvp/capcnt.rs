#[doc = "Register `CAPCNT` reader"]
pub type R = crate::R<CAPCNT_SPEC>;
#[doc = "Register `CAPCNT` writer"]
pub type W = crate::W<CAPCNT_SPEC>;
#[doc = "Field `RB_DVP_CAPCNT` reader - Number of PCLK cycles captured by clipping window"]
pub type RB_DVP_CAPCNT_R = crate::FieldReader<u16>;
#[doc = "Field `RB_DVP_CAPCNT` writer - Number of PCLK cycles captured by clipping window"]
pub type RB_DVP_CAPCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of PCLK cycles captured by clipping window"]
    #[inline(always)]
    pub fn rb_dvp_capcnt(&self) -> RB_DVP_CAPCNT_R {
        RB_DVP_CAPCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of PCLK cycles captured by clipping window"]
    #[inline(always)]
    #[must_use]
    pub fn rb_dvp_capcnt(&mut self) -> RB_DVP_CAPCNT_W<CAPCNT_SPEC, 0> {
        RB_DVP_CAPCNT_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Digital Video Capture count register (DVP_CAPCNT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`capcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAPCNT_SPEC;
impl crate::RegisterSpec for CAPCNT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`capcnt::R`](R) reader structure"]
impl crate::Readable for CAPCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`capcnt::W`](W) writer structure"]
impl crate::Writable for CAPCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAPCNT to value 0"]
impl crate::Resettable for CAPCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
