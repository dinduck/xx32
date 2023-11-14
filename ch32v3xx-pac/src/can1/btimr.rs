#[doc = "Register `BTIMR` reader"]
pub type R = crate::R<BTIMR_SPEC>;
#[doc = "Register `BTIMR` writer"]
pub type W = crate::W<BTIMR_SPEC>;
#[doc = "Field `BRP` reader - Baud rate prescaler"]
pub type BRP_R = crate::FieldReader<u16>;
#[doc = "Field `BRP` writer - Baud rate prescaler"]
pub type BRP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `TS1` reader - Time segment 1"]
pub type TS1_R = crate::FieldReader;
#[doc = "Field `TS1` writer - Time segment 1"]
pub type TS1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TS2` reader - Time segment 2"]
pub type TS2_R = crate::FieldReader;
#[doc = "Field `TS2` writer - Time segment 2"]
pub type TS2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SJW` reader - Resynchronization jump width"]
pub type SJW_R = crate::FieldReader;
#[doc = "Field `SJW` writer - Resynchronization jump width"]
pub type SJW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LBKM` reader - Loop back mode (debug)"]
pub type LBKM_R = crate::BitReader;
#[doc = "Field `LBKM` writer - Loop back mode (debug)"]
pub type LBKM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SILM` reader - Silent mode (debug)"]
pub type SILM_R = crate::BitReader;
#[doc = "Field `SILM` writer - Silent mode (debug)"]
pub type SILM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:9 - Baud rate prescaler"]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:19 - Time segment 1"]
    #[inline(always)]
    pub fn ts1(&self) -> TS1_R {
        TS1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Time segment 2"]
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Resynchronization jump width"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - Loop back mode (debug)"]
    #[inline(always)]
    pub fn lbkm(&self) -> LBKM_R {
        LBKM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Silent mode (debug)"]
    #[inline(always)]
    pub fn silm(&self) -> SILM_R {
        SILM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Baud rate prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn brp(&mut self) -> BRP_W<BTIMR_SPEC, 0> {
        BRP_W::new(self)
    }
    #[doc = "Bits 16:19 - Time segment 1"]
    #[inline(always)]
    #[must_use]
    pub fn ts1(&mut self) -> TS1_W<BTIMR_SPEC, 16> {
        TS1_W::new(self)
    }
    #[doc = "Bits 20:22 - Time segment 2"]
    #[inline(always)]
    #[must_use]
    pub fn ts2(&mut self) -> TS2_W<BTIMR_SPEC, 20> {
        TS2_W::new(self)
    }
    #[doc = "Bits 24:25 - Resynchronization jump width"]
    #[inline(always)]
    #[must_use]
    pub fn sjw(&mut self) -> SJW_W<BTIMR_SPEC, 24> {
        SJW_W::new(self)
    }
    #[doc = "Bit 30 - Loop back mode (debug)"]
    #[inline(always)]
    #[must_use]
    pub fn lbkm(&mut self) -> LBKM_W<BTIMR_SPEC, 30> {
        LBKM_W::new(self)
    }
    #[doc = "Bit 31 - Silent mode (debug)"]
    #[inline(always)]
    #[must_use]
    pub fn silm(&mut self) -> SILM_W<BTIMR_SPEC, 31> {
        SILM_W::new(self)
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
#[doc = "CAN bit timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btimr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btimr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTIMR_SPEC;
impl crate::RegisterSpec for BTIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btimr::R`](R) reader structure"]
impl crate::Readable for BTIMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`btimr::W`](W) writer structure"]
impl crate::Writable for BTIMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BTIMR to value 0x0123_0000"]
impl crate::Resettable for BTIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0123_0000;
}
