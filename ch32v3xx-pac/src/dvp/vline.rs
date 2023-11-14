#[doc = "Register `VLINE` reader"]
pub type R = crate::R<VLINE_SPEC>;
#[doc = "Register `VLINE` writer"]
pub type W = crate::W<VLINE_SPEC>;
#[doc = "Field `RB_DVP_VLINE` reader - Crop the number of rows captured by window"]
pub type RB_DVP_VLINE_R = crate::FieldReader<u16>;
#[doc = "Field `RB_DVP_VLINE` writer - Crop the number of rows captured by window"]
pub type RB_DVP_VLINE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Crop the number of rows captured by window"]
    #[inline(always)]
    pub fn rb_dvp_vline(&self) -> RB_DVP_VLINE_R {
        RB_DVP_VLINE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Crop the number of rows captured by window"]
    #[inline(always)]
    #[must_use]
    pub fn rb_dvp_vline(&mut self) -> RB_DVP_VLINE_W<VLINE_SPEC, 0> {
        RB_DVP_VLINE_W::new(self)
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
#[doc = "Digital Video Vertical line count register (DVP_VLINE)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vline::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vline::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VLINE_SPEC;
impl crate::RegisterSpec for VLINE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`vline::R`](R) reader structure"]
impl crate::Readable for VLINE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vline::W`](W) writer structure"]
impl crate::Writable for VLINE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VLINE to value 0"]
impl crate::Resettable for VLINE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
