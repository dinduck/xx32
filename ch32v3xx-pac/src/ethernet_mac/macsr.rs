#[doc = "Register `MACSR` reader"]
pub type R = crate::R<MACSR_SPEC>;
#[doc = "Register `MACSR` writer"]
pub type W = crate::W<MACSR_SPEC>;
#[doc = "Field `PMTS` reader - PMT status"]
pub type PMTS_R = crate::BitReader;
#[doc = "Field `PMTS` writer - PMT status"]
pub type PMTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MMCS` reader - MMC status"]
pub type MMCS_R = crate::BitReader;
#[doc = "Field `MMCS` writer - MMC status"]
pub type MMCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MMCRS` reader - MMC receive status"]
pub type MMCRS_R = crate::BitReader;
#[doc = "Field `MMCRS` writer - MMC receive status"]
pub type MMCRS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MMCTS` reader - MMC transmit status"]
pub type MMCTS_R = crate::BitReader;
#[doc = "Field `MMCTS` writer - MMC transmit status"]
pub type MMCTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSTS` reader - Time stamp trigger status"]
pub type TSTS_R = crate::BitReader;
#[doc = "Field `TSTS` writer - Time stamp trigger status"]
pub type TSTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 3 - PMT status"]
    #[inline(always)]
    pub fn pmts(&self) -> PMTS_R {
        PMTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC status"]
    #[inline(always)]
    pub fn mmcs(&self) -> MMCS_R {
        MMCS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC receive status"]
    #[inline(always)]
    pub fn mmcrs(&self) -> MMCRS_R {
        MMCRS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC transmit status"]
    #[inline(always)]
    pub fn mmcts(&self) -> MMCTS_R {
        MMCTS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Time stamp trigger status"]
    #[inline(always)]
    pub fn tsts(&self) -> TSTS_R {
        TSTS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PMT status"]
    #[inline(always)]
    #[must_use]
    pub fn pmts(&mut self) -> PMTS_W<MACSR_SPEC, 3> {
        PMTS_W::new(self)
    }
    #[doc = "Bit 4 - MMC status"]
    #[inline(always)]
    #[must_use]
    pub fn mmcs(&mut self) -> MMCS_W<MACSR_SPEC, 4> {
        MMCS_W::new(self)
    }
    #[doc = "Bit 5 - MMC receive status"]
    #[inline(always)]
    #[must_use]
    pub fn mmcrs(&mut self) -> MMCRS_W<MACSR_SPEC, 5> {
        MMCRS_W::new(self)
    }
    #[doc = "Bit 6 - MMC transmit status"]
    #[inline(always)]
    #[must_use]
    pub fn mmcts(&mut self) -> MMCTS_W<MACSR_SPEC, 6> {
        MMCTS_W::new(self)
    }
    #[doc = "Bit 9 - Time stamp trigger status"]
    #[inline(always)]
    #[must_use]
    pub fn tsts(&mut self) -> TSTS_W<MACSR_SPEC, 9> {
        TSTS_W::new(self)
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
#[doc = "Ethernet MAC interrupt status register (ETH_MACSR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACSR_SPEC;
impl crate::RegisterSpec for MACSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macsr::R`](R) reader structure"]
impl crate::Readable for MACSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macsr::W`](W) writer structure"]
impl crate::Writable for MACSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACSR to value 0"]
impl crate::Resettable for MACSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
