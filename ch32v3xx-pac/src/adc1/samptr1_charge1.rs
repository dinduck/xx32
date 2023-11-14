#[doc = "Register `SAMPTR1_CHARGE1` reader"]
pub type R = crate::R<SAMPTR1_CHARGE1_SPEC>;
#[doc = "Register `SAMPTR1_CHARGE1` writer"]
pub type W = crate::W<SAMPTR1_CHARGE1_SPEC>;
#[doc = "Field `SMP10_TKCG10` reader - Channel 10 sample time selection"]
pub type SMP10_TKCG10_R = crate::FieldReader;
#[doc = "Field `SMP10_TKCG10` writer - Channel 10 sample time selection"]
pub type SMP10_TKCG10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP11_TKCG11` reader - Channel 11 sample time selection"]
pub type SMP11_TKCG11_R = crate::FieldReader;
#[doc = "Field `SMP11_TKCG11` writer - Channel 11 sample time selection"]
pub type SMP11_TKCG11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP12_TKCG12` reader - Channel 12 sample time selection"]
pub type SMP12_TKCG12_R = crate::FieldReader;
#[doc = "Field `SMP12_TKCG12` writer - Channel 12 sample time selection"]
pub type SMP12_TKCG12_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP13_TKCG13` reader - Channel 13 sample time selection"]
pub type SMP13_TKCG13_R = crate::FieldReader;
#[doc = "Field `SMP13_TKCG13` writer - Channel 13 sample time selection"]
pub type SMP13_TKCG13_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP14_TKCG14` reader - Channel 14 sample time selection"]
pub type SMP14_TKCG14_R = crate::FieldReader;
#[doc = "Field `SMP14_TKCG14` writer - Channel 14 sample time selection"]
pub type SMP14_TKCG14_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP15_TKCG15` reader - Channel 15 sample time selection"]
pub type SMP15_TKCG15_R = crate::FieldReader;
#[doc = "Field `SMP15_TKCG15` writer - Channel 15 sample time selection"]
pub type SMP15_TKCG15_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP16_TKCG16` reader - Channel 16 sample time selection"]
pub type SMP16_TKCG16_R = crate::FieldReader;
#[doc = "Field `SMP16_TKCG16` writer - Channel 16 sample time selection"]
pub type SMP16_TKCG16_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP17_TKCG17` reader - Channel 17 sample time selection"]
pub type SMP17_TKCG17_R = crate::FieldReader;
#[doc = "Field `SMP17_TKCG17` writer - Channel 17 sample time selection"]
pub type SMP17_TKCG17_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Channel 10 sample time selection"]
    #[inline(always)]
    pub fn smp10_tkcg10(&self) -> SMP10_TKCG10_R {
        SMP10_TKCG10_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Channel 11 sample time selection"]
    #[inline(always)]
    pub fn smp11_tkcg11(&self) -> SMP11_TKCG11_R {
        SMP11_TKCG11_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Channel 12 sample time selection"]
    #[inline(always)]
    pub fn smp12_tkcg12(&self) -> SMP12_TKCG12_R {
        SMP12_TKCG12_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Channel 13 sample time selection"]
    #[inline(always)]
    pub fn smp13_tkcg13(&self) -> SMP13_TKCG13_R {
        SMP13_TKCG13_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel 14 sample time selection"]
    #[inline(always)]
    pub fn smp14_tkcg14(&self) -> SMP14_TKCG14_R {
        SMP14_TKCG14_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel 15 sample time selection"]
    #[inline(always)]
    pub fn smp15_tkcg15(&self) -> SMP15_TKCG15_R {
        SMP15_TKCG15_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Channel 16 sample time selection"]
    #[inline(always)]
    pub fn smp16_tkcg16(&self) -> SMP16_TKCG16_R {
        SMP16_TKCG16_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Channel 17 sample time selection"]
    #[inline(always)]
    pub fn smp17_tkcg17(&self) -> SMP17_TKCG17_R {
        SMP17_TKCG17_R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel 10 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp10_tkcg10(&mut self) -> SMP10_TKCG10_W<SAMPTR1_CHARGE1_SPEC, 0> {
        SMP10_TKCG10_W::new(self)
    }
    #[doc = "Bits 3:5 - Channel 11 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp11_tkcg11(&mut self) -> SMP11_TKCG11_W<SAMPTR1_CHARGE1_SPEC, 3> {
        SMP11_TKCG11_W::new(self)
    }
    #[doc = "Bits 6:8 - Channel 12 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp12_tkcg12(&mut self) -> SMP12_TKCG12_W<SAMPTR1_CHARGE1_SPEC, 6> {
        SMP12_TKCG12_W::new(self)
    }
    #[doc = "Bits 9:11 - Channel 13 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp13_tkcg13(&mut self) -> SMP13_TKCG13_W<SAMPTR1_CHARGE1_SPEC, 9> {
        SMP13_TKCG13_W::new(self)
    }
    #[doc = "Bits 12:14 - Channel 14 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp14_tkcg14(&mut self) -> SMP14_TKCG14_W<SAMPTR1_CHARGE1_SPEC, 12> {
        SMP14_TKCG14_W::new(self)
    }
    #[doc = "Bits 15:17 - Channel 15 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp15_tkcg15(&mut self) -> SMP15_TKCG15_W<SAMPTR1_CHARGE1_SPEC, 15> {
        SMP15_TKCG15_W::new(self)
    }
    #[doc = "Bits 18:20 - Channel 16 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp16_tkcg16(&mut self) -> SMP16_TKCG16_W<SAMPTR1_CHARGE1_SPEC, 18> {
        SMP16_TKCG16_W::new(self)
    }
    #[doc = "Bits 21:23 - Channel 17 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp17_tkcg17(&mut self) -> SMP17_TKCG17_W<SAMPTR1_CHARGE1_SPEC, 21> {
        SMP17_TKCG17_W::new(self)
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
#[doc = "sample time register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`samptr1_charge1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`samptr1_charge1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAMPTR1_CHARGE1_SPEC;
impl crate::RegisterSpec for SAMPTR1_CHARGE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`samptr1_charge1::R`](R) reader structure"]
impl crate::Readable for SAMPTR1_CHARGE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`samptr1_charge1::W`](W) writer structure"]
impl crate::Writable for SAMPTR1_CHARGE1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAMPTR1_CHARGE1 to value 0"]
impl crate::Resettable for SAMPTR1_CHARGE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
