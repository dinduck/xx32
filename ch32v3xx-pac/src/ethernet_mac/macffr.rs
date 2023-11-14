#[doc = "Register `MACFFR` reader"]
pub type R = crate::R<MACFFR_SPEC>;
#[doc = "Register `MACFFR` writer"]
pub type W = crate::W<MACFFR_SPEC>;
#[doc = "Field `PM` reader - Promiscuous mode"]
pub type PM_R = crate::BitReader;
#[doc = "Field `PM` writer - Promiscuous mode"]
pub type PM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HU` reader - Hash unicast"]
pub type HU_R = crate::BitReader;
#[doc = "Field `HU` writer - Hash unicast"]
pub type HU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HM` reader - Hash multicast"]
pub type HM_R = crate::BitReader;
#[doc = "Field `HM` writer - Hash multicast"]
pub type HM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DAIF` reader - Destination address inverse filtering"]
pub type DAIF_R = crate::BitReader;
#[doc = "Field `DAIF` writer - Destination address inverse filtering"]
pub type DAIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PAM` reader - Pass all multicast"]
pub type PAM_R = crate::BitReader;
#[doc = "Field `PAM` writer - Pass all multicast"]
pub type PAM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BFD` reader - Broadcast frames disable"]
pub type BFD_R = crate::BitReader;
#[doc = "Field `BFD` writer - Broadcast frames disable"]
pub type BFD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCF` reader - Pass control frames"]
pub type PCF_R = crate::FieldReader;
#[doc = "Field `PCF` writer - Pass control frames"]
pub type PCF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SAIF` reader - Source address inverse filtering"]
pub type SAIF_R = crate::BitReader;
#[doc = "Field `SAIF` writer - Source address inverse filtering"]
pub type SAIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAF` reader - Source address filter"]
pub type SAF_R = crate::BitReader;
#[doc = "Field `SAF` writer - Source address filter"]
pub type SAF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HPF` reader - Hash or perfect filter"]
pub type HPF_R = crate::BitReader;
#[doc = "Field `HPF` writer - Hash or perfect filter"]
pub type HPF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RA` reader - Receive all"]
pub type RA_R = crate::BitReader;
#[doc = "Field `RA` writer - Receive all"]
pub type RA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Promiscuous mode"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hash unicast"]
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hash multicast"]
    #[inline(always)]
    pub fn hm(&self) -> HM_R {
        HM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Destination address inverse filtering"]
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pass all multicast"]
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Broadcast frames disable"]
    #[inline(always)]
    pub fn bfd(&self) -> BFD_R {
        BFD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Pass control frames"]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Source address inverse filtering"]
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Source address filter"]
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hash or perfect filter"]
    #[inline(always)]
    pub fn hpf(&self) -> HPF_R {
        HPF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive all"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Promiscuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<MACFFR_SPEC, 0> {
        PM_W::new(self)
    }
    #[doc = "Bit 1 - Hash unicast"]
    #[inline(always)]
    #[must_use]
    pub fn hu(&mut self) -> HU_W<MACFFR_SPEC, 1> {
        HU_W::new(self)
    }
    #[doc = "Bit 2 - Hash multicast"]
    #[inline(always)]
    #[must_use]
    pub fn hm(&mut self) -> HM_W<MACFFR_SPEC, 2> {
        HM_W::new(self)
    }
    #[doc = "Bit 3 - Destination address inverse filtering"]
    #[inline(always)]
    #[must_use]
    pub fn daif(&mut self) -> DAIF_W<MACFFR_SPEC, 3> {
        DAIF_W::new(self)
    }
    #[doc = "Bit 4 - Pass all multicast"]
    #[inline(always)]
    #[must_use]
    pub fn pam(&mut self) -> PAM_W<MACFFR_SPEC, 4> {
        PAM_W::new(self)
    }
    #[doc = "Bit 5 - Broadcast frames disable"]
    #[inline(always)]
    #[must_use]
    pub fn bfd(&mut self) -> BFD_W<MACFFR_SPEC, 5> {
        BFD_W::new(self)
    }
    #[doc = "Bits 6:7 - Pass control frames"]
    #[inline(always)]
    #[must_use]
    pub fn pcf(&mut self) -> PCF_W<MACFFR_SPEC, 6> {
        PCF_W::new(self)
    }
    #[doc = "Bit 8 - Source address inverse filtering"]
    #[inline(always)]
    #[must_use]
    pub fn saif(&mut self) -> SAIF_W<MACFFR_SPEC, 8> {
        SAIF_W::new(self)
    }
    #[doc = "Bit 9 - Source address filter"]
    #[inline(always)]
    #[must_use]
    pub fn saf(&mut self) -> SAF_W<MACFFR_SPEC, 9> {
        SAF_W::new(self)
    }
    #[doc = "Bit 10 - Hash or perfect filter"]
    #[inline(always)]
    #[must_use]
    pub fn hpf(&mut self) -> HPF_W<MACFFR_SPEC, 10> {
        HPF_W::new(self)
    }
    #[doc = "Bit 31 - Receive all"]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RA_W<MACFFR_SPEC, 31> {
        RA_W::new(self)
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
#[doc = "Ethernet MAC frame filter register (ETH_MACCFFR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macffr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macffr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACFFR_SPEC;
impl crate::RegisterSpec for MACFFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macffr::R`](R) reader structure"]
impl crate::Readable for MACFFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macffr::W`](W) writer structure"]
impl crate::Writable for MACFFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACFFR to value 0"]
impl crate::Resettable for MACFFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
