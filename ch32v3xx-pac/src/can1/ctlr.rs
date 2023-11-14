#[doc = "Register `CTLR` reader"]
pub type R = crate::R<CTLR_SPEC>;
#[doc = "Register `CTLR` writer"]
pub type W = crate::W<CTLR_SPEC>;
#[doc = "Field `INRQ` reader - Initialization request"]
pub type INRQ_R = crate::BitReader;
#[doc = "Field `INRQ` writer - Initialization request"]
pub type INRQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLEEP` reader - Sleep mode request"]
pub type SLEEP_R = crate::BitReader;
#[doc = "Field `SLEEP` writer - Sleep mode request"]
pub type SLEEP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXFP` reader - Transmit FIFO priority"]
pub type TXFP_R = crate::BitReader;
#[doc = "Field `TXFP` writer - Transmit FIFO priority"]
pub type TXFP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFLM` reader - Receive FIFO locked mode"]
pub type RFLM_R = crate::BitReader;
#[doc = "Field `RFLM` writer - Receive FIFO locked mode"]
pub type RFLM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NART` reader - No automatic retransmission"]
pub type NART_R = crate::BitReader;
#[doc = "Field `NART` writer - No automatic retransmission"]
pub type NART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AWUM` reader - Automatic wakeup mode"]
pub type AWUM_R = crate::BitReader;
#[doc = "Field `AWUM` writer - Automatic wakeup mode"]
pub type AWUM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ABOM` reader - Automatic bus-off management"]
pub type ABOM_R = crate::BitReader;
#[doc = "Field `ABOM` writer - Automatic bus-off management"]
pub type ABOM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TTCM` reader - Time triggered communication mode"]
pub type TTCM_R = crate::BitReader;
#[doc = "Field `TTCM` writer - Time triggered communication mode"]
pub type TTCM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESET` reader - Software master reset"]
pub type RESET_R = crate::BitReader;
#[doc = "Field `RESET` writer - Software master reset"]
pub type RESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBF` reader - Debug freeze"]
pub type DBF_R = crate::BitReader;
#[doc = "Field `DBF` writer - Debug freeze"]
pub type DBF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Initialization request"]
    #[inline(always)]
    pub fn inrq(&self) -> INRQ_R {
        INRQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sleep mode request"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO priority"]
    #[inline(always)]
    pub fn txfp(&self) -> TXFP_R {
        TXFP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO locked mode"]
    #[inline(always)]
    pub fn rflm(&self) -> RFLM_R {
        RFLM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - No automatic retransmission"]
    #[inline(always)]
    pub fn nart(&self) -> NART_R {
        NART_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic wakeup mode"]
    #[inline(always)]
    pub fn awum(&self) -> AWUM_R {
        AWUM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Automatic bus-off management"]
    #[inline(always)]
    pub fn abom(&self) -> ABOM_R {
        ABOM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Time triggered communication mode"]
    #[inline(always)]
    pub fn ttcm(&self) -> TTCM_R {
        TTCM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Software master reset"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Debug freeze"]
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialization request"]
    #[inline(always)]
    #[must_use]
    pub fn inrq(&mut self) -> INRQ_W<CTLR_SPEC, 0> {
        INRQ_W::new(self)
    }
    #[doc = "Bit 1 - Sleep mode request"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<CTLR_SPEC, 1> {
        SLEEP_W::new(self)
    }
    #[doc = "Bit 2 - Transmit FIFO priority"]
    #[inline(always)]
    #[must_use]
    pub fn txfp(&mut self) -> TXFP_W<CTLR_SPEC, 2> {
        TXFP_W::new(self)
    }
    #[doc = "Bit 3 - Receive FIFO locked mode"]
    #[inline(always)]
    #[must_use]
    pub fn rflm(&mut self) -> RFLM_W<CTLR_SPEC, 3> {
        RFLM_W::new(self)
    }
    #[doc = "Bit 4 - No automatic retransmission"]
    #[inline(always)]
    #[must_use]
    pub fn nart(&mut self) -> NART_W<CTLR_SPEC, 4> {
        NART_W::new(self)
    }
    #[doc = "Bit 5 - Automatic wakeup mode"]
    #[inline(always)]
    #[must_use]
    pub fn awum(&mut self) -> AWUM_W<CTLR_SPEC, 5> {
        AWUM_W::new(self)
    }
    #[doc = "Bit 6 - Automatic bus-off management"]
    #[inline(always)]
    #[must_use]
    pub fn abom(&mut self) -> ABOM_W<CTLR_SPEC, 6> {
        ABOM_W::new(self)
    }
    #[doc = "Bit 7 - Time triggered communication mode"]
    #[inline(always)]
    #[must_use]
    pub fn ttcm(&mut self) -> TTCM_W<CTLR_SPEC, 7> {
        TTCM_W::new(self)
    }
    #[doc = "Bit 15 - Software master reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<CTLR_SPEC, 15> {
        RESET_W::new(self)
    }
    #[doc = "Bit 16 - Debug freeze"]
    #[inline(always)]
    #[must_use]
    pub fn dbf(&mut self) -> DBF_W<CTLR_SPEC, 16> {
        DBF_W::new(self)
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
#[doc = "CAN Master control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTLR_SPEC;
impl crate::RegisterSpec for CTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlr::R`](R) reader structure"]
impl crate::Readable for CTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctlr::W`](W) writer structure"]
impl crate::Writable for CTLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTLR to value 0x0001_0002"]
impl crate::Resettable for CTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0002;
}
