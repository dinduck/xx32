#[doc = "Register `APB1PCENR` reader"]
pub type R = crate::R<APB1PCENR_SPEC>;
#[doc = "Register `APB1PCENR` writer"]
pub type W = crate::W<APB1PCENR_SPEC>;
#[doc = "Field `TIM2EN` reader - Timer 2 clock enable"]
pub type TIM2EN_R = crate::BitReader;
#[doc = "Field `TIM2EN` writer - Timer 2 clock enable"]
pub type TIM2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM3EN` reader - Timer 3 clock enable"]
pub type TIM3EN_R = crate::BitReader;
#[doc = "Field `TIM3EN` writer - Timer 3 clock enable"]
pub type TIM3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM4EN` reader - Timer 4 clock enable"]
pub type TIM4EN_R = crate::BitReader;
#[doc = "Field `TIM4EN` writer - Timer 4 clock enable"]
pub type TIM4EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM5EN` reader - Timer 5 clock enable"]
pub type TIM5EN_R = crate::BitReader;
#[doc = "Field `TIM5EN` writer - Timer 5 clock enable"]
pub type TIM5EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM6EN` reader - Timer 6 clock enable"]
pub type TIM6EN_R = crate::BitReader;
#[doc = "Field `TIM6EN` writer - Timer 6 clock enable"]
pub type TIM6EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM7EN` reader - Timer 7 clock enable"]
pub type TIM7EN_R = crate::BitReader;
#[doc = "Field `TIM7EN` writer - Timer 7 clock enable"]
pub type TIM7EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART6_EN` reader - USART 6 clock enable"]
pub type USART6_EN_R = crate::BitReader;
#[doc = "Field `USART6_EN` writer - USART 6 clock enable"]
pub type USART6_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART7_EN` reader - USART 7 clock enable"]
pub type USART7_EN_R = crate::BitReader;
#[doc = "Field `USART7_EN` writer - USART 7 clock enable"]
pub type USART7_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART8_EN` reader - USART 8 clock enable"]
pub type USART8_EN_R = crate::BitReader;
#[doc = "Field `USART8_EN` writer - USART 8 clock enable"]
pub type USART8_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WWDGEN` reader - Window watchdog clock enable"]
pub type WWDGEN_R = crate::BitReader;
#[doc = "Field `WWDGEN` writer - Window watchdog clock enable"]
pub type WWDGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI2EN` reader - SPI 2 clock enable"]
pub type SPI2EN_R = crate::BitReader;
#[doc = "Field `SPI2EN` writer - SPI 2 clock enable"]
pub type SPI2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI3EN` reader - SPI 3 clock enable"]
pub type SPI3EN_R = crate::BitReader;
#[doc = "Field `SPI3EN` writer - SPI 3 clock enable"]
pub type SPI3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART2EN` reader - USART 2 clock enable"]
pub type USART2EN_R = crate::BitReader;
#[doc = "Field `USART2EN` writer - USART 2 clock enable"]
pub type USART2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART3EN` reader - USART 3 clock enable"]
pub type USART3EN_R = crate::BitReader;
#[doc = "Field `USART3EN` writer - USART 3 clock enable"]
pub type USART3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART4EN` reader - UART 4 clock enable"]
pub type UART4EN_R = crate::BitReader;
#[doc = "Field `UART4EN` writer - UART 4 clock enable"]
pub type UART4EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART5EN` reader - UART 5 clock enable"]
pub type UART5EN_R = crate::BitReader;
#[doc = "Field `UART5EN` writer - UART 5 clock enable"]
pub type UART5EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C1EN` reader - I2C 1 clock enable"]
pub type I2C1EN_R = crate::BitReader;
#[doc = "Field `I2C1EN` writer - I2C 1 clock enable"]
pub type I2C1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C2EN` reader - I2C 2 clock enable"]
pub type I2C2EN_R = crate::BitReader;
#[doc = "Field `I2C2EN` writer - I2C 2 clock enable"]
pub type I2C2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBDEN` reader - USBD clock enable"]
pub type USBDEN_R = crate::BitReader;
#[doc = "Field `USBDEN` writer - USBD clock enable"]
pub type USBDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN1EN` reader - CAN1 clock enable"]
pub type CAN1EN_R = crate::BitReader;
#[doc = "Field `CAN1EN` writer - CAN1 clock enable"]
pub type CAN1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN2EN` reader - CAN2 clock enable"]
pub type CAN2EN_R = crate::BitReader;
#[doc = "Field `CAN2EN` writer - CAN2 clock enable"]
pub type CAN2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BKPEN` reader - Backup interface clock enable"]
pub type BKPEN_R = crate::BitReader;
#[doc = "Field `BKPEN` writer - Backup interface clock enable"]
pub type BKPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWREN` reader - Power interface clock enable"]
pub type PWREN_R = crate::BitReader;
#[doc = "Field `PWREN` writer - Power interface clock enable"]
pub type PWREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DACEN` reader - DAC interface clock enable"]
pub type DACEN_R = crate::BitReader;
#[doc = "Field `DACEN` writer - DAC interface clock enable"]
pub type DACEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Timer 2 clock enable"]
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 3 clock enable"]
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 4 clock enable"]
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer 5 clock enable"]
    #[inline(always)]
    pub fn tim5en(&self) -> TIM5EN_R {
        TIM5EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 6 clock enable"]
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 7 clock enable"]
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USART 6 clock enable"]
    #[inline(always)]
    pub fn usart6_en(&self) -> USART6_EN_R {
        USART6_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USART 7 clock enable"]
    #[inline(always)]
    pub fn usart7_en(&self) -> USART7_EN_R {
        USART7_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - USART 8 clock enable"]
    #[inline(always)]
    pub fn usart8_en(&self) -> USART8_EN_R {
        USART8_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI 2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI 3 clock enable"]
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART 2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART 3 clock enable"]
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART 4 clock enable"]
    #[inline(always)]
    pub fn uart4en(&self) -> UART4EN_R {
        UART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART 5 clock enable"]
    #[inline(always)]
    pub fn uart5en(&self) -> UART5EN_R {
        UART5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C 1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C 2 clock enable"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USBD clock enable"]
    #[inline(always)]
    pub fn usbden(&self) -> USBDEN_R {
        USBDEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1 clock enable"]
    #[inline(always)]
    pub fn can1en(&self) -> CAN1EN_R {
        CAN1EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN2 clock enable"]
    #[inline(always)]
    pub fn can2en(&self) -> CAN2EN_R {
        CAN2EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Backup interface clock enable"]
    #[inline(always)]
    pub fn bkpen(&self) -> BKPEN_R {
        BKPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC interface clock enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim2en(&mut self) -> TIM2EN_W<APB1PCENR_SPEC, 0> {
        TIM2EN_W::new(self)
    }
    #[doc = "Bit 1 - Timer 3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim3en(&mut self) -> TIM3EN_W<APB1PCENR_SPEC, 1> {
        TIM3EN_W::new(self)
    }
    #[doc = "Bit 2 - Timer 4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim4en(&mut self) -> TIM4EN_W<APB1PCENR_SPEC, 2> {
        TIM4EN_W::new(self)
    }
    #[doc = "Bit 3 - Timer 5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim5en(&mut self) -> TIM5EN_W<APB1PCENR_SPEC, 3> {
        TIM5EN_W::new(self)
    }
    #[doc = "Bit 4 - Timer 6 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim6en(&mut self) -> TIM6EN_W<APB1PCENR_SPEC, 4> {
        TIM6EN_W::new(self)
    }
    #[doc = "Bit 5 - Timer 7 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim7en(&mut self) -> TIM7EN_W<APB1PCENR_SPEC, 5> {
        TIM7EN_W::new(self)
    }
    #[doc = "Bit 6 - USART 6 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart6_en(&mut self) -> USART6_EN_W<APB1PCENR_SPEC, 6> {
        USART6_EN_W::new(self)
    }
    #[doc = "Bit 7 - USART 7 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart7_en(&mut self) -> USART7_EN_W<APB1PCENR_SPEC, 7> {
        USART7_EN_W::new(self)
    }
    #[doc = "Bit 8 - USART 8 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart8_en(&mut self) -> USART8_EN_W<APB1PCENR_SPEC, 8> {
        USART8_EN_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WWDGEN_W<APB1PCENR_SPEC, 11> {
        WWDGEN_W::new(self)
    }
    #[doc = "Bit 14 - SPI 2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> SPI2EN_W<APB1PCENR_SPEC, 14> {
        SPI2EN_W::new(self)
    }
    #[doc = "Bit 15 - SPI 3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi3en(&mut self) -> SPI3EN_W<APB1PCENR_SPEC, 15> {
        SPI3EN_W::new(self)
    }
    #[doc = "Bit 17 - USART 2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> USART2EN_W<APB1PCENR_SPEC, 17> {
        USART2EN_W::new(self)
    }
    #[doc = "Bit 18 - USART 3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart3en(&mut self) -> USART3EN_W<APB1PCENR_SPEC, 18> {
        USART3EN_W::new(self)
    }
    #[doc = "Bit 19 - UART 4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart4en(&mut self) -> UART4EN_W<APB1PCENR_SPEC, 19> {
        UART4EN_W::new(self)
    }
    #[doc = "Bit 20 - UART 5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart5en(&mut self) -> UART5EN_W<APB1PCENR_SPEC, 20> {
        UART5EN_W::new(self)
    }
    #[doc = "Bit 21 - I2C 1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<APB1PCENR_SPEC, 21> {
        I2C1EN_W::new(self)
    }
    #[doc = "Bit 22 - I2C 2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2C2EN_W<APB1PCENR_SPEC, 22> {
        I2C2EN_W::new(self)
    }
    #[doc = "Bit 23 - USBD clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbden(&mut self) -> USBDEN_W<APB1PCENR_SPEC, 23> {
        USBDEN_W::new(self)
    }
    #[doc = "Bit 25 - CAN1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn can1en(&mut self) -> CAN1EN_W<APB1PCENR_SPEC, 25> {
        CAN1EN_W::new(self)
    }
    #[doc = "Bit 26 - CAN2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn can2en(&mut self) -> CAN2EN_W<APB1PCENR_SPEC, 26> {
        CAN2EN_W::new(self)
    }
    #[doc = "Bit 27 - Backup interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkpen(&mut self) -> BKPEN_W<APB1PCENR_SPEC, 27> {
        BKPEN_W::new(self)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwren(&mut self) -> PWREN_W<APB1PCENR_SPEC, 28> {
        PWREN_W::new(self)
    }
    #[doc = "Bit 29 - DAC interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacen(&mut self) -> DACEN_W<APB1PCENR_SPEC, 29> {
        DACEN_W::new(self)
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
#[doc = "APB1 peripheral clock enable register (RCC_APB1PCENR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1pcenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1pcenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1PCENR_SPEC;
impl crate::RegisterSpec for APB1PCENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1pcenr::R`](R) reader structure"]
impl crate::Readable for APB1PCENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb1pcenr::W`](W) writer structure"]
impl crate::Writable for APB1PCENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1PCENR to value 0"]
impl crate::Resettable for APB1PCENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
