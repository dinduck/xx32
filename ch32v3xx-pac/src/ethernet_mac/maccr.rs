#[doc = "Register `MACCR` reader"]
pub type R = crate::R<MACCR_SPEC>;
#[doc = "Register `MACCR` writer"]
pub type W = crate::W<MACCR_SPEC>;
#[doc = "Field `TCES` reader - Send clock selection bit"]
pub type TCES_R = crate::BitReader;
#[doc = "Field `TCES` writer - Send clock selection bit"]
pub type TCES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCF` reader - Send clock reversal"]
pub type TCF_R = crate::BitReader;
#[doc = "Field `TCF` writer - Send clock reversal"]
pub type TCF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RE` reader - Receiver enable"]
pub type RE_R = crate::BitReader;
#[doc = "Field `RE` writer - Receiver enable"]
pub type RE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TE` reader - Transmitter enable"]
pub type TE_R = crate::BitReader;
#[doc = "Field `TE` writer - Transmitter enable"]
pub type TE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DC` reader - Deferral check"]
pub type DC_R = crate::BitReader;
#[doc = "Field `DC` writer - Deferral check"]
pub type DC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BL` reader - Back-off limit"]
pub type BL_R = crate::FieldReader;
#[doc = "Field `BL` writer - Back-off limit"]
pub type BL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `APCS` reader - Automatic pad/CRC stripping"]
pub type APCS_R = crate::BitReader;
#[doc = "Field `APCS` writer - Automatic pad/CRC stripping"]
pub type APCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RD` reader - Retry disable"]
pub type RD_R = crate::BitReader;
#[doc = "Field `RD` writer - Retry disable"]
pub type RD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IPCO` reader - IPv4 checksum offload"]
pub type IPCO_R = crate::BitReader;
#[doc = "Field `IPCO` writer - IPv4 checksum offload"]
pub type IPCO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DM` reader - Duplex mode"]
pub type DM_R = crate::BitReader;
#[doc = "Field `DM` writer - Duplex mode"]
pub type DM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LM` reader - Loopback mode"]
pub type LM_R = crate::BitReader;
#[doc = "Field `LM` writer - Loopback mode"]
pub type LM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ROD` reader - Receive own disable"]
pub type ROD_R = crate::BitReader;
#[doc = "Field `ROD` writer - Receive own disable"]
pub type ROD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FES` reader - Fast Ethernet speed"]
pub type FES_R = crate::BitReader;
#[doc = "Field `FES` writer - Fast Ethernet speed"]
pub type FES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSD` reader - Carrier sense disable"]
pub type CSD_R = crate::BitReader;
#[doc = "Field `CSD` writer - Carrier sense disable"]
pub type CSD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IFG` reader - Interframe gap"]
pub type IFG_R = crate::FieldReader;
#[doc = "Field `IFG` writer - Interframe gap"]
pub type IFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `IRE` reader - 10MPHY 50Ω set"]
pub type IRE_R = crate::BitReader;
#[doc = "Field `IRE` writer - 10MPHY 50Ω set"]
pub type IRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDI` reader - 10MPHY TX DRIVER bisa current"]
pub type PDI_R = crate::BitReader;
#[doc = "Field `PDI` writer - 10MPHY TX DRIVER bisa current"]
pub type PDI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `JD` reader - Jabber disable"]
pub type JD_R = crate::BitReader;
#[doc = "Field `JD` writer - Jabber disable"]
pub type JD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WD` reader - Watchdog disable"]
pub type WD_R = crate::BitReader;
#[doc = "Field `WD` writer - Watchdog disable"]
pub type WD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCD` reader - SEND clock delay"]
pub type TCD_R = crate::FieldReader;
#[doc = "Field `TCD` writer - SEND clock delay"]
pub type TCD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bit 0 - Send clock selection bit"]
    #[inline(always)]
    pub fn tces(&self) -> TCES_R {
        TCES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Send clock reversal"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Deferral check"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Back-off limit"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Automatic pad/CRC stripping"]
    #[inline(always)]
    pub fn apcs(&self) -> APCS_R {
        APCS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Retry disable"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IPv4 checksum offload"]
    #[inline(always)]
    pub fn ipco(&self) -> IPCO_R {
        IPCO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Duplex mode"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loopback mode"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Receive own disable"]
    #[inline(always)]
    pub fn rod(&self) -> ROD_R {
        ROD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fast Ethernet speed"]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Carrier sense disable"]
    #[inline(always)]
    pub fn csd(&self) -> CSD_R {
        CSD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Interframe gap"]
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - 10MPHY 50Ω set"]
    #[inline(always)]
    pub fn ire(&self) -> IRE_R {
        IRE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 10MPHY TX DRIVER bisa current"]
    #[inline(always)]
    pub fn pdi(&self) -> PDI_R {
        PDI_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Jabber disable"]
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Watchdog disable"]
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 29:31 - SEND clock delay"]
    #[inline(always)]
    pub fn tcd(&self) -> TCD_R {
        TCD_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Send clock selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn tces(&mut self) -> TCES_W<MACCR_SPEC, 0> {
        TCES_W::new(self)
    }
    #[doc = "Bit 1 - Send clock reversal"]
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<MACCR_SPEC, 1> {
        TCF_W::new(self)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<MACCR_SPEC, 2> {
        RE_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<MACCR_SPEC, 3> {
        TE_W::new(self)
    }
    #[doc = "Bit 4 - Deferral check"]
    #[inline(always)]
    #[must_use]
    pub fn dc(&mut self) -> DC_W<MACCR_SPEC, 4> {
        DC_W::new(self)
    }
    #[doc = "Bits 5:6 - Back-off limit"]
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BL_W<MACCR_SPEC, 5> {
        BL_W::new(self)
    }
    #[doc = "Bit 7 - Automatic pad/CRC stripping"]
    #[inline(always)]
    #[must_use]
    pub fn apcs(&mut self) -> APCS_W<MACCR_SPEC, 7> {
        APCS_W::new(self)
    }
    #[doc = "Bit 9 - Retry disable"]
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RD_W<MACCR_SPEC, 9> {
        RD_W::new(self)
    }
    #[doc = "Bit 10 - IPv4 checksum offload"]
    #[inline(always)]
    #[must_use]
    pub fn ipco(&mut self) -> IPCO_W<MACCR_SPEC, 10> {
        IPCO_W::new(self)
    }
    #[doc = "Bit 11 - Duplex mode"]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DM_W<MACCR_SPEC, 11> {
        DM_W::new(self)
    }
    #[doc = "Bit 12 - Loopback mode"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<MACCR_SPEC, 12> {
        LM_W::new(self)
    }
    #[doc = "Bit 13 - Receive own disable"]
    #[inline(always)]
    #[must_use]
    pub fn rod(&mut self) -> ROD_W<MACCR_SPEC, 13> {
        ROD_W::new(self)
    }
    #[doc = "Bit 14 - Fast Ethernet speed"]
    #[inline(always)]
    #[must_use]
    pub fn fes(&mut self) -> FES_W<MACCR_SPEC, 14> {
        FES_W::new(self)
    }
    #[doc = "Bit 16 - Carrier sense disable"]
    #[inline(always)]
    #[must_use]
    pub fn csd(&mut self) -> CSD_W<MACCR_SPEC, 16> {
        CSD_W::new(self)
    }
    #[doc = "Bits 17:19 - Interframe gap"]
    #[inline(always)]
    #[must_use]
    pub fn ifg(&mut self) -> IFG_W<MACCR_SPEC, 17> {
        IFG_W::new(self)
    }
    #[doc = "Bit 20 - 10MPHY 50Ω set"]
    #[inline(always)]
    #[must_use]
    pub fn ire(&mut self) -> IRE_W<MACCR_SPEC, 20> {
        IRE_W::new(self)
    }
    #[doc = "Bit 21 - 10MPHY TX DRIVER bisa current"]
    #[inline(always)]
    #[must_use]
    pub fn pdi(&mut self) -> PDI_W<MACCR_SPEC, 21> {
        PDI_W::new(self)
    }
    #[doc = "Bit 22 - Jabber disable"]
    #[inline(always)]
    #[must_use]
    pub fn jd(&mut self) -> JD_W<MACCR_SPEC, 22> {
        JD_W::new(self)
    }
    #[doc = "Bit 23 - Watchdog disable"]
    #[inline(always)]
    #[must_use]
    pub fn wd(&mut self) -> WD_W<MACCR_SPEC, 23> {
        WD_W::new(self)
    }
    #[doc = "Bits 29:31 - SEND clock delay"]
    #[inline(always)]
    #[must_use]
    pub fn tcd(&mut self) -> TCD_W<MACCR_SPEC, 29> {
        TCD_W::new(self)
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
#[doc = "Ethernet MAC configuration register (ETH_MACCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACCR_SPEC;
impl crate::RegisterSpec for MACCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maccr::R`](R) reader structure"]
impl crate::Readable for MACCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maccr::W`](W) writer structure"]
impl crate::Writable for MACCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACCR to value 0x8000"]
impl crate::Resettable for MACCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
