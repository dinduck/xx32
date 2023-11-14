#[doc = "Register `VTFIDR` reader"]
pub type R = crate::R<VTFIDR_SPEC>;
#[doc = "Register `VTFIDR` writer"]
pub type W = crate::W<VTFIDR_SPEC>;
#[doc = "Field `VTFID0` reader - VTFID0"]
pub type VTFID0_R = crate::FieldReader;
#[doc = "Field `VTFID0` writer - VTFID0"]
pub type VTFID0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `VTFID1` reader - VTFID1"]
pub type VTFID1_R = crate::FieldReader;
#[doc = "Field `VTFID1` writer - VTFID1"]
pub type VTFID1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `VTFID2` reader - VTFID2"]
pub type VTFID2_R = crate::FieldReader;
#[doc = "Field `VTFID2` writer - VTFID2"]
pub type VTFID2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `VTFID3` reader - VTFID3"]
pub type VTFID3_R = crate::FieldReader;
#[doc = "Field `VTFID3` writer - VTFID3"]
pub type VTFID3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - VTFID0"]
    #[inline(always)]
    pub fn vtfid0(&self) -> VTFID0_R {
        VTFID0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - VTFID1"]
    #[inline(always)]
    pub fn vtfid1(&self) -> VTFID1_R {
        VTFID1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - VTFID2"]
    #[inline(always)]
    pub fn vtfid2(&self) -> VTFID2_R {
        VTFID2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - VTFID3"]
    #[inline(always)]
    pub fn vtfid3(&self) -> VTFID3_R {
        VTFID3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - VTFID0"]
    #[inline(always)]
    #[must_use]
    pub fn vtfid0(&mut self) -> VTFID0_W<VTFIDR_SPEC, 0> {
        VTFID0_W::new(self)
    }
    #[doc = "Bits 8:15 - VTFID1"]
    #[inline(always)]
    #[must_use]
    pub fn vtfid1(&mut self) -> VTFID1_W<VTFIDR_SPEC, 8> {
        VTFID1_W::new(self)
    }
    #[doc = "Bits 16:23 - VTFID2"]
    #[inline(always)]
    #[must_use]
    pub fn vtfid2(&mut self) -> VTFID2_W<VTFIDR_SPEC, 16> {
        VTFID2_W::new(self)
    }
    #[doc = "Bits 24:31 - VTFID3"]
    #[inline(always)]
    #[must_use]
    pub fn vtfid3(&mut self) -> VTFID3_W<VTFIDR_SPEC, 24> {
        VTFID3_W::new(self)
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
#[doc = "ID Config Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vtfidr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vtfidr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VTFIDR_SPEC;
impl crate::RegisterSpec for VTFIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vtfidr::R`](R) reader structure"]
impl crate::Readable for VTFIDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vtfidr::W`](W) writer structure"]
impl crate::Writable for VTFIDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VTFIDR to value 0"]
impl crate::Resettable for VTFIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
