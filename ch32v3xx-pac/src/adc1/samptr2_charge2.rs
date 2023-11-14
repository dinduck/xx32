#[doc = "Register `SAMPTR2_CHARGE2` reader"]
pub type R = crate::R<SAMPTR2_CHARGE2_SPEC>;
#[doc = "Register `SAMPTR2_CHARGE2` writer"]
pub type W = crate::W<SAMPTR2_CHARGE2_SPEC>;
#[doc = "Field `SMP0_TKCG0` reader - Channel 0 sample time selection"]
pub type SMP0_TKCG0_R = crate::FieldReader;
#[doc = "Field `SMP0_TKCG0` writer - Channel 0 sample time selection"]
pub type SMP0_TKCG0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP1_TKCG1` reader - Channel 1 sample time selection"]
pub type SMP1_TKCG1_R = crate::FieldReader;
#[doc = "Field `SMP1_TKCG1` writer - Channel 1 sample time selection"]
pub type SMP1_TKCG1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP2_TKCG2` reader - Channel 2 sample time selection"]
pub type SMP2_TKCG2_R = crate::FieldReader;
#[doc = "Field `SMP2_TKCG2` writer - Channel 2 sample time selection"]
pub type SMP2_TKCG2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP3_TKCG3` reader - Channel 3 sample time selection"]
pub type SMP3_TKCG3_R = crate::FieldReader;
#[doc = "Field `SMP3_TKCG3` writer - Channel 3 sample time selection"]
pub type SMP3_TKCG3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP4_TKCG4` reader - Channel 4 sample time selection"]
pub type SMP4_TKCG4_R = crate::FieldReader;
#[doc = "Field `SMP4_TKCG4` writer - Channel 4 sample time selection"]
pub type SMP4_TKCG4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP5_TKCG5` reader - Channel 5 sample time selection"]
pub type SMP5_TKCG5_R = crate::FieldReader;
#[doc = "Field `SMP5_TKCG5` writer - Channel 5 sample time selection"]
pub type SMP5_TKCG5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP6_TKCG6` reader - Channel 6 sample time selection"]
pub type SMP6_TKCG6_R = crate::FieldReader;
#[doc = "Field `SMP6_TKCG6` writer - Channel 6 sample time selection"]
pub type SMP6_TKCG6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP7_TKCG7` reader - Channel 7 sample time selection"]
pub type SMP7_TKCG7_R = crate::FieldReader;
#[doc = "Field `SMP7_TKCG7` writer - Channel 7 sample time selection"]
pub type SMP7_TKCG7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP8_TKCG8` reader - Channel 8 sample time selection"]
pub type SMP8_TKCG8_R = crate::FieldReader;
#[doc = "Field `SMP8_TKCG8` writer - Channel 8 sample time selection"]
pub type SMP8_TKCG8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP9_TKCG9` reader - Channel 9 sample time selection"]
pub type SMP9_TKCG9_R = crate::FieldReader;
#[doc = "Field `SMP9_TKCG9` writer - Channel 9 sample time selection"]
pub type SMP9_TKCG9_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Channel 0 sample time selection"]
    #[inline(always)]
    pub fn smp0_tkcg0(&self) -> SMP0_TKCG0_R {
        SMP0_TKCG0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Channel 1 sample time selection"]
    #[inline(always)]
    pub fn smp1_tkcg1(&self) -> SMP1_TKCG1_R {
        SMP1_TKCG1_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Channel 2 sample time selection"]
    #[inline(always)]
    pub fn smp2_tkcg2(&self) -> SMP2_TKCG2_R {
        SMP2_TKCG2_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Channel 3 sample time selection"]
    #[inline(always)]
    pub fn smp3_tkcg3(&self) -> SMP3_TKCG3_R {
        SMP3_TKCG3_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel 4 sample time selection"]
    #[inline(always)]
    pub fn smp4_tkcg4(&self) -> SMP4_TKCG4_R {
        SMP4_TKCG4_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel 5 sample time selection"]
    #[inline(always)]
    pub fn smp5_tkcg5(&self) -> SMP5_TKCG5_R {
        SMP5_TKCG5_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Channel 6 sample time selection"]
    #[inline(always)]
    pub fn smp6_tkcg6(&self) -> SMP6_TKCG6_R {
        SMP6_TKCG6_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Channel 7 sample time selection"]
    #[inline(always)]
    pub fn smp7_tkcg7(&self) -> SMP7_TKCG7_R {
        SMP7_TKCG7_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Channel 8 sample time selection"]
    #[inline(always)]
    pub fn smp8_tkcg8(&self) -> SMP8_TKCG8_R {
        SMP8_TKCG8_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Channel 9 sample time selection"]
    #[inline(always)]
    pub fn smp9_tkcg9(&self) -> SMP9_TKCG9_R {
        SMP9_TKCG9_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel 0 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp0_tkcg0(&mut self) -> SMP0_TKCG0_W<SAMPTR2_CHARGE2_SPEC, 0> {
        SMP0_TKCG0_W::new(self)
    }
    #[doc = "Bits 3:5 - Channel 1 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp1_tkcg1(&mut self) -> SMP1_TKCG1_W<SAMPTR2_CHARGE2_SPEC, 3> {
        SMP1_TKCG1_W::new(self)
    }
    #[doc = "Bits 6:8 - Channel 2 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp2_tkcg2(&mut self) -> SMP2_TKCG2_W<SAMPTR2_CHARGE2_SPEC, 6> {
        SMP2_TKCG2_W::new(self)
    }
    #[doc = "Bits 9:11 - Channel 3 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp3_tkcg3(&mut self) -> SMP3_TKCG3_W<SAMPTR2_CHARGE2_SPEC, 9> {
        SMP3_TKCG3_W::new(self)
    }
    #[doc = "Bits 12:14 - Channel 4 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp4_tkcg4(&mut self) -> SMP4_TKCG4_W<SAMPTR2_CHARGE2_SPEC, 12> {
        SMP4_TKCG4_W::new(self)
    }
    #[doc = "Bits 15:17 - Channel 5 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp5_tkcg5(&mut self) -> SMP5_TKCG5_W<SAMPTR2_CHARGE2_SPEC, 15> {
        SMP5_TKCG5_W::new(self)
    }
    #[doc = "Bits 18:20 - Channel 6 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp6_tkcg6(&mut self) -> SMP6_TKCG6_W<SAMPTR2_CHARGE2_SPEC, 18> {
        SMP6_TKCG6_W::new(self)
    }
    #[doc = "Bits 21:23 - Channel 7 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp7_tkcg7(&mut self) -> SMP7_TKCG7_W<SAMPTR2_CHARGE2_SPEC, 21> {
        SMP7_TKCG7_W::new(self)
    }
    #[doc = "Bits 24:26 - Channel 8 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp8_tkcg8(&mut self) -> SMP8_TKCG8_W<SAMPTR2_CHARGE2_SPEC, 24> {
        SMP8_TKCG8_W::new(self)
    }
    #[doc = "Bits 27:29 - Channel 9 sample time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp9_tkcg9(&mut self) -> SMP9_TKCG9_W<SAMPTR2_CHARGE2_SPEC, 27> {
        SMP9_TKCG9_W::new(self)
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
#[doc = "sample time register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`samptr2_charge2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`samptr2_charge2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAMPTR2_CHARGE2_SPEC;
impl crate::RegisterSpec for SAMPTR2_CHARGE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`samptr2_charge2::R`](R) reader structure"]
impl crate::Readable for SAMPTR2_CHARGE2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`samptr2_charge2::W`](W) writer structure"]
impl crate::Writable for SAMPTR2_CHARGE2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAMPTR2_CHARGE2 to value 0"]
impl crate::Resettable for SAMPTR2_CHARGE2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
