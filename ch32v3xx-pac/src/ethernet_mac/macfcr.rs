#[doc = "Register `MACFCR` reader"]
pub type R = crate::R<MACFCR_SPEC>;
#[doc = "Register `MACFCR` writer"]
pub type W = crate::W<MACFCR_SPEC>;
#[doc = "Field `FCB_BPA` reader - Flow control busy/back pressure activate"]
pub type FCB_BPA_R = crate::BitReader;
#[doc = "Field `FCB_BPA` writer - Flow control busy/back pressure activate"]
pub type FCB_BPA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TFCE` reader - Transmit flow control enable"]
pub type TFCE_R = crate::BitReader;
#[doc = "Field `TFCE` writer - Transmit flow control enable"]
pub type TFCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFCE` reader - Receive flow control enable"]
pub type RFCE_R = crate::BitReader;
#[doc = "Field `RFCE` writer - Receive flow control enable"]
pub type RFCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPFD` reader - Unicast pause frame detect"]
pub type UPFD_R = crate::BitReader;
#[doc = "Field `UPFD` writer - Unicast pause frame detect"]
pub type UPFD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLT` reader - Pause low threshold"]
pub type PLT_R = crate::FieldReader;
#[doc = "Field `PLT` writer - Pause low threshold"]
pub type PLT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ZQPD` reader - Zero-quanta pause disable"]
pub type ZQPD_R = crate::BitReader;
#[doc = "Field `ZQPD` writer - Zero-quanta pause disable"]
pub type ZQPD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PT` reader - Pass control frames"]
pub type PT_R = crate::FieldReader<u16>;
#[doc = "Field `PT` writer - Pass control frames"]
pub type PT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - Flow control busy/back pressure activate"]
    #[inline(always)]
    pub fn fcb_bpa(&self) -> FCB_BPA_R {
        FCB_BPA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit flow control enable"]
    #[inline(always)]
    pub fn tfce(&self) -> TFCE_R {
        TFCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive flow control enable"]
    #[inline(always)]
    pub fn rfce(&self) -> RFCE_R {
        RFCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Unicast pause frame detect"]
    #[inline(always)]
    pub fn upfd(&self) -> UPFD_R {
        UPFD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Pause low threshold"]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Zero-quanta pause disable"]
    #[inline(always)]
    pub fn zqpd(&self) -> ZQPD_R {
        ZQPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Pass control frames"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flow control busy/back pressure activate"]
    #[inline(always)]
    #[must_use]
    pub fn fcb_bpa(&mut self) -> FCB_BPA_W<MACFCR_SPEC, 0> {
        FCB_BPA_W::new(self)
    }
    #[doc = "Bit 1 - Transmit flow control enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfce(&mut self) -> TFCE_W<MACFCR_SPEC, 1> {
        TFCE_W::new(self)
    }
    #[doc = "Bit 2 - Receive flow control enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfce(&mut self) -> RFCE_W<MACFCR_SPEC, 2> {
        RFCE_W::new(self)
    }
    #[doc = "Bit 3 - Unicast pause frame detect"]
    #[inline(always)]
    #[must_use]
    pub fn upfd(&mut self) -> UPFD_W<MACFCR_SPEC, 3> {
        UPFD_W::new(self)
    }
    #[doc = "Bits 4:5 - Pause low threshold"]
    #[inline(always)]
    #[must_use]
    pub fn plt(&mut self) -> PLT_W<MACFCR_SPEC, 4> {
        PLT_W::new(self)
    }
    #[doc = "Bit 7 - Zero-quanta pause disable"]
    #[inline(always)]
    #[must_use]
    pub fn zqpd(&mut self) -> ZQPD_W<MACFCR_SPEC, 7> {
        ZQPD_W::new(self)
    }
    #[doc = "Bits 16:31 - Pass control frames"]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PT_W<MACFCR_SPEC, 16> {
        PT_W::new(self)
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
#[doc = "Ethernet MAC flow control register (ETH_MACFCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macfcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macfcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACFCR_SPEC;
impl crate::RegisterSpec for MACFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macfcr::R`](R) reader structure"]
impl crate::Readable for MACFCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macfcr::W`](W) writer structure"]
impl crate::Writable for MACFCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACFCR to value 0"]
impl crate::Resettable for MACFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
