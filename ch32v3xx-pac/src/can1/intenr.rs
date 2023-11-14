#[doc = "Register `INTENR` reader"]
pub type R = crate::R<INTENR_SPEC>;
#[doc = "Register `INTENR` writer"]
pub type W = crate::W<INTENR_SPEC>;
#[doc = "Field `TMEIE` reader - Transmit mailbox empty interrupt enable"]
pub type TMEIE_R = crate::BitReader;
#[doc = "Field `TMEIE` writer - Transmit mailbox empty interrupt enable"]
pub type TMEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMPIE0` reader - FIFO message pending interrupt enable"]
pub type FMPIE0_R = crate::BitReader;
#[doc = "Field `FMPIE0` writer - FIFO message pending interrupt enable"]
pub type FMPIE0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FFIE0` reader - FIFO full interrupt enable"]
pub type FFIE0_R = crate::BitReader;
#[doc = "Field `FFIE0` writer - FIFO full interrupt enable"]
pub type FFIE0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FOVIE0` reader - FIFO overrun interrupt enable"]
pub type FOVIE0_R = crate::BitReader;
#[doc = "Field `FOVIE0` writer - FIFO overrun interrupt enable"]
pub type FOVIE0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMPIE1` reader - FIFO message pending interrupt enable"]
pub type FMPIE1_R = crate::BitReader;
#[doc = "Field `FMPIE1` writer - FIFO message pending interrupt enable"]
pub type FMPIE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FFIE1` reader - FIFO full interrupt enable"]
pub type FFIE1_R = crate::BitReader;
#[doc = "Field `FFIE1` writer - FIFO full interrupt enable"]
pub type FFIE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FOVIE1` reader - FIFO overrun interrupt enable"]
pub type FOVIE1_R = crate::BitReader;
#[doc = "Field `FOVIE1` writer - FIFO overrun interrupt enable"]
pub type FOVIE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EWGIE` reader - Error warning interrupt enable"]
pub type EWGIE_R = crate::BitReader;
#[doc = "Field `EWGIE` writer - Error warning interrupt enable"]
pub type EWGIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPVIE` reader - Error passive interrupt enable"]
pub type EPVIE_R = crate::BitReader;
#[doc = "Field `EPVIE` writer - Error passive interrupt enable"]
pub type EPVIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BOFIE` reader - Bus-off interrupt enable"]
pub type BOFIE_R = crate::BitReader;
#[doc = "Field `BOFIE` writer - Bus-off interrupt enable"]
pub type BOFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LECIE` reader - Last error code interrupt enable"]
pub type LECIE_R = crate::BitReader;
#[doc = "Field `LECIE` writer - Last error code interrupt enable"]
pub type LECIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WKUIE` reader - Wakeup interrupt enable"]
pub type WKUIE_R = crate::BitReader;
#[doc = "Field `WKUIE` writer - Wakeup interrupt enable"]
pub type WKUIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLKIE` reader - Sleep interrupt enable"]
pub type SLKIE_R = crate::BitReader;
#[doc = "Field `SLKIE` writer - Sleep interrupt enable"]
pub type SLKIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Transmit mailbox empty interrupt enable"]
    #[inline(always)]
    pub fn tmeie(&self) -> TMEIE_R {
        TMEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO message pending interrupt enable"]
    #[inline(always)]
    pub fn fmpie0(&self) -> FMPIE0_R {
        FMPIE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO full interrupt enable"]
    #[inline(always)]
    pub fn ffie0(&self) -> FFIE0_R {
        FFIE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO overrun interrupt enable"]
    #[inline(always)]
    pub fn fovie0(&self) -> FOVIE0_R {
        FOVIE0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FIFO message pending interrupt enable"]
    #[inline(always)]
    pub fn fmpie1(&self) -> FMPIE1_R {
        FMPIE1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FIFO full interrupt enable"]
    #[inline(always)]
    pub fn ffie1(&self) -> FFIE1_R {
        FFIE1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FIFO overrun interrupt enable"]
    #[inline(always)]
    pub fn fovie1(&self) -> FOVIE1_R {
        FOVIE1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Error warning interrupt enable"]
    #[inline(always)]
    pub fn ewgie(&self) -> EWGIE_R {
        EWGIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Error passive interrupt enable"]
    #[inline(always)]
    pub fn epvie(&self) -> EPVIE_R {
        EPVIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bus-off interrupt enable"]
    #[inline(always)]
    pub fn bofie(&self) -> BOFIE_R {
        BOFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Last error code interrupt enable"]
    #[inline(always)]
    pub fn lecie(&self) -> LECIE_R {
        LECIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Wakeup interrupt enable"]
    #[inline(always)]
    pub fn wkuie(&self) -> WKUIE_R {
        WKUIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Sleep interrupt enable"]
    #[inline(always)]
    pub fn slkie(&self) -> SLKIE_R {
        SLKIE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit mailbox empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmeie(&mut self) -> TMEIE_W<INTENR_SPEC, 0> {
        TMEIE_W::new(self)
    }
    #[doc = "Bit 1 - FIFO message pending interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fmpie0(&mut self) -> FMPIE0_W<INTENR_SPEC, 1> {
        FMPIE0_W::new(self)
    }
    #[doc = "Bit 2 - FIFO full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ffie0(&mut self) -> FFIE0_W<INTENR_SPEC, 2> {
        FFIE0_W::new(self)
    }
    #[doc = "Bit 3 - FIFO overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fovie0(&mut self) -> FOVIE0_W<INTENR_SPEC, 3> {
        FOVIE0_W::new(self)
    }
    #[doc = "Bit 4 - FIFO message pending interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fmpie1(&mut self) -> FMPIE1_W<INTENR_SPEC, 4> {
        FMPIE1_W::new(self)
    }
    #[doc = "Bit 5 - FIFO full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ffie1(&mut self) -> FFIE1_W<INTENR_SPEC, 5> {
        FFIE1_W::new(self)
    }
    #[doc = "Bit 6 - FIFO overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fovie1(&mut self) -> FOVIE1_W<INTENR_SPEC, 6> {
        FOVIE1_W::new(self)
    }
    #[doc = "Bit 8 - Error warning interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ewgie(&mut self) -> EWGIE_W<INTENR_SPEC, 8> {
        EWGIE_W::new(self)
    }
    #[doc = "Bit 9 - Error passive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn epvie(&mut self) -> EPVIE_W<INTENR_SPEC, 9> {
        EPVIE_W::new(self)
    }
    #[doc = "Bit 10 - Bus-off interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bofie(&mut self) -> BOFIE_W<INTENR_SPEC, 10> {
        BOFIE_W::new(self)
    }
    #[doc = "Bit 11 - Last error code interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lecie(&mut self) -> LECIE_W<INTENR_SPEC, 11> {
        LECIE_W::new(self)
    }
    #[doc = "Bit 15 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<INTENR_SPEC, 15> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 16 - Wakeup interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wkuie(&mut self) -> WKUIE_W<INTENR_SPEC, 16> {
        WKUIE_W::new(self)
    }
    #[doc = "Bit 17 - Sleep interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn slkie(&mut self) -> SLKIE_W<INTENR_SPEC, 17> {
        SLKIE_W::new(self)
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
#[doc = "CAN interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTENR_SPEC;
impl crate::RegisterSpec for INTENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenr::R`](R) reader structure"]
impl crate::Readable for INTENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intenr::W`](W) writer structure"]
impl crate::Writable for INTENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENR to value 0"]
impl crate::Resettable for INTENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
