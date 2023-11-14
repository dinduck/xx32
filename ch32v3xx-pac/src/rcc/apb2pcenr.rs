#[doc = "Register `APB2PCENR` reader"]
pub type R = crate::R<APB2PCENR_SPEC>;
#[doc = "Register `APB2PCENR` writer"]
pub type W = crate::W<APB2PCENR_SPEC>;
#[doc = "Field `AFIOEN` reader - Alternate function I/O clock enable"]
pub type AFIOEN_R = crate::BitReader;
#[doc = "Field `AFIOEN` writer - Alternate function I/O clock enable"]
pub type AFIOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOPAEN` reader - I/O port A clock enable"]
pub type IOPAEN_R = crate::BitReader;
#[doc = "Field `IOPAEN` writer - I/O port A clock enable"]
pub type IOPAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOPBEN` reader - I/O port B clock enable"]
pub type IOPBEN_R = crate::BitReader;
#[doc = "Field `IOPBEN` writer - I/O port B clock enable"]
pub type IOPBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOPCEN` reader - I/O port C clock enable"]
pub type IOPCEN_R = crate::BitReader;
#[doc = "Field `IOPCEN` writer - I/O port C clock enable"]
pub type IOPCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOPDEN` reader - I/O port D clock enable"]
pub type IOPDEN_R = crate::BitReader;
#[doc = "Field `IOPDEN` writer - I/O port D clock enable"]
pub type IOPDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOPEEN` reader - I/O port E clock enable"]
pub type IOPEEN_R = crate::BitReader;
#[doc = "Field `IOPEEN` writer - I/O port E clock enable"]
pub type IOPEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC1EN` reader - ADC1 interface clock enable"]
pub type ADC1EN_R = crate::BitReader;
#[doc = "Field `ADC1EN` writer - ADC1 interface clock enable"]
pub type ADC1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC2EN` reader - ADC 2 interface clock enable"]
pub type ADC2EN_R = crate::BitReader;
#[doc = "Field `ADC2EN` writer - ADC 2 interface clock enable"]
pub type ADC2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM1EN` reader - TIM1 Timer clock enable"]
pub type TIM1EN_R = crate::BitReader;
#[doc = "Field `TIM1EN` writer - TIM1 Timer clock enable"]
pub type TIM1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI1EN` reader - SPI 1 clock enable"]
pub type SPI1EN_R = crate::BitReader;
#[doc = "Field `SPI1EN` writer - SPI 1 clock enable"]
pub type SPI1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM8EN` reader - TIM8 Timer clock enable"]
pub type TIM8EN_R = crate::BitReader;
#[doc = "Field `TIM8EN` writer - TIM8 Timer clock enable"]
pub type TIM8EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub type USART1EN_R = crate::BitReader;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub type USART1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM9_EN` reader - TIM9 Timer clock enable"]
pub type TIM9_EN_R = crate::BitReader;
#[doc = "Field `TIM9_EN` writer - TIM9 Timer clock enable"]
pub type TIM9_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM10_EN` reader - TIM10 Timer clock enable"]
pub type TIM10_EN_R = crate::BitReader;
#[doc = "Field `TIM10_EN` writer - TIM10 Timer clock enable"]
pub type TIM10_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Alternate function I/O clock enable"]
    #[inline(always)]
    pub fn afioen(&self) -> AFIOEN_R {
        AFIOEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - I/O port A clock enable"]
    #[inline(always)]
    pub fn iopaen(&self) -> IOPAEN_R {
        IOPAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port B clock enable"]
    #[inline(always)]
    pub fn iopben(&self) -> IOPBEN_R {
        IOPBEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I/O port C clock enable"]
    #[inline(always)]
    pub fn iopcen(&self) -> IOPCEN_R {
        IOPCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port D clock enable"]
    #[inline(always)]
    pub fn iopden(&self) -> IOPDEN_R {
        IOPDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I/O port E clock enable"]
    #[inline(always)]
    pub fn iopeen(&self) -> IOPEEN_R {
        IOPEEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC1 interface clock enable"]
    #[inline(always)]
    pub fn adc1en(&self) -> ADC1EN_R {
        ADC1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC 2 interface clock enable"]
    #[inline(always)]
    pub fn adc2en(&self) -> ADC2EN_R {
        ADC2EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 Timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI 1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 Timer clock enable"]
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 19 - TIM9 Timer clock enable"]
    #[inline(always)]
    pub fn tim9_en(&self) -> TIM9_EN_R {
        TIM9_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TIM10 Timer clock enable"]
    #[inline(always)]
    pub fn tim10_en(&self) -> TIM10_EN_R {
        TIM10_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alternate function I/O clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn afioen(&mut self) -> AFIOEN_W<APB2PCENR_SPEC, 0> {
        AFIOEN_W::new(self)
    }
    #[doc = "Bit 2 - I/O port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopaen(&mut self) -> IOPAEN_W<APB2PCENR_SPEC, 2> {
        IOPAEN_W::new(self)
    }
    #[doc = "Bit 3 - I/O port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopben(&mut self) -> IOPBEN_W<APB2PCENR_SPEC, 3> {
        IOPBEN_W::new(self)
    }
    #[doc = "Bit 4 - I/O port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopcen(&mut self) -> IOPCEN_W<APB2PCENR_SPEC, 4> {
        IOPCEN_W::new(self)
    }
    #[doc = "Bit 5 - I/O port D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopden(&mut self) -> IOPDEN_W<APB2PCENR_SPEC, 5> {
        IOPDEN_W::new(self)
    }
    #[doc = "Bit 6 - I/O port E clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iopeen(&mut self) -> IOPEEN_W<APB2PCENR_SPEC, 6> {
        IOPEEN_W::new(self)
    }
    #[doc = "Bit 9 - ADC1 interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc1en(&mut self) -> ADC1EN_W<APB2PCENR_SPEC, 9> {
        ADC1EN_W::new(self)
    }
    #[doc = "Bit 10 - ADC 2 interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc2en(&mut self) -> ADC2EN_W<APB2PCENR_SPEC, 10> {
        ADC2EN_W::new(self)
    }
    #[doc = "Bit 11 - TIM1 Timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<APB2PCENR_SPEC, 11> {
        TIM1EN_W::new(self)
    }
    #[doc = "Bit 12 - SPI 1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<APB2PCENR_SPEC, 12> {
        SPI1EN_W::new(self)
    }
    #[doc = "Bit 13 - TIM8 Timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim8en(&mut self) -> TIM8EN_W<APB2PCENR_SPEC, 13> {
        TIM8EN_W::new(self)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<APB2PCENR_SPEC, 14> {
        USART1EN_W::new(self)
    }
    #[doc = "Bit 19 - TIM9 Timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim9_en(&mut self) -> TIM9_EN_W<APB2PCENR_SPEC, 19> {
        TIM9_EN_W::new(self)
    }
    #[doc = "Bit 20 - TIM10 Timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim10_en(&mut self) -> TIM10_EN_W<APB2PCENR_SPEC, 20> {
        TIM10_EN_W::new(self)
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
#[doc = "APB2 peripheral clock enable register (RCC_APB2PCENR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2pcenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2pcenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2PCENR_SPEC;
impl crate::RegisterSpec for APB2PCENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2pcenr::R`](R) reader structure"]
impl crate::Readable for APB2PCENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2pcenr::W`](W) writer structure"]
impl crate::Writable for APB2PCENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2PCENR to value 0"]
impl crate::Resettable for APB2PCENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
