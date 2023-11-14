#[doc = "Register `CTLR1` reader"]
pub type R = crate::R<CTLR1_SPEC>;
#[doc = "Register `CTLR1` writer"]
pub type W = crate::W<CTLR1_SPEC>;
#[doc = "Field `AWDCH` reader - Analog watchdog channel select bits"]
pub type AWDCH_R = crate::FieldReader;
#[doc = "Field `AWDCH` writer - Analog watchdog channel select bits"]
pub type AWDCH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `EOCIE` reader - Interrupt enable for EOC"]
pub type EOCIE_R = crate::BitReader;
#[doc = "Field `EOCIE` writer - Interrupt enable for EOC"]
pub type EOCIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AWDIE` reader - Analog watchdog interrupt enable"]
pub type AWDIE_R = crate::BitReader;
#[doc = "Field `AWDIE` writer - Analog watchdog interrupt enable"]
pub type AWDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `JEOCIE` reader - Interrupt enable for injected channels"]
pub type JEOCIE_R = crate::BitReader;
#[doc = "Field `JEOCIE` writer - Interrupt enable for injected channels"]
pub type JEOCIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCAN` reader - Scan mode enable"]
pub type SCAN_R = crate::BitReader;
#[doc = "Field `SCAN` writer - Scan mode enable"]
pub type SCAN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AWDSGL` reader - Enable the watchdog on a single channel in scan mode"]
pub type AWDSGL_R = crate::BitReader;
#[doc = "Field `AWDSGL` writer - Enable the watchdog on a single channel in scan mode"]
pub type AWDSGL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `JAUTO` reader - Automatic injected group conversion"]
pub type JAUTO_R = crate::BitReader;
#[doc = "Field `JAUTO` writer - Automatic injected group conversion"]
pub type JAUTO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISCEN` reader - Discontinuous mode on regular channels"]
pub type DISCEN_R = crate::BitReader;
#[doc = "Field `DISCEN` writer - Discontinuous mode on regular channels"]
pub type DISCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `JDISCEN` reader - Discontinuous mode on injected channels"]
pub type JDISCEN_R = crate::BitReader;
#[doc = "Field `JDISCEN` writer - Discontinuous mode on injected channels"]
pub type JDISCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISCNUM` reader - Discontinuous mode channel count"]
pub type DISCNUM_R = crate::FieldReader;
#[doc = "Field `DISCNUM` writer - Discontinuous mode channel count"]
pub type DISCNUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DUALMOD` reader - Dual mode selection"]
pub type DUALMOD_R = crate::FieldReader;
#[doc = "Field `DUALMOD` writer - Dual mode selection"]
pub type DUALMOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `JAWDEN` reader - Analog watchdog enable on injected channels"]
pub type JAWDEN_R = crate::BitReader;
#[doc = "Field `JAWDEN` writer - Analog watchdog enable on injected channels"]
pub type JAWDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AWDEN` reader - Analog watchdog enable on regular channels"]
pub type AWDEN_R = crate::BitReader;
#[doc = "Field `AWDEN` writer - Analog watchdog enable on regular channels"]
pub type AWDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TKEYEN` reader - TKEY enable, including TKEY_F and TKEY_V"]
pub type TKEYEN_R = crate::BitReader;
#[doc = "Field `TKEYEN` writer - TKEY enable, including TKEY_F and TKEY_V"]
pub type TKEYEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TKITUNE` reader - TKEY_I enable"]
pub type TKITUNE_R = crate::BitReader;
#[doc = "Field `TKITUNE` writer - TKEY_I enable"]
pub type TKITUNE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BUFEN` reader - TKEY_BUF_Enable"]
pub type BUFEN_R = crate::BitReader;
#[doc = "Field `BUFEN` writer - TKEY_BUF_Enable"]
pub type BUFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGA` reader - ADC_PGA"]
pub type PGA_R = crate::FieldReader;
#[doc = "Field `PGA` writer - ADC_PGA"]
pub type PGA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:4 - Analog watchdog channel select bits"]
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog watchdog interrupt enable"]
    #[inline(always)]
    pub fn awdie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable for injected channels"]
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Scan mode enable"]
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable the watchdog on a single channel in scan mode"]
    #[inline(always)]
    pub fn awdsgl(&self) -> AWDSGL_R {
        AWDSGL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Automatic injected group conversion"]
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Discontinuous mode on regular channels"]
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Discontinuous mode on injected channels"]
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Discontinuous mode channel count"]
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Dual mode selection"]
    #[inline(always)]
    pub fn dualmod(&self) -> DUALMOD_R {
        DUALMOD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - Analog watchdog enable on injected channels"]
    #[inline(always)]
    pub fn jawden(&self) -> JAWDEN_R {
        JAWDEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Analog watchdog enable on regular channels"]
    #[inline(always)]
    pub fn awden(&self) -> AWDEN_R {
        AWDEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TKEY enable, including TKEY_F and TKEY_V"]
    #[inline(always)]
    pub fn tkeyen(&self) -> TKEYEN_R {
        TKEYEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TKEY_I enable"]
    #[inline(always)]
    pub fn tkitune(&self) -> TKITUNE_R {
        TKITUNE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TKEY_BUF_Enable"]
    #[inline(always)]
    pub fn bufen(&self) -> BUFEN_R {
        BUFEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - ADC_PGA"]
    #[inline(always)]
    pub fn pga(&self) -> PGA_R {
        PGA_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Analog watchdog channel select bits"]
    #[inline(always)]
    #[must_use]
    pub fn awdch(&mut self) -> AWDCH_W<CTLR1_SPEC, 0> {
        AWDCH_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EOCIE_W<CTLR1_SPEC, 5> {
        EOCIE_W::new(self)
    }
    #[doc = "Bit 6 - Analog watchdog interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn awdie(&mut self) -> AWDIE_W<CTLR1_SPEC, 6> {
        AWDIE_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt enable for injected channels"]
    #[inline(always)]
    #[must_use]
    pub fn jeocie(&mut self) -> JEOCIE_W<CTLR1_SPEC, 7> {
        JEOCIE_W::new(self)
    }
    #[doc = "Bit 8 - Scan mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn scan(&mut self) -> SCAN_W<CTLR1_SPEC, 8> {
        SCAN_W::new(self)
    }
    #[doc = "Bit 9 - Enable the watchdog on a single channel in scan mode"]
    #[inline(always)]
    #[must_use]
    pub fn awdsgl(&mut self) -> AWDSGL_W<CTLR1_SPEC, 9> {
        AWDSGL_W::new(self)
    }
    #[doc = "Bit 10 - Automatic injected group conversion"]
    #[inline(always)]
    #[must_use]
    pub fn jauto(&mut self) -> JAUTO_W<CTLR1_SPEC, 10> {
        JAUTO_W::new(self)
    }
    #[doc = "Bit 11 - Discontinuous mode on regular channels"]
    #[inline(always)]
    #[must_use]
    pub fn discen(&mut self) -> DISCEN_W<CTLR1_SPEC, 11> {
        DISCEN_W::new(self)
    }
    #[doc = "Bit 12 - Discontinuous mode on injected channels"]
    #[inline(always)]
    #[must_use]
    pub fn jdiscen(&mut self) -> JDISCEN_W<CTLR1_SPEC, 12> {
        JDISCEN_W::new(self)
    }
    #[doc = "Bits 13:15 - Discontinuous mode channel count"]
    #[inline(always)]
    #[must_use]
    pub fn discnum(&mut self) -> DISCNUM_W<CTLR1_SPEC, 13> {
        DISCNUM_W::new(self)
    }
    #[doc = "Bits 16:19 - Dual mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn dualmod(&mut self) -> DUALMOD_W<CTLR1_SPEC, 16> {
        DUALMOD_W::new(self)
    }
    #[doc = "Bit 22 - Analog watchdog enable on injected channels"]
    #[inline(always)]
    #[must_use]
    pub fn jawden(&mut self) -> JAWDEN_W<CTLR1_SPEC, 22> {
        JAWDEN_W::new(self)
    }
    #[doc = "Bit 23 - Analog watchdog enable on regular channels"]
    #[inline(always)]
    #[must_use]
    pub fn awden(&mut self) -> AWDEN_W<CTLR1_SPEC, 23> {
        AWDEN_W::new(self)
    }
    #[doc = "Bit 24 - TKEY enable, including TKEY_F and TKEY_V"]
    #[inline(always)]
    #[must_use]
    pub fn tkeyen(&mut self) -> TKEYEN_W<CTLR1_SPEC, 24> {
        TKEYEN_W::new(self)
    }
    #[doc = "Bit 25 - TKEY_I enable"]
    #[inline(always)]
    #[must_use]
    pub fn tkitune(&mut self) -> TKITUNE_W<CTLR1_SPEC, 25> {
        TKITUNE_W::new(self)
    }
    #[doc = "Bit 26 - TKEY_BUF_Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bufen(&mut self) -> BUFEN_W<CTLR1_SPEC, 26> {
        BUFEN_W::new(self)
    }
    #[doc = "Bits 27:28 - ADC_PGA"]
    #[inline(always)]
    #[must_use]
    pub fn pga(&mut self) -> PGA_W<CTLR1_SPEC, 27> {
        PGA_W::new(self)
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
#[doc = "control register 1/TKEY_V_CTLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTLR1_SPEC;
impl crate::RegisterSpec for CTLR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlr1::R`](R) reader structure"]
impl crate::Readable for CTLR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctlr1::W`](W) writer structure"]
impl crate::Writable for CTLR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTLR1 to value 0"]
impl crate::Resettable for CTLR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
