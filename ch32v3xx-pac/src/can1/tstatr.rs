#[doc = "Register `TSTATR` reader"]
pub type R = crate::R<TSTATR_SPEC>;
#[doc = "Register `TSTATR` writer"]
pub type W = crate::W<TSTATR_SPEC>;
#[doc = "Field `RQCP0` reader - Request completed mailbox0"]
pub type RQCP0_R = crate::BitReader;
#[doc = "Field `RQCP0` writer - Request completed mailbox0"]
pub type RQCP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXOK0` reader - Transmission OK of mailbox0"]
pub type TXOK0_R = crate::BitReader;
#[doc = "Field `TXOK0` writer - Transmission OK of mailbox0"]
pub type TXOK0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ALST0` reader - Arbitration lost for mailbox0"]
pub type ALST0_R = crate::BitReader;
#[doc = "Field `ALST0` writer - Arbitration lost for mailbox0"]
pub type ALST0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TERR0` reader - Transmission error of mailbox0"]
pub type TERR0_R = crate::BitReader;
#[doc = "Field `TERR0` writer - Transmission error of mailbox0"]
pub type TERR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ABRQ0` reader - Abort request for mailbox0"]
pub type ABRQ0_R = crate::BitReader;
#[doc = "Field `ABRQ0` writer - Abort request for mailbox0"]
pub type ABRQ0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RQCP1` reader - Request completed mailbox1"]
pub type RQCP1_R = crate::BitReader;
#[doc = "Field `RQCP1` writer - Request completed mailbox1"]
pub type RQCP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXOK1` reader - Transmission OK of mailbox1"]
pub type TXOK1_R = crate::BitReader;
#[doc = "Field `TXOK1` writer - Transmission OK of mailbox1"]
pub type TXOK1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ALST1` reader - Arbitration lost for mailbox1"]
pub type ALST1_R = crate::BitReader;
#[doc = "Field `ALST1` writer - Arbitration lost for mailbox1"]
pub type ALST1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TERR1` reader - Transmission error of mailbox1"]
pub type TERR1_R = crate::BitReader;
#[doc = "Field `TERR1` writer - Transmission error of mailbox1"]
pub type TERR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ABRQ1` reader - Abort request for mailbox 1"]
pub type ABRQ1_R = crate::BitReader;
#[doc = "Field `ABRQ1` writer - Abort request for mailbox 1"]
pub type ABRQ1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RQCP2` reader - Request completed mailbox2"]
pub type RQCP2_R = crate::BitReader;
#[doc = "Field `RQCP2` writer - Request completed mailbox2"]
pub type RQCP2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXOK2` reader - Transmission OK of mailbox 2"]
pub type TXOK2_R = crate::BitReader;
#[doc = "Field `TXOK2` writer - Transmission OK of mailbox 2"]
pub type TXOK2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ALST2` reader - Arbitration lost for mailbox 2"]
pub type ALST2_R = crate::BitReader;
#[doc = "Field `ALST2` writer - Arbitration lost for mailbox 2"]
pub type ALST2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TERR2` reader - Transmission error of mailbox 2"]
pub type TERR2_R = crate::BitReader;
#[doc = "Field `TERR2` writer - Transmission error of mailbox 2"]
pub type TERR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ABRQ2` reader - Abort request for mailbox 2"]
pub type ABRQ2_R = crate::BitReader;
#[doc = "Field `ABRQ2` writer - Abort request for mailbox 2"]
pub type ABRQ2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CODE` reader - Mailbox code"]
pub type CODE_R = crate::FieldReader;
#[doc = "Field `TME0` reader - Transmit mailbox 0 empty"]
pub type TME0_R = crate::BitReader;
#[doc = "Field `TME1` reader - Transmit mailbox 1 empty"]
pub type TME1_R = crate::BitReader;
#[doc = "Field `TME2` reader - Transmit mailbox 2 empty"]
pub type TME2_R = crate::BitReader;
#[doc = "Field `LOW0` reader - Lowest priority flag for mailbox 0"]
pub type LOW0_R = crate::BitReader;
#[doc = "Field `LOW1` reader - Lowest priority flag for mailbox 1"]
pub type LOW1_R = crate::BitReader;
#[doc = "Field `LOW2` reader - Lowest priority flag for mailbox 2"]
pub type LOW2_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Request completed mailbox0"]
    #[inline(always)]
    pub fn rqcp0(&self) -> RQCP0_R {
        RQCP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmission OK of mailbox0"]
    #[inline(always)]
    pub fn txok0(&self) -> TXOK0_R {
        TXOK0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Arbitration lost for mailbox0"]
    #[inline(always)]
    pub fn alst0(&self) -> ALST0_R {
        ALST0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission error of mailbox0"]
    #[inline(always)]
    pub fn terr0(&self) -> TERR0_R {
        TERR0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Abort request for mailbox0"]
    #[inline(always)]
    pub fn abrq0(&self) -> ABRQ0_R {
        ABRQ0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Request completed mailbox1"]
    #[inline(always)]
    pub fn rqcp1(&self) -> RQCP1_R {
        RQCP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission OK of mailbox1"]
    #[inline(always)]
    pub fn txok1(&self) -> TXOK1_R {
        TXOK1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Arbitration lost for mailbox1"]
    #[inline(always)]
    pub fn alst1(&self) -> ALST1_R {
        ALST1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmission error of mailbox1"]
    #[inline(always)]
    pub fn terr1(&self) -> TERR1_R {
        TERR1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Abort request for mailbox 1"]
    #[inline(always)]
    pub fn abrq1(&self) -> ABRQ1_R {
        ABRQ1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Request completed mailbox2"]
    #[inline(always)]
    pub fn rqcp2(&self) -> RQCP2_R {
        RQCP2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmission OK of mailbox 2"]
    #[inline(always)]
    pub fn txok2(&self) -> TXOK2_R {
        TXOK2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Arbitration lost for mailbox 2"]
    #[inline(always)]
    pub fn alst2(&self) -> ALST2_R {
        ALST2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transmission error of mailbox 2"]
    #[inline(always)]
    pub fn terr2(&self) -> TERR2_R {
        TERR2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - Abort request for mailbox 2"]
    #[inline(always)]
    pub fn abrq2(&self) -> ABRQ2_R {
        ABRQ2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Mailbox code"]
    #[inline(always)]
    pub fn code(&self) -> CODE_R {
        CODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Transmit mailbox 0 empty"]
    #[inline(always)]
    pub fn tme0(&self) -> TME0_R {
        TME0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmit mailbox 1 empty"]
    #[inline(always)]
    pub fn tme1(&self) -> TME1_R {
        TME1_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmit mailbox 2 empty"]
    #[inline(always)]
    pub fn tme2(&self) -> TME2_R {
        TME2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Lowest priority flag for mailbox 0"]
    #[inline(always)]
    pub fn low0(&self) -> LOW0_R {
        LOW0_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Lowest priority flag for mailbox 1"]
    #[inline(always)]
    pub fn low1(&self) -> LOW1_R {
        LOW1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Lowest priority flag for mailbox 2"]
    #[inline(always)]
    pub fn low2(&self) -> LOW2_R {
        LOW2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Request completed mailbox0"]
    #[inline(always)]
    #[must_use]
    pub fn rqcp0(&mut self) -> RQCP0_W<TSTATR_SPEC, 0> {
        RQCP0_W::new(self)
    }
    #[doc = "Bit 1 - Transmission OK of mailbox0"]
    #[inline(always)]
    #[must_use]
    pub fn txok0(&mut self) -> TXOK0_W<TSTATR_SPEC, 1> {
        TXOK0_W::new(self)
    }
    #[doc = "Bit 2 - Arbitration lost for mailbox0"]
    #[inline(always)]
    #[must_use]
    pub fn alst0(&mut self) -> ALST0_W<TSTATR_SPEC, 2> {
        ALST0_W::new(self)
    }
    #[doc = "Bit 3 - Transmission error of mailbox0"]
    #[inline(always)]
    #[must_use]
    pub fn terr0(&mut self) -> TERR0_W<TSTATR_SPEC, 3> {
        TERR0_W::new(self)
    }
    #[doc = "Bit 7 - Abort request for mailbox0"]
    #[inline(always)]
    #[must_use]
    pub fn abrq0(&mut self) -> ABRQ0_W<TSTATR_SPEC, 7> {
        ABRQ0_W::new(self)
    }
    #[doc = "Bit 8 - Request completed mailbox1"]
    #[inline(always)]
    #[must_use]
    pub fn rqcp1(&mut self) -> RQCP1_W<TSTATR_SPEC, 8> {
        RQCP1_W::new(self)
    }
    #[doc = "Bit 9 - Transmission OK of mailbox1"]
    #[inline(always)]
    #[must_use]
    pub fn txok1(&mut self) -> TXOK1_W<TSTATR_SPEC, 9> {
        TXOK1_W::new(self)
    }
    #[doc = "Bit 10 - Arbitration lost for mailbox1"]
    #[inline(always)]
    #[must_use]
    pub fn alst1(&mut self) -> ALST1_W<TSTATR_SPEC, 10> {
        ALST1_W::new(self)
    }
    #[doc = "Bit 11 - Transmission error of mailbox1"]
    #[inline(always)]
    #[must_use]
    pub fn terr1(&mut self) -> TERR1_W<TSTATR_SPEC, 11> {
        TERR1_W::new(self)
    }
    #[doc = "Bit 15 - Abort request for mailbox 1"]
    #[inline(always)]
    #[must_use]
    pub fn abrq1(&mut self) -> ABRQ1_W<TSTATR_SPEC, 15> {
        ABRQ1_W::new(self)
    }
    #[doc = "Bit 16 - Request completed mailbox2"]
    #[inline(always)]
    #[must_use]
    pub fn rqcp2(&mut self) -> RQCP2_W<TSTATR_SPEC, 16> {
        RQCP2_W::new(self)
    }
    #[doc = "Bit 17 - Transmission OK of mailbox 2"]
    #[inline(always)]
    #[must_use]
    pub fn txok2(&mut self) -> TXOK2_W<TSTATR_SPEC, 17> {
        TXOK2_W::new(self)
    }
    #[doc = "Bit 18 - Arbitration lost for mailbox 2"]
    #[inline(always)]
    #[must_use]
    pub fn alst2(&mut self) -> ALST2_W<TSTATR_SPEC, 18> {
        ALST2_W::new(self)
    }
    #[doc = "Bit 19 - Transmission error of mailbox 2"]
    #[inline(always)]
    #[must_use]
    pub fn terr2(&mut self) -> TERR2_W<TSTATR_SPEC, 19> {
        TERR2_W::new(self)
    }
    #[doc = "Bit 23 - Abort request for mailbox 2"]
    #[inline(always)]
    #[must_use]
    pub fn abrq2(&mut self) -> ABRQ2_W<TSTATR_SPEC, 23> {
        ABRQ2_W::new(self)
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
#[doc = "CAN transmit status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tstatr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tstatr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSTATR_SPEC;
impl crate::RegisterSpec for TSTATR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tstatr::R`](R) reader structure"]
impl crate::Readable for TSTATR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tstatr::W`](W) writer structure"]
impl crate::Writable for TSTATR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSTATR to value 0x1c00_0000"]
impl crate::Resettable for TSTATR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1c00_0000;
}
