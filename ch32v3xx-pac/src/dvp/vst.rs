#[doc = "Register `VST` reader"]
pub type R = crate::R<VST_SPEC>;
#[doc = "Register `VST` writer"]
pub type W = crate::W<VST_SPEC>;
#[doc = "Field `RB_DVP_VST` reader - The number of lines captured by the image"]
pub type RB_DVP_VST_R = crate::FieldReader<u16>;
#[doc = "Field `RB_DVP_VST` writer - The number of lines captured by the image"]
pub type RB_DVP_VST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - The number of lines captured by the image"]
    #[inline(always)]
    pub fn rb_dvp_vst(&self) -> RB_DVP_VST_R {
        RB_DVP_VST_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - The number of lines captured by the image"]
    #[inline(always)]
    #[must_use]
    pub fn rb_dvp_vst(&mut self) -> RB_DVP_VST_W<VST_SPEC, 0> {
        RB_DVP_VST_W::new(self)
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
#[doc = "Digital Video line number register (DVP_VST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VST_SPEC;
impl crate::RegisterSpec for VST_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`vst::R`](R) reader structure"]
impl crate::Readable for VST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vst::W`](W) writer structure"]
impl crate::Writable for VST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VST to value 0"]
impl crate::Resettable for VST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
