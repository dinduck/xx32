#[doc = "Register `RSQR2` reader"]
pub type R = crate::R<RSQR2_SPEC>;
#[doc = "Register `RSQR2` writer"]
pub type W = crate::W<RSQR2_SPEC>;
#[doc = "Field `SQ7` reader - 7th conversion in regular sequence"]
pub type SQ7_R = crate::FieldReader;
#[doc = "Field `SQ7` writer - 7th conversion in regular sequence"]
pub type SQ7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SQ8` reader - 8th conversion in regular sequence"]
pub type SQ8_R = crate::FieldReader;
#[doc = "Field `SQ8` writer - 8th conversion in regular sequence"]
pub type SQ8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SQ9` reader - 9th conversion in regular sequence"]
pub type SQ9_R = crate::FieldReader;
#[doc = "Field `SQ9` writer - 9th conversion in regular sequence"]
pub type SQ9_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SQ10` reader - 10th conversion in regular sequence"]
pub type SQ10_R = crate::FieldReader;
#[doc = "Field `SQ10` writer - 10th conversion in regular sequence"]
pub type SQ10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SQ11` reader - 11th conversion in regular sequence"]
pub type SQ11_R = crate::FieldReader;
#[doc = "Field `SQ11` writer - 11th conversion in regular sequence"]
pub type SQ11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SQ12` reader - 12th conversion in regular sequence"]
pub type SQ12_R = crate::FieldReader;
#[doc = "Field `SQ12` writer - 12th conversion in regular sequence"]
pub type SQ12_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - 7th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq7(&self) -> SQ7_R {
        SQ7_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 8th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq8(&self) -> SQ8_R {
        SQ8_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 9th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq9(&self) -> SQ9_R {
        SQ9_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 10th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq10(&self) -> SQ10_R {
        SQ10_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 11th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq11(&self) -> SQ11_R {
        SQ11_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - 12th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq12(&self) -> SQ12_R {
        SQ12_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 7th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq7(&mut self) -> SQ7_W<RSQR2_SPEC, 0> {
        SQ7_W::new(self)
    }
    #[doc = "Bits 5:9 - 8th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq8(&mut self) -> SQ8_W<RSQR2_SPEC, 5> {
        SQ8_W::new(self)
    }
    #[doc = "Bits 10:14 - 9th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq9(&mut self) -> SQ9_W<RSQR2_SPEC, 10> {
        SQ9_W::new(self)
    }
    #[doc = "Bits 15:19 - 10th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq10(&mut self) -> SQ10_W<RSQR2_SPEC, 15> {
        SQ10_W::new(self)
    }
    #[doc = "Bits 20:24 - 11th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq11(&mut self) -> SQ11_W<RSQR2_SPEC, 20> {
        SQ11_W::new(self)
    }
    #[doc = "Bits 25:29 - 12th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq12(&mut self) -> SQ12_W<RSQR2_SPEC, 25> {
        SQ12_W::new(self)
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
#[doc = "regular sequence register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsqr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsqr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSQR2_SPEC;
impl crate::RegisterSpec for RSQR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsqr2::R`](R) reader structure"]
impl crate::Readable for RSQR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rsqr2::W`](W) writer structure"]
impl crate::Writable for RSQR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSQR2 to value 0"]
impl crate::Resettable for RSQR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
