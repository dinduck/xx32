#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<CFGR1_SPEC>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<CFGR1_SPEC>;
#[doc = "Field `DEG_IWDG` reader - DEG_IWDG"]
pub type DEG_IWDG_R = crate::BitReader;
#[doc = "Field `DEG_IWDG` writer - DEG_IWDG"]
pub type DEG_IWDG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEG_WWDG` reader - DEG_WWDG"]
pub type DEG_WWDG_R = crate::BitReader;
#[doc = "Field `DEG_WWDG` writer - DEG_WWDG"]
pub type DEG_WWDG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEG_I2C1` reader - DEG_I2C1"]
pub type DEG_I2C1_R = crate::BitReader;
#[doc = "Field `DEG_I2C1` writer - DEG_I2C1"]
pub type DEG_I2C1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEG_I2C2` reader - DEG_I2C2"]
pub type DEG_I2C2_R = crate::BitReader;
#[doc = "Field `DEG_I2C2` writer - DEG_I2C2"]
pub type DEG_I2C2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEG_TIM1` reader - DEG_TIM1"]
pub type DEG_TIM1_R = crate::BitReader;
#[doc = "Field `DEG_TIM1` writer - DEG_TIM1"]
pub type DEG_TIM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEG_TIM2` reader - DEG_TIM2"]
pub type DEG_TIM2_R = crate::BitReader;
#[doc = "Field `DEG_TIM2` writer - DEG_TIM2"]
pub type DEG_TIM2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEG_TIM3` reader - DEG_TIM3"]
pub type DEG_TIM3_R = crate::BitReader;
#[doc = "Field `DEG_TIM3` writer - DEG_TIM3"]
pub type DEG_TIM3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEG_TIM4` reader - DEG_TIM4"]
pub type DEG_TIM4_R = crate::BitReader;
#[doc = "Field `DEG_TIM4` writer - DEG_TIM4"]
pub type DEG_TIM4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - DEG_IWDG"]
    #[inline(always)]
    pub fn deg_iwdg(&self) -> DEG_IWDG_R {
        DEG_IWDG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DEG_WWDG"]
    #[inline(always)]
    pub fn deg_wwdg(&self) -> DEG_WWDG_R {
        DEG_WWDG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DEG_I2C1"]
    #[inline(always)]
    pub fn deg_i2c1(&self) -> DEG_I2C1_R {
        DEG_I2C1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DEG_I2C2"]
    #[inline(always)]
    pub fn deg_i2c2(&self) -> DEG_I2C2_R {
        DEG_I2C2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DEG_TIM1"]
    #[inline(always)]
    pub fn deg_tim1(&self) -> DEG_TIM1_R {
        DEG_TIM1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DEG_TIM2"]
    #[inline(always)]
    pub fn deg_tim2(&self) -> DEG_TIM2_R {
        DEG_TIM2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DEG_TIM3"]
    #[inline(always)]
    pub fn deg_tim3(&self) -> DEG_TIM3_R {
        DEG_TIM3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DEG_TIM4"]
    #[inline(always)]
    pub fn deg_tim4(&self) -> DEG_TIM4_R {
        DEG_TIM4_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DEG_IWDG"]
    #[inline(always)]
    #[must_use]
    pub fn deg_iwdg(&mut self) -> DEG_IWDG_W<CFGR1_SPEC, 0> {
        DEG_IWDG_W::new(self)
    }
    #[doc = "Bit 1 - DEG_WWDG"]
    #[inline(always)]
    #[must_use]
    pub fn deg_wwdg(&mut self) -> DEG_WWDG_W<CFGR1_SPEC, 1> {
        DEG_WWDG_W::new(self)
    }
    #[doc = "Bit 2 - DEG_I2C1"]
    #[inline(always)]
    #[must_use]
    pub fn deg_i2c1(&mut self) -> DEG_I2C1_W<CFGR1_SPEC, 2> {
        DEG_I2C1_W::new(self)
    }
    #[doc = "Bit 3 - DEG_I2C2"]
    #[inline(always)]
    #[must_use]
    pub fn deg_i2c2(&mut self) -> DEG_I2C2_W<CFGR1_SPEC, 3> {
        DEG_I2C2_W::new(self)
    }
    #[doc = "Bit 4 - DEG_TIM1"]
    #[inline(always)]
    #[must_use]
    pub fn deg_tim1(&mut self) -> DEG_TIM1_W<CFGR1_SPEC, 4> {
        DEG_TIM1_W::new(self)
    }
    #[doc = "Bit 5 - DEG_TIM2"]
    #[inline(always)]
    #[must_use]
    pub fn deg_tim2(&mut self) -> DEG_TIM2_W<CFGR1_SPEC, 5> {
        DEG_TIM2_W::new(self)
    }
    #[doc = "Bit 6 - DEG_TIM3"]
    #[inline(always)]
    #[must_use]
    pub fn deg_tim3(&mut self) -> DEG_TIM3_W<CFGR1_SPEC, 6> {
        DEG_TIM3_W::new(self)
    }
    #[doc = "Bit 7 - DEG_TIM4"]
    #[inline(always)]
    #[must_use]
    pub fn deg_tim4(&mut self) -> DEG_TIM4_W<CFGR1_SPEC, 7> {
        DEG_TIM4_W::new(self)
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
#[doc = "DBGMCU_CFGR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr1::R`](R) reader structure"]
impl crate::Readable for CFGR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure"]
impl crate::Writable for CFGR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
