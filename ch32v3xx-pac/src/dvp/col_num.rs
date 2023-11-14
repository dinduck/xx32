#[doc = "Register `COL_NUM` reader"]
pub type R = crate::R<COL_NUM_SPEC>;
#[doc = "Register `COL_NUM` writer"]
pub type W = crate::W<COL_NUM_SPEC>;
#[doc = "Field `RB_DVP_COL_NUM` reader - Number of PCLK cycles for row data"]
pub type RB_DVP_COL_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `RB_DVP_COL_NUM` writer - Number of PCLK cycles for row data"]
pub type RB_DVP_COL_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of PCLK cycles for row data"]
    #[inline(always)]
    pub fn rb_dvp_col_num(&self) -> RB_DVP_COL_NUM_R {
        RB_DVP_COL_NUM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of PCLK cycles for row data"]
    #[inline(always)]
    #[must_use]
    pub fn rb_dvp_col_num(&mut self) -> RB_DVP_COL_NUM_W<COL_NUM_SPEC, 0> {
        RB_DVP_COL_NUM_W::new(self)
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
#[doc = "Image column number configuration register (DVP_COL_NUM)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`col_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`col_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COL_NUM_SPEC;
impl crate::RegisterSpec for COL_NUM_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`col_num::R`](R) reader structure"]
impl crate::Readable for COL_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`col_num::W`](W) writer structure"]
impl crate::Writable for COL_NUM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COL_NUM to value 0"]
impl crate::Resettable for COL_NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
