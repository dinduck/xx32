#[doc = "Register `PCFR` reader"]
pub type R = crate::R<PCFR_SPEC>;
#[doc = "Register `PCFR` writer"]
pub type W = crate::W<PCFR_SPEC>;
#[doc = "Field `SPI1RM` reader - SPI1 remapping"]
pub type SPI1RM_R = crate::BitReader;
#[doc = "Field `SPI1RM` writer - SPI1 remapping"]
pub type SPI1RM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C1RM` reader - I2C1 remapping"]
pub type I2C1RM_R = crate::BitReader;
#[doc = "Field `I2C1RM` writer - I2C1 remapping"]
pub type I2C1RM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART1RM` reader - USART1 remapping"]
pub type USART1RM_R = crate::BitReader;
#[doc = "Field `USART1RM` writer - USART1 remapping"]
pub type USART1RM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART2RM` reader - USART2 remapping"]
pub type USART2RM_R = crate::BitReader;
#[doc = "Field `USART2RM` writer - USART2 remapping"]
pub type USART2RM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART3RM` reader - USART3 remapping"]
pub type USART3RM_R = crate::FieldReader;
#[doc = "Field `USART3RM` writer - USART3 remapping"]
pub type USART3RM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TIM1RM` reader - TIM1 remapping"]
pub type TIM1RM_R = crate::FieldReader;
#[doc = "Field `TIM1RM` writer - TIM1 remapping"]
pub type TIM1RM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TIM2RM` reader - TIM2 remapping"]
pub type TIM2RM_R = crate::FieldReader;
#[doc = "Field `TIM2RM` writer - TIM2 remapping"]
pub type TIM2RM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TIM3RM` reader - TIM3 remapping"]
pub type TIM3RM_R = crate::FieldReader;
#[doc = "Field `TIM3RM` writer - TIM3 remapping"]
pub type TIM3RM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TIM4RM` reader - TIM4 remapping"]
pub type TIM4RM_R = crate::BitReader;
#[doc = "Field `TIM4RM` writer - TIM4 remapping"]
pub type TIM4RM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN1RM` reader - CAN1 remapping"]
pub type CAN1RM_R = crate::FieldReader;
#[doc = "Field `CAN1RM` writer - CAN1 remapping"]
pub type CAN1RM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PD01RM` reader - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
pub type PD01RM_R = crate::BitReader;
#[doc = "Field `PD01RM` writer - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
pub type PD01RM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM5CH4RM` reader - TIM5 channel4 internal remap"]
pub type TIM5CH4RM_R = crate::BitReader;
#[doc = "Field `TIM5CH4RM` writer - TIM5 channel4 internal remap"]
pub type TIM5CH4RM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC1_ETRGINJ_RM` reader - ADC 1 External trigger injected conversion remapping"]
pub type ADC1_ETRGINJ_RM_R = crate::BitReader;
#[doc = "Field `ADC1_ETRGINJ_RM` writer - ADC 1 External trigger injected conversion remapping"]
pub type ADC1_ETRGINJ_RM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC1_ETRGREG_RM` reader - ADC 1 external trigger regular conversion remapping"]
pub type ADC1_ETRGREG_RM_R = crate::BitReader;
#[doc = "Field `ADC1_ETRGREG_RM` writer - ADC 1 external trigger regular conversion remapping"]
pub type ADC1_ETRGREG_RM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC2_ETRGINJ_RM` reader - ADC 2 External trigger injected conversion remapping"]
pub type ADC2_ETRGINJ_RM_R = crate::BitReader;
#[doc = "Field `ADC2_ETRGINJ_RM` writer - ADC 2 External trigger injected conversion remapping"]
pub type ADC2_ETRGINJ_RM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC2_ETRGREG_RM` reader - ADC 2 external trigger regular conversion remapping"]
pub type ADC2_ETRGREG_RM_R = crate::BitReader;
#[doc = "Field `ADC2_ETRGREG_RM` writer - ADC 2 external trigger regular conversion remapping"]
pub type ADC2_ETRGREG_RM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETHRM` reader - Ethernet remapping"]
pub type ETHRM_R = crate::BitReader;
#[doc = "Field `ETHRM` writer - Ethernet remapping"]
pub type ETHRM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN2RM` reader - CAN2 remapping"]
pub type CAN2RM_R = crate::BitReader;
#[doc = "Field `CAN2RM` writer - CAN2 remapping"]
pub type CAN2RM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MII_RMII_SEL` reader - MII_RMII_SEL"]
pub type MII_RMII_SEL_R = crate::BitReader;
#[doc = "Field `MII_RMII_SEL` writer - MII_RMII_SEL"]
pub type MII_RMII_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWCFG` writer - Serial wire JTAG configuration"]
pub type SWCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SPI3_RM` reader - SPI3 remapping"]
pub type SPI3_RM_R = crate::BitReader;
#[doc = "Field `SPI3_RM` writer - SPI3 remapping"]
pub type SPI3_RM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM2ITRA_RM` reader - TIM2 internally triggers 1 remapping"]
pub type TIM2ITRA_RM_R = crate::BitReader;
#[doc = "Field `TIM2ITRA_RM` writer - TIM2 internally triggers 1 remapping"]
pub type TIM2ITRA_RM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PTP_PPSP_RM` reader - Ethernet PTP_PPS remapping"]
pub type PTP_PPSP_RM_R = crate::BitReader;
#[doc = "Field `PTP_PPSP_RM` writer - Ethernet PTP_PPS remapping"]
pub type PTP_PPSP_RM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SPI1 remapping"]
    #[inline(always)]
    pub fn spi1rm(&self) -> SPI1RM_R {
        SPI1RM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C1 remapping"]
    #[inline(always)]
    pub fn i2c1rm(&self) -> I2C1RM_R {
        I2C1RM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USART1 remapping"]
    #[inline(always)]
    pub fn usart1rm(&self) -> USART1RM_R {
        USART1RM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USART2 remapping"]
    #[inline(always)]
    pub fn usart2rm(&self) -> USART2RM_R {
        USART2RM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - USART3 remapping"]
    #[inline(always)]
    pub fn usart3rm(&self) -> USART3RM_R {
        USART3RM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - TIM1 remapping"]
    #[inline(always)]
    pub fn tim1rm(&self) -> TIM1RM_R {
        TIM1RM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TIM2 remapping"]
    #[inline(always)]
    pub fn tim2rm(&self) -> TIM2RM_R {
        TIM2RM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - TIM3 remapping"]
    #[inline(always)]
    pub fn tim3rm(&self) -> TIM3RM_R {
        TIM3RM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - TIM4 remapping"]
    #[inline(always)]
    pub fn tim4rm(&self) -> TIM4RM_R {
        TIM4RM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - CAN1 remapping"]
    #[inline(always)]
    pub fn can1rm(&self) -> CAN1RM_R {
        CAN1RM_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
    #[inline(always)]
    pub fn pd01rm(&self) -> PD01RM_R {
        PD01RM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM5 channel4 internal remap"]
    #[inline(always)]
    pub fn tim5ch4rm(&self) -> TIM5CH4RM_R {
        TIM5CH4RM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC 1 External trigger injected conversion remapping"]
    #[inline(always)]
    pub fn adc1_etrginj_rm(&self) -> ADC1_ETRGINJ_RM_R {
        ADC1_ETRGINJ_RM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC 1 external trigger regular conversion remapping"]
    #[inline(always)]
    pub fn adc1_etrgreg_rm(&self) -> ADC1_ETRGREG_RM_R {
        ADC1_ETRGREG_RM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADC 2 External trigger injected conversion remapping"]
    #[inline(always)]
    pub fn adc2_etrginj_rm(&self) -> ADC2_ETRGINJ_RM_R {
        ADC2_ETRGINJ_RM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC 2 external trigger regular conversion remapping"]
    #[inline(always)]
    pub fn adc2_etrgreg_rm(&self) -> ADC2_ETRGREG_RM_R {
        ADC2_ETRGREG_RM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Ethernet remapping"]
    #[inline(always)]
    pub fn ethrm(&self) -> ETHRM_R {
        ETHRM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CAN2 remapping"]
    #[inline(always)]
    pub fn can2rm(&self) -> CAN2RM_R {
        CAN2RM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MII_RMII_SEL"]
    #[inline(always)]
    pub fn mii_rmii_sel(&self) -> MII_RMII_SEL_R {
        MII_RMII_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - SPI3 remapping"]
    #[inline(always)]
    pub fn spi3_rm(&self) -> SPI3_RM_R {
        SPI3_RM_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TIM2 internally triggers 1 remapping"]
    #[inline(always)]
    pub fn tim2itra_rm(&self) -> TIM2ITRA_RM_R {
        TIM2ITRA_RM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Ethernet PTP_PPS remapping"]
    #[inline(always)]
    pub fn ptp_ppsp_rm(&self) -> PTP_PPSP_RM_R {
        PTP_PPSP_RM_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rm(&mut self) -> SPI1RM_W<PCFR_SPEC, 0> {
        SPI1RM_W::new(self)
    }
    #[doc = "Bit 1 - I2C1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1rm(&mut self) -> I2C1RM_W<PCFR_SPEC, 1> {
        I2C1RM_W::new(self)
    }
    #[doc = "Bit 2 - USART1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart1rm(&mut self) -> USART1RM_W<PCFR_SPEC, 2> {
        USART1RM_W::new(self)
    }
    #[doc = "Bit 3 - USART2 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart2rm(&mut self) -> USART2RM_W<PCFR_SPEC, 3> {
        USART2RM_W::new(self)
    }
    #[doc = "Bits 4:5 - USART3 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn usart3rm(&mut self) -> USART3RM_W<PCFR_SPEC, 4> {
        USART3RM_W::new(self)
    }
    #[doc = "Bits 6:7 - TIM1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim1rm(&mut self) -> TIM1RM_W<PCFR_SPEC, 6> {
        TIM1RM_W::new(self)
    }
    #[doc = "Bits 8:9 - TIM2 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim2rm(&mut self) -> TIM2RM_W<PCFR_SPEC, 8> {
        TIM2RM_W::new(self)
    }
    #[doc = "Bits 10:11 - TIM3 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim3rm(&mut self) -> TIM3RM_W<PCFR_SPEC, 10> {
        TIM3RM_W::new(self)
    }
    #[doc = "Bit 12 - TIM4 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim4rm(&mut self) -> TIM4RM_W<PCFR_SPEC, 12> {
        TIM4RM_W::new(self)
    }
    #[doc = "Bits 13:14 - CAN1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn can1rm(&mut self) -> CAN1RM_W<PCFR_SPEC, 13> {
        CAN1RM_W::new(self)
    }
    #[doc = "Bit 15 - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
    #[inline(always)]
    #[must_use]
    pub fn pd01rm(&mut self) -> PD01RM_W<PCFR_SPEC, 15> {
        PD01RM_W::new(self)
    }
    #[doc = "Bit 16 - TIM5 channel4 internal remap"]
    #[inline(always)]
    #[must_use]
    pub fn tim5ch4rm(&mut self) -> TIM5CH4RM_W<PCFR_SPEC, 16> {
        TIM5CH4RM_W::new(self)
    }
    #[doc = "Bit 17 - ADC 1 External trigger injected conversion remapping"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_etrginj_rm(&mut self) -> ADC1_ETRGINJ_RM_W<PCFR_SPEC, 17> {
        ADC1_ETRGINJ_RM_W::new(self)
    }
    #[doc = "Bit 18 - ADC 1 external trigger regular conversion remapping"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_etrgreg_rm(&mut self) -> ADC1_ETRGREG_RM_W<PCFR_SPEC, 18> {
        ADC1_ETRGREG_RM_W::new(self)
    }
    #[doc = "Bit 19 - ADC 2 External trigger injected conversion remapping"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_etrginj_rm(&mut self) -> ADC2_ETRGINJ_RM_W<PCFR_SPEC, 19> {
        ADC2_ETRGINJ_RM_W::new(self)
    }
    #[doc = "Bit 20 - ADC 2 external trigger regular conversion remapping"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_etrgreg_rm(&mut self) -> ADC2_ETRGREG_RM_W<PCFR_SPEC, 20> {
        ADC2_ETRGREG_RM_W::new(self)
    }
    #[doc = "Bit 21 - Ethernet remapping"]
    #[inline(always)]
    #[must_use]
    pub fn ethrm(&mut self) -> ETHRM_W<PCFR_SPEC, 21> {
        ETHRM_W::new(self)
    }
    #[doc = "Bit 22 - CAN2 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn can2rm(&mut self) -> CAN2RM_W<PCFR_SPEC, 22> {
        CAN2RM_W::new(self)
    }
    #[doc = "Bit 23 - MII_RMII_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn mii_rmii_sel(&mut self) -> MII_RMII_SEL_W<PCFR_SPEC, 23> {
        MII_RMII_SEL_W::new(self)
    }
    #[doc = "Bits 24:26 - Serial wire JTAG configuration"]
    #[inline(always)]
    #[must_use]
    pub fn swcfg(&mut self) -> SWCFG_W<PCFR_SPEC, 24> {
        SWCFG_W::new(self)
    }
    #[doc = "Bit 28 - SPI3 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn spi3_rm(&mut self) -> SPI3_RM_W<PCFR_SPEC, 28> {
        SPI3_RM_W::new(self)
    }
    #[doc = "Bit 29 - TIM2 internally triggers 1 remapping"]
    #[inline(always)]
    #[must_use]
    pub fn tim2itra_rm(&mut self) -> TIM2ITRA_RM_W<PCFR_SPEC, 29> {
        TIM2ITRA_RM_W::new(self)
    }
    #[doc = "Bit 30 - Ethernet PTP_PPS remapping"]
    #[inline(always)]
    #[must_use]
    pub fn ptp_ppsp_rm(&mut self) -> PTP_PPSP_RM_W<PCFR_SPEC, 30> {
        PTP_PPSP_RM_W::new(self)
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
#[doc = "AF remap and debug I/O configuration register (AFIO_PCFR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCFR_SPEC;
impl crate::RegisterSpec for PCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcfr::R`](R) reader structure"]
impl crate::Readable for PCFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcfr::W`](W) writer structure"]
impl crate::Writable for PCFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCFR to value 0"]
impl crate::Resettable for PCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
