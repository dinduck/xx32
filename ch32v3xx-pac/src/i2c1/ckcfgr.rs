#[doc = "Register `CKCFGR` reader"]
pub type R = crate::R<CKCFGR_SPEC>;
#[doc = "Register `CKCFGR` writer"]
pub type W = crate::W<CKCFGR_SPEC>;
#[doc = "Field `CCR` reader - Clock control register in Fast/Standard mode (Master mode)"]
pub type CCR_R = crate::FieldReader<u16>;
#[doc = "Field `CCR` writer - Clock control register in Fast/Standard mode (Master mode)"]
pub type CCR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `DUTY` reader - Fast mode duty cycle"]
pub type DUTY_R = crate::BitReader;
#[doc = "Field `DUTY` writer - Fast mode duty cycle"]
pub type DUTY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `F_S` reader - I2C master mode selection"]
pub type F_S_R = crate::BitReader;
#[doc = "Field `F_S` writer - I2C master mode selection"]
pub type F_S_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:11 - Clock control register in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    pub fn ccr(&self) -> CCR_R {
        CCR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - Fast mode duty cycle"]
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C master mode selection"]
    #[inline(always)]
    pub fn f_s(&self) -> F_S_R {
        F_S_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Clock control register in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn ccr(&mut self) -> CCR_W<CKCFGR_SPEC, 0> {
        CCR_W::new(self)
    }
    #[doc = "Bit 14 - Fast mode duty cycle"]
    #[inline(always)]
    #[must_use]
    pub fn duty(&mut self) -> DUTY_W<CKCFGR_SPEC, 14> {
        DUTY_W::new(self)
    }
    #[doc = "Bit 15 - I2C master mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn f_s(&mut self) -> F_S_W<CKCFGR_SPEC, 15> {
        F_S_W::new(self)
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
#[doc = "Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CKCFGR_SPEC;
impl crate::RegisterSpec for CKCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckcfgr::R`](R) reader structure"]
impl crate::Readable for CKCFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ckcfgr::W`](W) writer structure"]
impl crate::Writable for CKCFGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKCFGR to value 0"]
impl crate::Resettable for CKCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
