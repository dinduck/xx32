#[doc = "Register `CR0` reader"]
pub type R = crate::R<CR0_SPEC>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<CR0_SPEC>;
#[doc = "Field `RB_DVP_ENABLE` reader - DVP enable"]
pub type RB_DVP_ENABLE_R = crate::BitReader;
#[doc = "Field `RB_DVP_ENABLE` writer - DVP enable"]
pub type RB_DVP_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_DVP_V_POLAR` reader - DVP VSYNC polarity control"]
pub type RB_DVP_V_POLAR_R = crate::BitReader;
#[doc = "Field `RB_DVP_V_POLAR` writer - DVP VSYNC polarity control"]
pub type RB_DVP_V_POLAR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_DVP_H_POLAR` reader - DVP HSYNC polarity control"]
pub type RB_DVP_H_POLAR_R = crate::BitReader;
#[doc = "Field `RB_DVP_H_POLAR` writer - DVP HSYNC polarity control"]
pub type RB_DVP_H_POLAR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_DVP_P_POLAR` reader - DVP PCLK polarity control"]
pub type RB_DVP_P_POLAR_R = crate::BitReader;
#[doc = "Field `RB_DVP_P_POLAR` writer - DVP PCLK polarity control"]
pub type RB_DVP_P_POLAR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_DVP_MSK_DAT_MOD` reader - DVP data mode"]
pub type RB_DVP_MSK_DAT_MOD_R = crate::FieldReader;
#[doc = "Field `RB_DVP_MSK_DAT_MOD` writer - DVP data mode"]
pub type RB_DVP_MSK_DAT_MOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `RB_DVP_JPEG` reader - DVP JPEG mode"]
pub type RB_DVP_JPEG_R = crate::BitReader;
#[doc = "Field `RB_DVP_JPEG` writer - DVP JPEG mode"]
pub type RB_DVP_JPEG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - DVP enable"]
    #[inline(always)]
    pub fn rb_dvp_enable(&self) -> RB_DVP_ENABLE_R {
        RB_DVP_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DVP VSYNC polarity control"]
    #[inline(always)]
    pub fn rb_dvp_v_polar(&self) -> RB_DVP_V_POLAR_R {
        RB_DVP_V_POLAR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DVP HSYNC polarity control"]
    #[inline(always)]
    pub fn rb_dvp_h_polar(&self) -> RB_DVP_H_POLAR_R {
        RB_DVP_H_POLAR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DVP PCLK polarity control"]
    #[inline(always)]
    pub fn rb_dvp_p_polar(&self) -> RB_DVP_P_POLAR_R {
        RB_DVP_P_POLAR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - DVP data mode"]
    #[inline(always)]
    pub fn rb_dvp_msk_dat_mod(&self) -> RB_DVP_MSK_DAT_MOD_R {
        RB_DVP_MSK_DAT_MOD_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - DVP JPEG mode"]
    #[inline(always)]
    pub fn rb_dvp_jpeg(&self) -> RB_DVP_JPEG_R {
        RB_DVP_JPEG_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DVP enable"]
    #[inline(always)]
    #[must_use]
    pub fn rb_dvp_enable(&mut self) -> RB_DVP_ENABLE_W<CR0_SPEC, 0> {
        RB_DVP_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - DVP VSYNC polarity control"]
    #[inline(always)]
    #[must_use]
    pub fn rb_dvp_v_polar(&mut self) -> RB_DVP_V_POLAR_W<CR0_SPEC, 1> {
        RB_DVP_V_POLAR_W::new(self)
    }
    #[doc = "Bit 2 - DVP HSYNC polarity control"]
    #[inline(always)]
    #[must_use]
    pub fn rb_dvp_h_polar(&mut self) -> RB_DVP_H_POLAR_W<CR0_SPEC, 2> {
        RB_DVP_H_POLAR_W::new(self)
    }
    #[doc = "Bit 3 - DVP PCLK polarity control"]
    #[inline(always)]
    #[must_use]
    pub fn rb_dvp_p_polar(&mut self) -> RB_DVP_P_POLAR_W<CR0_SPEC, 3> {
        RB_DVP_P_POLAR_W::new(self)
    }
    #[doc = "Bits 4:5 - DVP data mode"]
    #[inline(always)]
    #[must_use]
    pub fn rb_dvp_msk_dat_mod(&mut self) -> RB_DVP_MSK_DAT_MOD_W<CR0_SPEC, 4> {
        RB_DVP_MSK_DAT_MOD_W::new(self)
    }
    #[doc = "Bit 6 - DVP JPEG mode"]
    #[inline(always)]
    #[must_use]
    pub fn rb_dvp_jpeg(&mut self) -> RB_DVP_JPEG_W<CR0_SPEC, 6> {
        RB_DVP_JPEG_W::new(self)
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
#[doc = "Digital Video control register (DVP_CR0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR0_SPEC;
impl crate::RegisterSpec for CR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for CR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for CR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for CR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
