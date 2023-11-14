#[doc = "Register `RSQR3__CHANNEL` reader"]
pub type R = crate::R<RSQR3__CHANNEL_SPEC>;
#[doc = "Register `RSQR3__CHANNEL` writer"]
pub type W = crate::W<RSQR3__CHANNEL_SPEC>;
#[doc = "Field `SQ1__CHSEL` reader - 1st conversion in regular sequence;TKDY_V channel select"]
pub type SQ1__CHSEL_R = crate::FieldReader;
#[doc = "Field `SQ1__CHSEL` writer - 1st conversion in regular sequence;TKDY_V channel select"]
pub type SQ1__CHSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SQ2` reader - 2nd conversion in regular sequence"]
pub type SQ2_R = crate::FieldReader;
#[doc = "Field `SQ2` writer - 2nd conversion in regular sequence"]
pub type SQ2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SQ3` reader - 3rd conversion in regular sequence"]
pub type SQ3_R = crate::FieldReader;
#[doc = "Field `SQ3` writer - 3rd conversion in regular sequence"]
pub type SQ3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SQ4` reader - 4th conversion in regular sequence"]
pub type SQ4_R = crate::FieldReader;
#[doc = "Field `SQ4` writer - 4th conversion in regular sequence"]
pub type SQ4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SQ5` reader - 5th conversion in regular sequence"]
pub type SQ5_R = crate::FieldReader;
#[doc = "Field `SQ5` writer - 5th conversion in regular sequence"]
pub type SQ5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SQ6` reader - 6th conversion in regular sequence"]
pub type SQ6_R = crate::FieldReader;
#[doc = "Field `SQ6` writer - 6th conversion in regular sequence"]
pub type SQ6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - 1st conversion in regular sequence;TKDY_V channel select"]
    #[inline(always)]
    pub fn sq1__chsel(&self) -> SQ1__CHSEL_R {
        SQ1__CHSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 2nd conversion in regular sequence"]
    #[inline(always)]
    pub fn sq2(&self) -> SQ2_R {
        SQ2_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 3rd conversion in regular sequence"]
    #[inline(always)]
    pub fn sq3(&self) -> SQ3_R {
        SQ3_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 4th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq4(&self) -> SQ4_R {
        SQ4_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 5th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq5(&self) -> SQ5_R {
        SQ5_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - 6th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq6(&self) -> SQ6_R {
        SQ6_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 1st conversion in regular sequence;TKDY_V channel select"]
    #[inline(always)]
    #[must_use]
    pub fn sq1__chsel(&mut self) -> SQ1__CHSEL_W<RSQR3__CHANNEL_SPEC, 0> {
        SQ1__CHSEL_W::new(self)
    }
    #[doc = "Bits 5:9 - 2nd conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq2(&mut self) -> SQ2_W<RSQR3__CHANNEL_SPEC, 5> {
        SQ2_W::new(self)
    }
    #[doc = "Bits 10:14 - 3rd conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq3(&mut self) -> SQ3_W<RSQR3__CHANNEL_SPEC, 10> {
        SQ3_W::new(self)
    }
    #[doc = "Bits 15:19 - 4th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq4(&mut self) -> SQ4_W<RSQR3__CHANNEL_SPEC, 15> {
        SQ4_W::new(self)
    }
    #[doc = "Bits 20:24 - 5th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq5(&mut self) -> SQ5_W<RSQR3__CHANNEL_SPEC, 20> {
        SQ5_W::new(self)
    }
    #[doc = "Bits 25:29 - 6th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq6(&mut self) -> SQ6_W<RSQR3__CHANNEL_SPEC, 25> {
        SQ6_W::new(self)
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
#[doc = "regular sequence register 3;TKEY_V_CHANNEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsqr3__channel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsqr3__channel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSQR3__CHANNEL_SPEC;
impl crate::RegisterSpec for RSQR3__CHANNEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsqr3__channel::R`](R) reader structure"]
impl crate::Readable for RSQR3__CHANNEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rsqr3__channel::W`](W) writer structure"]
impl crate::Writable for RSQR3__CHANNEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSQR3__CHANNEL to value 0"]
impl crate::Resettable for RSQR3__CHANNEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
