#[doc = "Register `SWEVGR` writer"]
pub type W = crate::W<SWEVGR_SPEC>;
#[doc = "Field `UG` writer - Update generation"]
pub type UG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CC1G` writer - Capture/compare 1 generation"]
pub type CC1G_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CC2G` writer - Capture/compare 2 generation"]
pub type CC2G_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CC3G` writer - Capture/compare 3 generation"]
pub type CC3G_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CC4G` writer - Capture/compare 4 generation"]
pub type CC4G_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMG` writer - Capture/compare generation"]
pub type COMG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TG` writer - Trigger generation"]
pub type TG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BG` writer - Brake generation"]
pub type BG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<SWEVGR_SPEC, 0> {
        UG_W::new(self)
    }
    #[doc = "Bit 1 - Capture/compare 1 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc1g(&mut self) -> CC1G_W<SWEVGR_SPEC, 1> {
        CC1G_W::new(self)
    }
    #[doc = "Bit 2 - Capture/compare 2 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc2g(&mut self) -> CC2G_W<SWEVGR_SPEC, 2> {
        CC2G_W::new(self)
    }
    #[doc = "Bit 3 - Capture/compare 3 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc3g(&mut self) -> CC3G_W<SWEVGR_SPEC, 3> {
        CC3G_W::new(self)
    }
    #[doc = "Bit 4 - Capture/compare 4 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc4g(&mut self) -> CC4G_W<SWEVGR_SPEC, 4> {
        CC4G_W::new(self)
    }
    #[doc = "Bit 5 - Capture/compare generation"]
    #[inline(always)]
    #[must_use]
    pub fn comg(&mut self) -> COMG_W<SWEVGR_SPEC, 5> {
        COMG_W::new(self)
    }
    #[doc = "Bit 6 - Trigger generation"]
    #[inline(always)]
    #[must_use]
    pub fn tg(&mut self) -> TG_W<SWEVGR_SPEC, 6> {
        TG_W::new(self)
    }
    #[doc = "Bit 7 - Brake generation"]
    #[inline(always)]
    #[must_use]
    pub fn bg(&mut self) -> BG_W<SWEVGR_SPEC, 7> {
        BG_W::new(self)
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
#[doc = "event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swevgr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWEVGR_SPEC;
impl crate::RegisterSpec for SWEVGR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swevgr::W`](W) writer structure"]
impl crate::Writable for SWEVGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWEVGR to value 0"]
impl crate::Resettable for SWEVGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
