#[doc = "Register `RTENR` reader"]
pub type R = crate::R<RTENR_SPEC>;
#[doc = "Register `RTENR` writer"]
pub type W = crate::W<RTENR_SPEC>;
#[doc = "Field `TR0` reader - Rising trigger event configuration of line 0"]
pub type TR0_R = crate::BitReader;
#[doc = "Field `TR0` writer - Rising trigger event configuration of line 0"]
pub type TR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TR1` reader - Rising trigger event configuration of line 1"]
pub type TR1_R = crate::BitReader;
#[doc = "Field `TR1` writer - Rising trigger event configuration of line 1"]
pub type TR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TR2` reader - Rising trigger event configuration of line 2"]
pub type TR2_R = crate::BitReader;
#[doc = "Field `TR2` writer - Rising trigger event configuration of line 2"]
pub type TR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TR3` reader - Rising trigger event configuration of line 3"]
pub type TR3_R = crate::BitReader;
#[doc = "Field `TR3` writer - Rising trigger event configuration of line 3"]
pub type TR3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TR4` reader - Rising trigger event configuration of line 4"]
pub type TR4_R = crate::BitReader;
#[doc = "Field `TR4` writer - Rising trigger event configuration of line 4"]
pub type TR4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TR5` reader - Rising trigger event configuration of line 5"]
pub type TR5_R = crate::BitReader;
#[doc = "Field `TR5` writer - Rising trigger event configuration of line 5"]
pub type TR5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TR6` reader - Rising trigger event configuration of line 6"]
pub type TR6_R = crate::BitReader;
#[doc = "Field `TR6` writer - Rising trigger event configuration of line 6"]
pub type TR6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TR7` reader - Rising trigger event configuration of line 7"]
pub type TR7_R = crate::BitReader;
#[doc = "Field `TR7` writer - Rising trigger event configuration of line 7"]
pub type TR7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TR8` reader - Rising trigger event configuration of line 8"]
pub type TR8_R = crate::BitReader;
#[doc = "Field `TR8` writer - Rising trigger event configuration of line 8"]
pub type TR8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TR9` reader - Rising trigger event configuration of line 9"]
pub type TR9_R = crate::BitReader;
#[doc = "Field `TR9` writer - Rising trigger event configuration of line 9"]
pub type TR9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TR10` reader - Rising trigger event configuration of line 10"]
pub type TR10_R = crate::BitReader;
#[doc = "Field `TR10` writer - Rising trigger event configuration of line 10"]
pub type TR10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TR11` reader - Rising trigger event configuration of line 11"]
pub type TR11_R = crate::BitReader;
#[doc = "Field `TR11` writer - Rising trigger event configuration of line 11"]
pub type TR11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TR12` reader - Rising trigger event configuration of line 12"]
pub type TR12_R = crate::BitReader;
#[doc = "Field `TR12` writer - Rising trigger event configuration of line 12"]
pub type TR12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TR13` reader - Rising trigger event configuration of line 13"]
pub type TR13_R = crate::BitReader;
#[doc = "Field `TR13` writer - Rising trigger event configuration of line 13"]
pub type TR13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TR14` reader - Rising trigger event configuration of line 14"]
pub type TR14_R = crate::BitReader;
#[doc = "Field `TR14` writer - Rising trigger event configuration of line 14"]
pub type TR14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TR15` reader - Rising trigger event configuration of line 15"]
pub type TR15_R = crate::BitReader;
#[doc = "Field `TR15` writer - Rising trigger event configuration of line 15"]
pub type TR15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TR16` reader - Rising trigger event configuration of line 16"]
pub type TR16_R = crate::BitReader;
#[doc = "Field `TR16` writer - Rising trigger event configuration of line 16"]
pub type TR16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TR17` reader - Rising trigger event configuration of line 17"]
pub type TR17_R = crate::BitReader;
#[doc = "Field `TR17` writer - Rising trigger event configuration of line 17"]
pub type TR17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TR18` reader - Rising trigger event configuration of line 18"]
pub type TR18_R = crate::BitReader;
#[doc = "Field `TR18` writer - Rising trigger event configuration of line 18"]
pub type TR18_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TR19` reader - Rising trigger event configuration of line 19"]
pub type TR19_R = crate::BitReader;
#[doc = "Field `TR19` writer - Rising trigger event configuration of line 19"]
pub type TR19_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Rising trigger event configuration of line 0"]
    #[inline(always)]
    pub fn tr0(&self) -> TR0_R {
        TR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration of line 1"]
    #[inline(always)]
    pub fn tr1(&self) -> TR1_R {
        TR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising trigger event configuration of line 2"]
    #[inline(always)]
    pub fn tr2(&self) -> TR2_R {
        TR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising trigger event configuration of line 3"]
    #[inline(always)]
    pub fn tr3(&self) -> TR3_R {
        TR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising trigger event configuration of line 4"]
    #[inline(always)]
    pub fn tr4(&self) -> TR4_R {
        TR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising trigger event configuration of line 5"]
    #[inline(always)]
    pub fn tr5(&self) -> TR5_R {
        TR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising trigger event configuration of line 6"]
    #[inline(always)]
    pub fn tr6(&self) -> TR6_R {
        TR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising trigger event configuration of line 7"]
    #[inline(always)]
    pub fn tr7(&self) -> TR7_R {
        TR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising trigger event configuration of line 8"]
    #[inline(always)]
    pub fn tr8(&self) -> TR8_R {
        TR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising trigger event configuration of line 9"]
    #[inline(always)]
    pub fn tr9(&self) -> TR9_R {
        TR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rising trigger event configuration of line 10"]
    #[inline(always)]
    pub fn tr10(&self) -> TR10_R {
        TR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rising trigger event configuration of line 11"]
    #[inline(always)]
    pub fn tr11(&self) -> TR11_R {
        TR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising trigger event configuration of line 12"]
    #[inline(always)]
    pub fn tr12(&self) -> TR12_R {
        TR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising trigger event configuration of line 13"]
    #[inline(always)]
    pub fn tr13(&self) -> TR13_R {
        TR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rising trigger event configuration of line 14"]
    #[inline(always)]
    pub fn tr14(&self) -> TR14_R {
        TR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rising trigger event configuration of line 15"]
    #[inline(always)]
    pub fn tr15(&self) -> TR15_R {
        TR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rising trigger event configuration of line 16"]
    #[inline(always)]
    pub fn tr16(&self) -> TR16_R {
        TR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rising trigger event configuration of line 17"]
    #[inline(always)]
    pub fn tr17(&self) -> TR17_R {
        TR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Rising trigger event configuration of line 18"]
    #[inline(always)]
    pub fn tr18(&self) -> TR18_R {
        TR18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Rising trigger event configuration of line 19"]
    #[inline(always)]
    pub fn tr19(&self) -> TR19_R {
        TR19_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising trigger event configuration of line 0"]
    #[inline(always)]
    #[must_use]
    pub fn tr0(&mut self) -> TR0_W<RTENR_SPEC, 0> {
        TR0_W::new(self)
    }
    #[doc = "Bit 1 - Rising trigger event configuration of line 1"]
    #[inline(always)]
    #[must_use]
    pub fn tr1(&mut self) -> TR1_W<RTENR_SPEC, 1> {
        TR1_W::new(self)
    }
    #[doc = "Bit 2 - Rising trigger event configuration of line 2"]
    #[inline(always)]
    #[must_use]
    pub fn tr2(&mut self) -> TR2_W<RTENR_SPEC, 2> {
        TR2_W::new(self)
    }
    #[doc = "Bit 3 - Rising trigger event configuration of line 3"]
    #[inline(always)]
    #[must_use]
    pub fn tr3(&mut self) -> TR3_W<RTENR_SPEC, 3> {
        TR3_W::new(self)
    }
    #[doc = "Bit 4 - Rising trigger event configuration of line 4"]
    #[inline(always)]
    #[must_use]
    pub fn tr4(&mut self) -> TR4_W<RTENR_SPEC, 4> {
        TR4_W::new(self)
    }
    #[doc = "Bit 5 - Rising trigger event configuration of line 5"]
    #[inline(always)]
    #[must_use]
    pub fn tr5(&mut self) -> TR5_W<RTENR_SPEC, 5> {
        TR5_W::new(self)
    }
    #[doc = "Bit 6 - Rising trigger event configuration of line 6"]
    #[inline(always)]
    #[must_use]
    pub fn tr6(&mut self) -> TR6_W<RTENR_SPEC, 6> {
        TR6_W::new(self)
    }
    #[doc = "Bit 7 - Rising trigger event configuration of line 7"]
    #[inline(always)]
    #[must_use]
    pub fn tr7(&mut self) -> TR7_W<RTENR_SPEC, 7> {
        TR7_W::new(self)
    }
    #[doc = "Bit 8 - Rising trigger event configuration of line 8"]
    #[inline(always)]
    #[must_use]
    pub fn tr8(&mut self) -> TR8_W<RTENR_SPEC, 8> {
        TR8_W::new(self)
    }
    #[doc = "Bit 9 - Rising trigger event configuration of line 9"]
    #[inline(always)]
    #[must_use]
    pub fn tr9(&mut self) -> TR9_W<RTENR_SPEC, 9> {
        TR9_W::new(self)
    }
    #[doc = "Bit 10 - Rising trigger event configuration of line 10"]
    #[inline(always)]
    #[must_use]
    pub fn tr10(&mut self) -> TR10_W<RTENR_SPEC, 10> {
        TR10_W::new(self)
    }
    #[doc = "Bit 11 - Rising trigger event configuration of line 11"]
    #[inline(always)]
    #[must_use]
    pub fn tr11(&mut self) -> TR11_W<RTENR_SPEC, 11> {
        TR11_W::new(self)
    }
    #[doc = "Bit 12 - Rising trigger event configuration of line 12"]
    #[inline(always)]
    #[must_use]
    pub fn tr12(&mut self) -> TR12_W<RTENR_SPEC, 12> {
        TR12_W::new(self)
    }
    #[doc = "Bit 13 - Rising trigger event configuration of line 13"]
    #[inline(always)]
    #[must_use]
    pub fn tr13(&mut self) -> TR13_W<RTENR_SPEC, 13> {
        TR13_W::new(self)
    }
    #[doc = "Bit 14 - Rising trigger event configuration of line 14"]
    #[inline(always)]
    #[must_use]
    pub fn tr14(&mut self) -> TR14_W<RTENR_SPEC, 14> {
        TR14_W::new(self)
    }
    #[doc = "Bit 15 - Rising trigger event configuration of line 15"]
    #[inline(always)]
    #[must_use]
    pub fn tr15(&mut self) -> TR15_W<RTENR_SPEC, 15> {
        TR15_W::new(self)
    }
    #[doc = "Bit 16 - Rising trigger event configuration of line 16"]
    #[inline(always)]
    #[must_use]
    pub fn tr16(&mut self) -> TR16_W<RTENR_SPEC, 16> {
        TR16_W::new(self)
    }
    #[doc = "Bit 17 - Rising trigger event configuration of line 17"]
    #[inline(always)]
    #[must_use]
    pub fn tr17(&mut self) -> TR17_W<RTENR_SPEC, 17> {
        TR17_W::new(self)
    }
    #[doc = "Bit 18 - Rising trigger event configuration of line 18"]
    #[inline(always)]
    #[must_use]
    pub fn tr18(&mut self) -> TR18_W<RTENR_SPEC, 18> {
        TR18_W::new(self)
    }
    #[doc = "Bit 19 - Rising trigger event configuration of line 19"]
    #[inline(always)]
    #[must_use]
    pub fn tr19(&mut self) -> TR19_W<RTENR_SPEC, 19> {
        TR19_W::new(self)
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
#[doc = "Rising Trigger selection register (EXTI_RTENR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTENR_SPEC;
impl crate::RegisterSpec for RTENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtenr::R`](R) reader structure"]
impl crate::Readable for RTENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtenr::W`](W) writer structure"]
impl crate::Writable for RTENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTENR to value 0"]
impl crate::Resettable for RTENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
