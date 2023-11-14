#[doc = "Register `UHOST_CTRL` reader"]
pub type R = crate::R<UHOST_CTRL_SPEC>;
#[doc = "Register `UHOST_CTRL` writer"]
pub type W = crate::W<UHOST_CTRL_SPEC>;
#[doc = "Field `bUH_TX_BUS_RESET` reader - USB host bus reset status"]
pub type B_UH_TX_BUS_RESET_R = crate::BitReader;
#[doc = "Field `bUH_TX_BUS_RESET` writer - USB host bus reset status"]
pub type B_UH_TX_BUS_RESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `bUH_TX_BUS_SUSPEND` reader - the host sends hang sigal"]
pub type B_UH_TX_BUS_SUSPEND_R = crate::BitReader;
#[doc = "Field `bUH_TX_BUS_SUSPEND` writer - the host sends hang sigal"]
pub type B_UH_TX_BUS_SUSPEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `bUH_TX_BUS_RESUME` reader - host wake up device"]
pub type B_UH_TX_BUS_RESUME_R = crate::BitReader;
#[doc = "Field `bUH_TX_BUS_RESUME` writer - host wake up device"]
pub type B_UH_TX_BUS_RESUME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `bUH_REMOTE_WKUP` reader - the remoke wake-up"]
pub type B_UH_REMOTE_WKUP_R = crate::BitReader;
#[doc = "Field `bUH_REMOTE_WKUP` writer - the remoke wake-up"]
pub type B_UH_REMOTE_WKUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `bUH_PHY_SUSPENDM` reader - USB-PHY thesuspended state the internal USB-PLL is turned off"]
pub type B_UH_PHY_SUSPENDM_R = crate::BitReader;
#[doc = "Field `bUH_PHY_SUSPENDM` writer - USB-PHY thesuspended state the internal USB-PLL is turned off"]
pub type B_UH_PHY_SUSPENDM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `bUH_SOF_FREE` reader - the bus is idle"]
pub type B_UH_SOF_FREE_R = crate::BitReader;
#[doc = "Field `bUH_SOF_EN` reader - automatically generate the SOF packet enabling control bit"]
pub type B_UH_SOF_EN_R = crate::BitReader;
#[doc = "Field `bUH_SOF_EN` writer - automatically generate the SOF packet enabling control bit"]
pub type B_UH_SOF_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - USB host bus reset status"]
    #[inline(always)]
    pub fn b_uh_tx_bus_reset(&self) -> B_UH_TX_BUS_RESET_R {
        B_UH_TX_BUS_RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the host sends hang sigal"]
    #[inline(always)]
    pub fn b_uh_tx_bus_suspend(&self) -> B_UH_TX_BUS_SUSPEND_R {
        B_UH_TX_BUS_SUSPEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - host wake up device"]
    #[inline(always)]
    pub fn b_uh_tx_bus_resume(&self) -> B_UH_TX_BUS_RESUME_R {
        B_UH_TX_BUS_RESUME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the remoke wake-up"]
    #[inline(always)]
    pub fn b_uh_remote_wkup(&self) -> B_UH_REMOTE_WKUP_R {
        B_UH_REMOTE_WKUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB-PHY thesuspended state the internal USB-PLL is turned off"]
    #[inline(always)]
    pub fn b_uh_phy_suspendm(&self) -> B_UH_PHY_SUSPENDM_R {
        B_UH_PHY_SUSPENDM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - the bus is idle"]
    #[inline(always)]
    pub fn b_uh_sof_free(&self) -> B_UH_SOF_FREE_R {
        B_UH_SOF_FREE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - automatically generate the SOF packet enabling control bit"]
    #[inline(always)]
    pub fn b_uh_sof_en(&self) -> B_UH_SOF_EN_R {
        B_UH_SOF_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB host bus reset status"]
    #[inline(always)]
    #[must_use]
    pub fn b_uh_tx_bus_reset(&mut self) -> B_UH_TX_BUS_RESET_W<UHOST_CTRL_SPEC, 0> {
        B_UH_TX_BUS_RESET_W::new(self)
    }
    #[doc = "Bit 1 - the host sends hang sigal"]
    #[inline(always)]
    #[must_use]
    pub fn b_uh_tx_bus_suspend(&mut self) -> B_UH_TX_BUS_SUSPEND_W<UHOST_CTRL_SPEC, 1> {
        B_UH_TX_BUS_SUSPEND_W::new(self)
    }
    #[doc = "Bit 2 - host wake up device"]
    #[inline(always)]
    #[must_use]
    pub fn b_uh_tx_bus_resume(&mut self) -> B_UH_TX_BUS_RESUME_W<UHOST_CTRL_SPEC, 2> {
        B_UH_TX_BUS_RESUME_W::new(self)
    }
    #[doc = "Bit 3 - the remoke wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn b_uh_remote_wkup(&mut self) -> B_UH_REMOTE_WKUP_W<UHOST_CTRL_SPEC, 3> {
        B_UH_REMOTE_WKUP_W::new(self)
    }
    #[doc = "Bit 4 - USB-PHY thesuspended state the internal USB-PLL is turned off"]
    #[inline(always)]
    #[must_use]
    pub fn b_uh_phy_suspendm(&mut self) -> B_UH_PHY_SUSPENDM_W<UHOST_CTRL_SPEC, 4> {
        B_UH_PHY_SUSPENDM_W::new(self)
    }
    #[doc = "Bit 7 - automatically generate the SOF packet enabling control bit"]
    #[inline(always)]
    #[must_use]
    pub fn b_uh_sof_en(&mut self) -> B_UH_SOF_EN_W<UHOST_CTRL_SPEC, 7> {
        B_UH_SOF_EN_W::new(self)
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
#[doc = "USB HOST control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uhost_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhost_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UHOST_CTRL_SPEC;
impl crate::RegisterSpec for UHOST_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uhost_ctrl::R`](R) reader structure"]
impl crate::Readable for UHOST_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uhost_ctrl::W`](W) writer structure"]
impl crate::Writable for UHOST_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UHOST_CTRL to value 0"]
impl crate::Resettable for UHOST_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
