#[doc = "Register `STATR` reader"]
pub type R = crate::R<STATR_SPEC>;
#[doc = "Register `STATR` writer"]
pub type W = crate::W<STATR_SPEC>;
#[doc = "Field `INAK` reader - Initialization acknowledge"]
pub type INAK_R = crate::BitReader;
#[doc = "Field `SLAK` reader - Sleep acknowledge"]
pub type SLAK_R = crate::BitReader;
#[doc = "Field `ERRI` reader - Error interrupt"]
pub type ERRI_R = crate::BitReader;
#[doc = "Field `ERRI` writer - Error interrupt"]
pub type ERRI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WKUI` reader - Wakeup interrupt"]
pub type WKUI_R = crate::BitReader;
#[doc = "Field `WKUI` writer - Wakeup interrupt"]
pub type WKUI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLAKI` reader - Sleep acknowledge interrupt"]
pub type SLAKI_R = crate::BitReader;
#[doc = "Field `SLAKI` writer - Sleep acknowledge interrupt"]
pub type SLAKI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXM` reader - Transmit mode"]
pub type TXM_R = crate::BitReader;
#[doc = "Field `RXM` reader - Receive mode"]
pub type RXM_R = crate::BitReader;
#[doc = "Field `SAMP` reader - Last sample point"]
pub type SAMP_R = crate::BitReader;
#[doc = "Field `RX` reader - Rx signal"]
pub type RX_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Initialization acknowledge"]
    #[inline(always)]
    pub fn inak(&self) -> INAK_R {
        INAK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sleep acknowledge"]
    #[inline(always)]
    pub fn slak(&self) -> SLAK_R {
        SLAK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error interrupt"]
    #[inline(always)]
    pub fn erri(&self) -> ERRI_R {
        ERRI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup interrupt"]
    #[inline(always)]
    pub fn wkui(&self) -> WKUI_R {
        WKUI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sleep acknowledge interrupt"]
    #[inline(always)]
    pub fn slaki(&self) -> SLAKI_R {
        SLAKI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit mode"]
    #[inline(always)]
    pub fn txm(&self) -> TXM_R {
        TXM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive mode"]
    #[inline(always)]
    pub fn rxm(&self) -> RXM_R {
        RXM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Last sample point"]
    #[inline(always)]
    pub fn samp(&self) -> SAMP_R {
        SAMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rx signal"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn erri(&mut self) -> ERRI_W<STATR_SPEC, 2> {
        ERRI_W::new(self)
    }
    #[doc = "Bit 3 - Wakeup interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wkui(&mut self) -> WKUI_W<STATR_SPEC, 3> {
        WKUI_W::new(self)
    }
    #[doc = "Bit 4 - Sleep acknowledge interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slaki(&mut self) -> SLAKI_W<STATR_SPEC, 4> {
        SLAKI_W::new(self)
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
#[doc = "CAN master status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`statr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATR_SPEC;
impl crate::RegisterSpec for STATR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statr::R`](R) reader structure"]
impl crate::Readable for STATR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`statr::W`](W) writer structure"]
impl crate::Writable for STATR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATR to value 0x0c02"]
impl crate::Resettable for STATR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c02;
}
