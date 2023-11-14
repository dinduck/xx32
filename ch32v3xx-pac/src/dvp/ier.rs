#[doc = "Register `IER` reader"]
pub type R = crate::R<IER_SPEC>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `RB_DVP_IE_STR_FRM` reader - DVP frame start interrupt enable"]
pub type RB_DVP_IE_STR_FRM_R = crate::BitReader;
#[doc = "Field `RB_DVP_IE_STR_FRM` writer - DVP frame start interrupt enable"]
pub type RB_DVP_IE_STR_FRM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_DVP_IE_ROW_DONE` reader - DVP row received done interrupt enable"]
pub type RB_DVP_IE_ROW_DONE_R = crate::BitReader;
#[doc = "Field `RB_DVP_IE_ROW_DONE` writer - DVP row received done interrupt enable"]
pub type RB_DVP_IE_ROW_DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_DVP_IE_FRM_DONE` reader - DVP frame received done interrupt enable"]
pub type RB_DVP_IE_FRM_DONE_R = crate::BitReader;
#[doc = "Field `RB_DVP_IE_FRM_DONE` writer - DVP frame received done interrupt enable"]
pub type RB_DVP_IE_FRM_DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_DVP_IE_FIFO_OV` reader - DVP receive fifo overflow interrupt enable"]
pub type RB_DVP_IE_FIFO_OV_R = crate::BitReader;
#[doc = "Field `RB_DVP_IE_FIFO_OV` writer - DVP receive fifo overflow interrupt enable"]
pub type RB_DVP_IE_FIFO_OV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_DVP_IE_STP_FRM` reader - DVP frame stop interrupt enable"]
pub type RB_DVP_IE_STP_FRM_R = crate::BitReader;
#[doc = "Field `RB_DVP_IE_STP_FRM` writer - DVP frame stop interrupt enable"]
pub type RB_DVP_IE_STP_FRM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - DVP frame start interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_str_frm(&self) -> RB_DVP_IE_STR_FRM_R {
        RB_DVP_IE_STR_FRM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DVP row received done interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_row_done(&self) -> RB_DVP_IE_ROW_DONE_R {
        RB_DVP_IE_ROW_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DVP frame received done interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_frm_done(&self) -> RB_DVP_IE_FRM_DONE_R {
        RB_DVP_IE_FRM_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DVP receive fifo overflow interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_fifo_ov(&self) -> RB_DVP_IE_FIFO_OV_R {
        RB_DVP_IE_FIFO_OV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DVP frame stop interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_stp_frm(&self) -> RB_DVP_IE_STP_FRM_R {
        RB_DVP_IE_STP_FRM_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DVP frame start interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rb_dvp_ie_str_frm(&mut self) -> RB_DVP_IE_STR_FRM_W<IER_SPEC, 0> {
        RB_DVP_IE_STR_FRM_W::new(self)
    }
    #[doc = "Bit 1 - DVP row received done interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rb_dvp_ie_row_done(&mut self) -> RB_DVP_IE_ROW_DONE_W<IER_SPEC, 1> {
        RB_DVP_IE_ROW_DONE_W::new(self)
    }
    #[doc = "Bit 2 - DVP frame received done interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rb_dvp_ie_frm_done(&mut self) -> RB_DVP_IE_FRM_DONE_W<IER_SPEC, 2> {
        RB_DVP_IE_FRM_DONE_W::new(self)
    }
    #[doc = "Bit 3 - DVP receive fifo overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rb_dvp_ie_fifo_ov(&mut self) -> RB_DVP_IE_FIFO_OV_W<IER_SPEC, 3> {
        RB_DVP_IE_FIFO_OV_W::new(self)
    }
    #[doc = "Bit 4 - DVP frame stop interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rb_dvp_ie_stp_frm(&mut self) -> RB_DVP_IE_STP_FRM_W<IER_SPEC, 4> {
        RB_DVP_IE_STP_FRM_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Digital Video Interrupt register (DVP_IER)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
