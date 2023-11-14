#[doc = "Register `R8_UEP7_MOD` reader"]
pub type R = crate::R<R8_UEP7_MOD_SPEC>;
#[doc = "Register `R8_UEP7_MOD` writer"]
pub type W = crate::W<R8_UEP7_MOD_SPEC>;
#[doc = "Field `RB_UEP7_BUF_MOD` reader - buffer mode of USB endpoint 7"]
pub type RB_UEP7_BUF_MOD_R = crate::BitReader;
#[doc = "Field `RB_UEP7_BUF_MOD` writer - buffer mode of USB endpoint 7"]
pub type RB_UEP7_BUF_MOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_UEP7_TX_EN` reader - enable USB endpoint 7 transmittal (IN)"]
pub type RB_UEP7_TX_EN_R = crate::BitReader;
#[doc = "Field `RB_UEP7_TX_EN` writer - enable USB endpoint 7 transmittal (IN)"]
pub type RB_UEP7_TX_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RB_UEP7_RX_EN` reader - enable USB endpoint 7 receiving (OUT)"]
pub type RB_UEP7_RX_EN_R = crate::BitReader;
#[doc = "Field `RB_UEP7_RX_EN` writer - enable USB endpoint 7 receiving (OUT)"]
pub type RB_UEP7_RX_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - buffer mode of USB endpoint 7"]
    #[inline(always)]
    pub fn rb_uep7_buf_mod(&self) -> RB_UEP7_BUF_MOD_R {
        RB_UEP7_BUF_MOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - enable USB endpoint 7 transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep7_tx_en(&self) -> RB_UEP7_TX_EN_R {
        RB_UEP7_TX_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable USB endpoint 7 receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep7_rx_en(&self) -> RB_UEP7_RX_EN_R {
        RB_UEP7_RX_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - buffer mode of USB endpoint 7"]
    #[inline(always)]
    #[must_use]
    pub fn rb_uep7_buf_mod(&mut self) -> RB_UEP7_BUF_MOD_W<R8_UEP7_MOD_SPEC, 0> {
        RB_UEP7_BUF_MOD_W::new(self)
    }
    #[doc = "Bit 2 - enable USB endpoint 7 transmittal (IN)"]
    #[inline(always)]
    #[must_use]
    pub fn rb_uep7_tx_en(&mut self) -> RB_UEP7_TX_EN_W<R8_UEP7_MOD_SPEC, 2> {
        RB_UEP7_TX_EN_W::new(self)
    }
    #[doc = "Bit 3 - enable USB endpoint 7 receiving (OUT)"]
    #[inline(always)]
    #[must_use]
    pub fn rb_uep7_rx_en(&mut self) -> RB_UEP7_RX_EN_W<R8_UEP7_MOD_SPEC, 3> {
        RB_UEP7_RX_EN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "endpoint 7 mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r8_uep7_mod::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r8_uep7_mod::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8_UEP7_MOD_SPEC;
impl crate::RegisterSpec for R8_UEP7_MOD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_uep7_mod::R`](R) reader structure"]
impl crate::Readable for R8_UEP7_MOD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`r8_uep7_mod::W`](W) writer structure"]
impl crate::Writable for R8_UEP7_MOD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets R8_UEP7_MOD to value 0"]
impl crate::Resettable for R8_UEP7_MOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
