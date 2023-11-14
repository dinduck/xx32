#[doc = "Register `ROW_NUM` reader"]
pub type R = crate::R<ROW_NUM_SPEC>;
#[doc = "Register `ROW_NUM` writer"]
pub type W = crate::W<ROW_NUM_SPEC>;
#[doc = "Field `RB_DVP_ROW_NUM` reader - The number of rows of frame image data"]
pub type RB_DVP_ROW_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `RB_DVP_ROW_NUM` writer - The number of rows of frame image data"]
pub type RB_DVP_ROW_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - The number of rows of frame image data"]
    #[inline(always)]
    pub fn rb_dvp_row_num(&self) -> RB_DVP_ROW_NUM_R {
        RB_DVP_ROW_NUM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - The number of rows of frame image data"]
    #[inline(always)]
    #[must_use]
    pub fn rb_dvp_row_num(&mut self) -> RB_DVP_ROW_NUM_W<ROW_NUM_SPEC, 0> {
        RB_DVP_ROW_NUM_W::new(self)
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
#[doc = "Image line count configuration register (DVP_ROW_NUM)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`row_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`row_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROW_NUM_SPEC;
impl crate::RegisterSpec for ROW_NUM_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`row_num::R`](R) reader structure"]
impl crate::Readable for ROW_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`row_num::W`](W) writer structure"]
impl crate::Writable for ROW_NUM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROW_NUM to value 0"]
impl crate::Resettable for ROW_NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
