#[doc = "Register `APB1PRSTR` reader"]
pub type R = crate::R<APB1PRSTR_SPEC>;
#[doc = "Register `APB1PRSTR` writer"]
pub type W = crate::W<APB1PRSTR_SPEC>;
#[doc = "Field `TIM2RST` reader - Timer 2 reset"]
pub type TIM2RST_R = crate::BitReader;
#[doc = "Field `TIM2RST` writer - Timer 2 reset"]
pub type TIM2RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM3RST` reader - Timer 3 reset"]
pub type TIM3RST_R = crate::BitReader;
#[doc = "Field `TIM3RST` writer - Timer 3 reset"]
pub type TIM3RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM4RST` reader - Timer 4 reset"]
pub type TIM4RST_R = crate::BitReader;
#[doc = "Field `TIM4RST` writer - Timer 4 reset"]
pub type TIM4RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM5RST` reader - Timer 5 reset"]
pub type TIM5RST_R = crate::BitReader;
#[doc = "Field `TIM5RST` writer - Timer 5 reset"]
pub type TIM5RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM6RST` reader - Timer 6 reset"]
pub type TIM6RST_R = crate::BitReader;
#[doc = "Field `TIM6RST` writer - Timer 6 reset"]
pub type TIM6RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM7RST` reader - Timer 7 reset"]
pub type TIM7RST_R = crate::BitReader;
#[doc = "Field `TIM7RST` writer - Timer 7 reset"]
pub type TIM7RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART6RST` reader - UART 6 reset"]
pub type UART6RST_R = crate::BitReader;
#[doc = "Field `UART6RST` writer - UART 6 reset"]
pub type UART6RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART7RST` reader - UART 7 reset"]
pub type UART7RST_R = crate::BitReader;
#[doc = "Field `UART7RST` writer - UART 7 reset"]
pub type UART7RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART8RST` reader - UART 8 reset"]
pub type UART8RST_R = crate::BitReader;
#[doc = "Field `UART8RST` writer - UART 8 reset"]
pub type UART8RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WWDGRST` reader - Window watchdog reset"]
pub type WWDGRST_R = crate::BitReader;
#[doc = "Field `WWDGRST` writer - Window watchdog reset"]
pub type WWDGRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI2RST` reader - SPI2 reset"]
pub type SPI2RST_R = crate::BitReader;
#[doc = "Field `SPI2RST` writer - SPI2 reset"]
pub type SPI2RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI3RST` reader - SPI3 reset"]
pub type SPI3RST_R = crate::BitReader;
#[doc = "Field `SPI3RST` writer - SPI3 reset"]
pub type SPI3RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART2RST` reader - USART 2 reset"]
pub type USART2RST_R = crate::BitReader;
#[doc = "Field `USART2RST` writer - USART 2 reset"]
pub type USART2RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART3RST` reader - USART 3 reset"]
pub type USART3RST_R = crate::BitReader;
#[doc = "Field `USART3RST` writer - USART 3 reset"]
pub type USART3RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART4RST` reader - USART 4 reset"]
pub type USART4RST_R = crate::BitReader;
#[doc = "Field `USART4RST` writer - USART 4 reset"]
pub type USART4RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART5RST` reader - USART 5 reset"]
pub type USART5RST_R = crate::BitReader;
#[doc = "Field `USART5RST` writer - USART 5 reset"]
pub type USART5RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C1RST` reader - I2C1 reset"]
pub type I2C1RST_R = crate::BitReader;
#[doc = "Field `I2C1RST` writer - I2C1 reset"]
pub type I2C1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C2RST` reader - I2C2 reset"]
pub type I2C2RST_R = crate::BitReader;
#[doc = "Field `I2C2RST` writer - I2C2 reset"]
pub type I2C2RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBDRST` reader - USBD reset"]
pub type USBDRST_R = crate::BitReader;
#[doc = "Field `USBDRST` writer - USBD reset"]
pub type USBDRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN1RST` reader - CAN1 reset"]
pub type CAN1RST_R = crate::BitReader;
#[doc = "Field `CAN1RST` writer - CAN1 reset"]
pub type CAN1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN2RST` reader - CAN2 reset"]
pub type CAN2RST_R = crate::BitReader;
#[doc = "Field `CAN2RST` writer - CAN2 reset"]
pub type CAN2RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BKPRST` reader - Backup interface reset"]
pub type BKPRST_R = crate::BitReader;
#[doc = "Field `BKPRST` writer - Backup interface reset"]
pub type BKPRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWRRST` reader - Power interface reset"]
pub type PWRRST_R = crate::BitReader;
#[doc = "Field `PWRRST` writer - Power interface reset"]
pub type PWRRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DACRST` reader - DAC interface reset"]
pub type DACRST_R = crate::BitReader;
#[doc = "Field `DACRST` writer - DAC interface reset"]
pub type DACRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Timer 2 reset"]
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 3 reset"]
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 4 reset"]
    #[inline(always)]
    pub fn tim4rst(&self) -> TIM4RST_R {
        TIM4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer 5 reset"]
    #[inline(always)]
    pub fn tim5rst(&self) -> TIM5RST_R {
        TIM5RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 6 reset"]
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 7 reset"]
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART 6 reset"]
    #[inline(always)]
    pub fn uart6rst(&self) -> UART6RST_R {
        UART6RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART 7 reset"]
    #[inline(always)]
    pub fn uart7rst(&self) -> UART7RST_R {
        UART7RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART 8 reset"]
    #[inline(always)]
    pub fn uart8rst(&self) -> UART8RST_R {
        UART8RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline(always)]
    pub fn wwdgrst(&self) -> WWDGRST_R {
        WWDGRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 reset"]
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART 3 reset"]
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART 4 reset"]
    #[inline(always)]
    pub fn usart4rst(&self) -> USART4RST_R {
        USART4RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - USART 5 reset"]
    #[inline(always)]
    pub fn usart5rst(&self) -> USART5RST_R {
        USART5RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USBD reset"]
    #[inline(always)]
    pub fn usbdrst(&self) -> USBDRST_R {
        USBDRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1 reset"]
    #[inline(always)]
    pub fn can1rst(&self) -> CAN1RST_R {
        CAN1RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN2 reset"]
    #[inline(always)]
    pub fn can2rst(&self) -> CAN2RST_R {
        CAN2RST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Backup interface reset"]
    #[inline(always)]
    pub fn bkprst(&self) -> BKPRST_R {
        BKPRST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC interface reset"]
    #[inline(always)]
    pub fn dacrst(&self) -> DACRST_R {
        DACRST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim2rst(&mut self) -> TIM2RST_W<APB1PRSTR_SPEC, 0> {
        TIM2RST_W::new(self)
    }
    #[doc = "Bit 1 - Timer 3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim3rst(&mut self) -> TIM3RST_W<APB1PRSTR_SPEC, 1> {
        TIM3RST_W::new(self)
    }
    #[doc = "Bit 2 - Timer 4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim4rst(&mut self) -> TIM4RST_W<APB1PRSTR_SPEC, 2> {
        TIM4RST_W::new(self)
    }
    #[doc = "Bit 3 - Timer 5 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim5rst(&mut self) -> TIM5RST_W<APB1PRSTR_SPEC, 3> {
        TIM5RST_W::new(self)
    }
    #[doc = "Bit 4 - Timer 6 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim6rst(&mut self) -> TIM6RST_W<APB1PRSTR_SPEC, 4> {
        TIM6RST_W::new(self)
    }
    #[doc = "Bit 5 - Timer 7 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim7rst(&mut self) -> TIM7RST_W<APB1PRSTR_SPEC, 5> {
        TIM7RST_W::new(self)
    }
    #[doc = "Bit 6 - UART 6 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart6rst(&mut self) -> UART6RST_W<APB1PRSTR_SPEC, 6> {
        UART6RST_W::new(self)
    }
    #[doc = "Bit 7 - UART 7 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart7rst(&mut self) -> UART7RST_W<APB1PRSTR_SPEC, 7> {
        UART7RST_W::new(self)
    }
    #[doc = "Bit 8 - UART 8 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart8rst(&mut self) -> UART8RST_W<APB1PRSTR_SPEC, 8> {
        UART8RST_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgrst(&mut self) -> WWDGRST_W<APB1PRSTR_SPEC, 11> {
        WWDGRST_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> SPI2RST_W<APB1PRSTR_SPEC, 14> {
        SPI2RST_W::new(self)
    }
    #[doc = "Bit 15 - SPI3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi3rst(&mut self) -> SPI3RST_W<APB1PRSTR_SPEC, 15> {
        SPI3RST_W::new(self)
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart2rst(&mut self) -> USART2RST_W<APB1PRSTR_SPEC, 17> {
        USART2RST_W::new(self)
    }
    #[doc = "Bit 18 - USART 3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart3rst(&mut self) -> USART3RST_W<APB1PRSTR_SPEC, 18> {
        USART3RST_W::new(self)
    }
    #[doc = "Bit 19 - USART 4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart4rst(&mut self) -> USART4RST_W<APB1PRSTR_SPEC, 19> {
        USART4RST_W::new(self)
    }
    #[doc = "Bit 20 - USART 5 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart5rst(&mut self) -> USART5RST_W<APB1PRSTR_SPEC, 20> {
        USART5RST_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<APB1PRSTR_SPEC, 21> {
        I2C1RST_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<APB1PRSTR_SPEC, 22> {
        I2C2RST_W::new(self)
    }
    #[doc = "Bit 23 - USBD reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbdrst(&mut self) -> USBDRST_W<APB1PRSTR_SPEC, 23> {
        USBDRST_W::new(self)
    }
    #[doc = "Bit 25 - CAN1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn can1rst(&mut self) -> CAN1RST_W<APB1PRSTR_SPEC, 25> {
        CAN1RST_W::new(self)
    }
    #[doc = "Bit 26 - CAN2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn can2rst(&mut self) -> CAN2RST_W<APB1PRSTR_SPEC, 26> {
        CAN2RST_W::new(self)
    }
    #[doc = "Bit 27 - Backup interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn bkprst(&mut self) -> BKPRST_W<APB1PRSTR_SPEC, 27> {
        BKPRST_W::new(self)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwrrst(&mut self) -> PWRRST_W<APB1PRSTR_SPEC, 28> {
        PWRRST_W::new(self)
    }
    #[doc = "Bit 29 - DAC interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn dacrst(&mut self) -> DACRST_W<APB1PRSTR_SPEC, 29> {
        DACRST_W::new(self)
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
#[doc = "APB1 peripheral reset register (RCC_APB1PRSTR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1prstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1prstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1PRSTR_SPEC;
impl crate::RegisterSpec for APB1PRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1prstr::R`](R) reader structure"]
impl crate::Readable for APB1PRSTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb1prstr::W`](W) writer structure"]
impl crate::Writable for APB1PRSTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1PRSTR to value 0"]
impl crate::Resettable for APB1PRSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
