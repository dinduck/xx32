#[doc = "Register `USBHD_UDEV_CTRL__USBHD_UHOST_CTRL` reader"]
pub type R = crate::R<USBHD_UDEV_CTRL__USBHD_UHOST_CTRL_SPEC>;
#[doc = "Register `USBHD_UDEV_CTRL__USBHD_UHOST_CTRL` writer"]
pub type W = crate::W<USBHD_UDEV_CTRL__USBHD_UHOST_CTRL_SPEC>;
#[doc = "Field `USBHD_UH_PORT_EN__USBHD_UD_PORT_EN` reader - enable USB port: 0=disable, 1=enable port, automatic disabled if USB device detached"]
pub type USBHD_UH_PORT_EN__USBHD_UD_PORT_EN_R = crate::BitReader;
#[doc = "Field `USBHD_UH_PORT_EN__USBHD_UD_PORT_EN` writer - enable USB port: 0=disable, 1=enable port, automatic disabled if USB device detached"]
pub type USBHD_UH_PORT_EN__USBHD_UD_PORT_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBHD_UH_BUS_RESET__USBHD_UD_GP_BIT` reader - force clear FIFO and count of USB"]
pub type USBHD_UH_BUS_RESET__USBHD_UD_GP_BIT_R = crate::BitReader;
#[doc = "Field `USBHD_UH_BUS_RESET__USBHD_UD_GP_BIT` writer - force clear FIFO and count of USB"]
pub type USBHD_UH_BUS_RESET__USBHD_UD_GP_BIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBHD_UH_LOW_SPEED__USBHD_UD_LOW_SPEED` reader - enable USB port low speed: 0=full speed, 1=low speed"]
pub type USBHD_UH_LOW_SPEED__USBHD_UD_LOW_SPEED_R = crate::BitReader;
#[doc = "Field `USBHD_UH_LOW_SPEED__USBHD_UD_LOW_SPEED` writer - enable USB port low speed: 0=full speed, 1=low speed"]
pub type USBHD_UH_LOW_SPEED__USBHD_UD_LOW_SPEED_W<'a, REG, const O: u8> =
    crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBHD_UH_DM_PIN__USBHD_UD_DM_PIN` reader - ReadOnly: indicate current UDM pin level"]
pub type USBHD_UH_DM_PIN__USBHD_UD_DM_PIN_R = crate::BitReader;
#[doc = "Field `USBHD_UH_DP_PIN__USBHD_UD_DP_PIN` reader - USB device enable and internal pullup resistance enable"]
pub type USBHD_UH_DP_PIN__USBHD_UD_DP_PIN_R = crate::BitReader;
#[doc = "Field `USBHD_UH_PD_DIS__USBHD_UD_PD_DIS` reader - disable USB UDP/UDM pulldown resistance: 0=enable pulldown, 1=disable"]
pub type USBHD_UH_PD_DIS__USBHD_UD_PD_DIS_R = crate::BitReader;
#[doc = "Field `USBHD_UH_PD_DIS__USBHD_UD_PD_DIS` writer - disable USB UDP/UDM pulldown resistance: 0=enable pulldown, 1=disable"]
pub type USBHD_UH_PD_DIS__USBHD_UD_PD_DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - enable USB port: 0=disable, 1=enable port, automatic disabled if USB device detached"]
    #[inline(always)]
    pub fn usbhd_uh_port_en__usbhd_ud_port_en(&self) -> USBHD_UH_PORT_EN__USBHD_UD_PORT_EN_R {
        USBHD_UH_PORT_EN__USBHD_UD_PORT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - force clear FIFO and count of USB"]
    #[inline(always)]
    pub fn usbhd_uh_bus_reset__usbhd_ud_gp_bit(&self) -> USBHD_UH_BUS_RESET__USBHD_UD_GP_BIT_R {
        USBHD_UH_BUS_RESET__USBHD_UD_GP_BIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable USB port low speed: 0=full speed, 1=low speed"]
    #[inline(always)]
    pub fn usbhd_uh_low_speed__usbhd_ud_low_speed(
        &self,
    ) -> USBHD_UH_LOW_SPEED__USBHD_UD_LOW_SPEED_R {
        USBHD_UH_LOW_SPEED__USBHD_UD_LOW_SPEED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - ReadOnly: indicate current UDM pin level"]
    #[inline(always)]
    pub fn usbhd_uh_dm_pin__usbhd_ud_dm_pin(&self) -> USBHD_UH_DM_PIN__USBHD_UD_DM_PIN_R {
        USBHD_UH_DM_PIN__USBHD_UD_DM_PIN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB device enable and internal pullup resistance enable"]
    #[inline(always)]
    pub fn usbhd_uh_dp_pin__usbhd_ud_dp_pin(&self) -> USBHD_UH_DP_PIN__USBHD_UD_DP_PIN_R {
        USBHD_UH_DP_PIN__USBHD_UD_DP_PIN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - disable USB UDP/UDM pulldown resistance: 0=enable pulldown, 1=disable"]
    #[inline(always)]
    pub fn usbhd_uh_pd_dis__usbhd_ud_pd_dis(&self) -> USBHD_UH_PD_DIS__USBHD_UD_PD_DIS_R {
        USBHD_UH_PD_DIS__USBHD_UD_PD_DIS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable USB port: 0=disable, 1=enable port, automatic disabled if USB device detached"]
    #[inline(always)]
    #[must_use]
    pub fn usbhd_uh_port_en__usbhd_ud_port_en(
        &mut self,
    ) -> USBHD_UH_PORT_EN__USBHD_UD_PORT_EN_W<USBHD_UDEV_CTRL__USBHD_UHOST_CTRL_SPEC, 0> {
        USBHD_UH_PORT_EN__USBHD_UD_PORT_EN_W::new(self)
    }
    #[doc = "Bit 1 - force clear FIFO and count of USB"]
    #[inline(always)]
    #[must_use]
    pub fn usbhd_uh_bus_reset__usbhd_ud_gp_bit(
        &mut self,
    ) -> USBHD_UH_BUS_RESET__USBHD_UD_GP_BIT_W<USBHD_UDEV_CTRL__USBHD_UHOST_CTRL_SPEC, 1> {
        USBHD_UH_BUS_RESET__USBHD_UD_GP_BIT_W::new(self)
    }
    #[doc = "Bit 2 - enable USB port low speed: 0=full speed, 1=low speed"]
    #[inline(always)]
    #[must_use]
    pub fn usbhd_uh_low_speed__usbhd_ud_low_speed(
        &mut self,
    ) -> USBHD_UH_LOW_SPEED__USBHD_UD_LOW_SPEED_W<USBHD_UDEV_CTRL__USBHD_UHOST_CTRL_SPEC, 2> {
        USBHD_UH_LOW_SPEED__USBHD_UD_LOW_SPEED_W::new(self)
    }
    #[doc = "Bit 7 - disable USB UDP/UDM pulldown resistance: 0=enable pulldown, 1=disable"]
    #[inline(always)]
    #[must_use]
    pub fn usbhd_uh_pd_dis__usbhd_ud_pd_dis(
        &mut self,
    ) -> USBHD_UH_PD_DIS__USBHD_UD_PD_DIS_W<USBHD_UDEV_CTRL__USBHD_UHOST_CTRL_SPEC, 7> {
        USBHD_UH_PD_DIS__USBHD_UD_PD_DIS_W::new(self)
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
#[doc = "USB device/host physical prot control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbhd_udev_ctrl__usbhd_uhost_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbhd_udev_ctrl__usbhd_uhost_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBHD_UDEV_CTRL__USBHD_UHOST_CTRL_SPEC;
impl crate::RegisterSpec for USBHD_UDEV_CTRL__USBHD_UHOST_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usbhd_udev_ctrl__usbhd_uhost_ctrl::R`](R) reader structure"]
impl crate::Readable for USBHD_UDEV_CTRL__USBHD_UHOST_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbhd_udev_ctrl__usbhd_uhost_ctrl::W`](W) writer structure"]
impl crate::Writable for USBHD_UDEV_CTRL__USBHD_UHOST_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBHD_UDEV_CTRL__USBHD_UHOST_CTRL to value 0"]
impl crate::Resettable for USBHD_UDEV_CTRL__USBHD_UHOST_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
